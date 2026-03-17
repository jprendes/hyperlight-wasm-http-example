use crate::wasi_impl::bindings::wasi;

use hyperlight_common::resource::BorrowedResourceGuard;
use wasi::sockets::network::{ErrorCode, IpAddressFamily, IpSocketAddress};
use wasi::sockets::udp::{IncomingDatagram, OutgoingDatagram};

use crate::wasi_impl::resource::{BlockOn, Resource};
use crate::wasi_impl::types::WasiImpl;
use crate::wasi_impl::types::io_poll::AnyPollable;
use std::sync::Arc;

fn io_error_to_wasi(err: &std::io::Error) -> ErrorCode {
    match err.kind() {
        std::io::ErrorKind::PermissionDenied => ErrorCode::AccessDenied,
        std::io::ErrorKind::AddrInUse => ErrorCode::AddressInUse,
        std::io::ErrorKind::AddrNotAvailable => ErrorCode::AddressNotBindable,
        std::io::ErrorKind::ConnectionRefused => ErrorCode::ConnectionRefused,
        std::io::ErrorKind::ConnectionReset => ErrorCode::ConnectionReset,
        std::io::ErrorKind::ConnectionAborted => ErrorCode::ConnectionAborted,
        std::io::ErrorKind::InvalidInput => ErrorCode::InvalidArgument,
        std::io::ErrorKind::TimedOut => ErrorCode::Timeout,
        std::io::ErrorKind::OutOfMemory => ErrorCode::OutOfMemory,
        std::io::ErrorKind::Unsupported => ErrorCode::NotSupported,
        std::io::ErrorKind::WouldBlock => ErrorCode::WouldBlock,
        _ => ErrorCode::Unknown,
    }
}

// --- Structs ---

/// Shared datagram stream state wrapping a tokio UDP socket.
/// Used for both incoming and outgoing datagram streams since
/// UDP is bidirectional on the same underlying OS socket.
pub struct DatagramStream {
    socket: Arc<tokio::net::UdpSocket>,
}

/// State for a WASI UDP socket resource.
/// The socket is temporarily held between `start_bind` and `stream()`;
/// once datagram streams are created, the socket reference is moved into them.
pub struct UdpSocketState {
    socket: Option<Arc<tokio::net::UdpSocket>>,
    address_family: IpAddressFamily,
    local_addr: Option<std::net::SocketAddr>,
}

fn wasi_addr_to_std(addr: &IpSocketAddress) -> std::net::SocketAddr {
    use wasi::sockets::network::*;
    match addr {
        IpSocketAddress::Ipv4(v4) => std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::new(v4.address.0, v4.address.1, v4.address.2, v4.address.3),
            v4.port,
        )),
        IpSocketAddress::Ipv6(v6) => std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
            std::net::Ipv6Addr::new(
                v6.address.0,
                v6.address.1,
                v6.address.2,
                v6.address.3,
                v6.address.4,
                v6.address.5,
                v6.address.6,
                v6.address.7,
            ),
            v6.port,
            v6.flow_info,
            v6.scope_id,
        )),
    }
}

fn std_addr_to_wasi_addr(addr: std::net::SocketAddr) -> IpSocketAddress {
    use wasi::sockets::network::*;
    match addr {
        std::net::SocketAddr::V4(v4) => {
            let octets = v4.ip().octets();
            IpSocketAddress::Ipv4(Ipv4SocketAddress {
                port: v4.port(),
                address: (octets[0], octets[1], octets[2], octets[3]),
            })
        }
        std::net::SocketAddr::V6(v6) => {
            let segments = v6.ip().segments();
            IpSocketAddress::Ipv6(Ipv6SocketAddress {
                port: v6.port(),
                flow_info: v6.flowinfo(),
                address: (
                    segments[0],
                    segments[1],
                    segments[2],
                    segments[3],
                    segments[4],
                    segments[5],
                    segments[6],
                    segments[7],
                ),
                scope_id: v6.scope_id(),
            })
        }
    }
}

impl wasi::sockets::udp::IncomingDatagramStream<ErrorCode, IpSocketAddress, Resource<AnyPollable>>
    for WasiImpl
{
    type T = Resource<DatagramStream>;

    fn receive(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        max_results: u64,
    ) -> Result<Vec<IncomingDatagram<IpSocketAddress>>, ErrorCode> {
        if max_results == 0 {
            return Ok(vec![]);
        }
        let stream = self_.read().block_on();
        let mut buf = vec![0u8; 65507];
        let mut datagrams = Vec::new();
        for _ in 0..max_results {
            match stream.socket.try_recv_from(&mut buf) {
                Ok((size, addr)) => datagrams.push(IncomingDatagram {
                    data: Vec::from(&buf[..size]),
                    remote_address: std_addr_to_wasi_addr(addr),
                }),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(ref e) => {
                    return if datagrams.is_empty() {
                        Err(io_error_to_wasi(e))
                    } else {
                        Ok(datagrams)
                    };
                }
            }
        }
        Ok(datagrams)
    }

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        let socket = self_.read().block_on().socket.clone();
        Resource::new(AnyPollable::future(async move {
            let _ = socket.readable().await;
        }))
    }
}

impl wasi::sockets::udp::OutgoingDatagramStream<ErrorCode, IpSocketAddress, Resource<AnyPollable>>
    for WasiImpl
{
    type T = Resource<DatagramStream>;

    fn check_send(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Result<u64, ErrorCode> {
        Ok(1024)
    }

    fn send(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        datagrams: Vec<OutgoingDatagram<IpSocketAddress>>,
    ) -> Result<u64, ErrorCode> {
        let stream = self_.read().block_on();
        let mut count = 0u64;
        for datagram in datagrams {
            let Some(ref addr) = datagram.remote_address else {
                return if count > 0 {
                    Ok(count)
                } else {
                    Err(ErrorCode::Unknown)
                };
            };
            let std_addr = wasi_addr_to_std(addr);
            match stream.socket.try_send_to(&datagram.data, std_addr) {
                Ok(_) => count += 1,
                Err(ref e) => {
                    return if count > 0 {
                        Ok(count)
                    } else {
                        Err(io_error_to_wasi(e))
                    };
                }
            }
        }
        Ok(count)
    }

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        let socket = self_.read().block_on().socket.clone();
        Resource::new(AnyPollable::future(async move {
            let _ = socket.writable().await;
        }))
    }
}

impl wasi::sockets::network::Network for WasiImpl {
    type T = ();
}

impl wasi::sockets::Network for WasiImpl {}

impl wasi::sockets::InstanceNetwork<()> for WasiImpl {
    fn instance_network(&mut self) -> () {
        ()
    }
}

impl
    wasi::sockets::udp::UdpSocket<
        ErrorCode,
        Resource<DatagramStream>,
        IpAddressFamily,
        IpSocketAddress,
        (),
        Resource<DatagramStream>,
        Resource<AnyPollable>,
    > for WasiImpl
{
    type T = Resource<UdpSocketState>;

    fn start_bind(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _network: BorrowedResourceGuard<()>,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        let std_addr = wasi_addr_to_std(&local_address);
        let tokio_socket = async { tokio::net::UdpSocket::bind(std_addr).await }
            .block_on()
            .map_err(|ref e| io_error_to_wasi(e))?;

        let local_addr = tokio_socket.local_addr().ok();
        let mut state = self_.write().block_on();
        state.socket = Some(Arc::new(tokio_socket));
        state.local_addr = local_addr;
        Ok(())
    }

    fn finish_bind(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn stream(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _remote_address: Option<IpSocketAddress>,
    ) -> Result<(Resource<DatagramStream>, Resource<DatagramStream>), ErrorCode> {
        let mut state = self_.write().block_on();
        let socket = state.socket.take().ok_or(ErrorCode::InvalidState)?;
        Ok((
            Resource::new(DatagramStream {
                socket: socket.clone(),
            }),
            Resource::new(DatagramStream { socket }),
        ))
    }

    fn local_address(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<IpSocketAddress, ErrorCode> {
        let state = self_.read().block_on();
        if let Some(ref socket) = state.socket {
            let addr = socket.local_addr().map_err(|ref e| io_error_to_wasi(e))?;
            return Ok(std_addr_to_wasi_addr(addr));
        }
        let addr = state.local_addr.ok_or(ErrorCode::InvalidState)?;
        Ok(std_addr_to_wasi_addr(addr))
    }

    fn remote_address(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<IpSocketAddress, ErrorCode> {
        // UDP sockets are connectionless; no remote address unless connected
        Err(ErrorCode::InvalidState)
    }

    fn address_family(&mut self, self_: BorrowedResourceGuard<Self::T>) -> IpAddressFamily {
        let state = self_.read().block_on();
        state.address_family
    }

    fn unicast_hop_limit(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u8, ErrorCode> {
        Ok(64)
    }

    fn set_unicast_hop_limit(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u8,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn receive_buffer_size(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u64, ErrorCode> {
        Ok(65536)
    }

    fn set_receive_buffer_size(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u64,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn send_buffer_size(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u64, ErrorCode> {
        Ok(65536)
    }

    fn set_send_buffer_size(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u64,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn subscribe(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        // UDP socket lifecycle events (bind) are synchronous, so always ready
        Resource::new(AnyPollable::future(std::future::ready(())))
    }
}

impl wasi::sockets::UdpCreateSocket<ErrorCode, IpAddressFamily, Resource<UdpSocketState>>
    for WasiImpl
{
    fn create_udp_socket(
        &mut self,
        address_family: IpAddressFamily,
    ) -> Result<Resource<UdpSocketState>, ErrorCode> {
        Ok(Resource::new(UdpSocketState {
            socket: None,
            address_family,
            local_addr: None,
        }))
    }
}

impl wasi::sockets::Udp<ErrorCode, IpAddressFamily, IpSocketAddress, (), Resource<AnyPollable>>
    for WasiImpl
{
}
