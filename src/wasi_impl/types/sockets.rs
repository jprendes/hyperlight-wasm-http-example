use crate::wasi_impl::bindings::wasi;

use hyperlight_common::resource::BorrowedResourceGuard;
use wasi::sockets::network::{ErrorCode, IpAddressFamily, IpSocketAddress};
use wasi::sockets::udp::{IncomingDatagram, OutgoingDatagram};

use crate::wasi_impl::resource::{BlockOn, Resource};
use crate::wasi_impl::types::WasiImpl;
use crate::wasi_impl::types::io_poll::AnyPollable;
use crate::wasi_impl::types::io_stream::Stream;
use std::collections::VecDeque;
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

// ---------------------------------------------------------------------------
// TCP socket implementation
// ---------------------------------------------------------------------------

/// State for a WASI TCP socket resource.
///
/// Follows the WASI lifecycle:
///   Created → start_bind/finish_bind → start_connect/finish_connect (client)
///                                    → start_listen/finish_listen  (server)
///                                      → accept (produces new connected sockets)
pub struct TcpSocketState {
    /// Raw pre-listen/connect socket (exists after bind, consumed by connect/listen).
    raw_socket: Option<tokio::net::TcpSocket>,
    /// Present once the socket has been set to listening mode.
    listener: Option<Arc<tokio::net::TcpListener>>,
    /// Background connect task; awaited by `finish_connect`.
    pending_connect: Option<tokio::task::JoinHandle<Result<tokio::net::TcpStream, std::io::Error>>>,
    /// Kept so we can explicitly drop it on shutdown to close the OS socket.
    active_stream: Option<Arc<tokio::net::TcpStream>>,
    /// Connections pre-accepted by `subscribe()` and queued for `accept()` to drain.
    accepted_queue: VecDeque<(tokio::net::TcpStream, std::net::SocketAddr)>,
    address_family: IpAddressFamily,
    is_listening: bool,
    local_addr: Option<std::net::SocketAddr>,
    remote_addr: Option<std::net::SocketAddr>,
}

/// Create a pair of WASI input/output streams backed directly by a TCP socket.
fn tcp_stream_pair(
    tcp_stream: Arc<tokio::net::TcpStream>,
) -> (Resource<Stream>, Resource<Stream>) {
    let input = Resource::new(Stream::tcp_read(tcp_stream.clone()));
    let output = Resource::new(Stream::tcp_write(tcp_stream));
    (input, output)
}

impl
    wasi::sockets::tcp::TcpSocket<
        u64, // Duration
        ErrorCode,
        Resource<Stream>, // InputStream
        IpAddressFamily,
        IpSocketAddress,
        (),               // Network
        Resource<Stream>, // OutputStream
        Resource<AnyPollable>,
    > for WasiImpl
{
    type T = Resource<TcpSocketState>;

    fn start_bind(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _network: BorrowedResourceGuard<()>,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        let std_addr = wasi_addr_to_std(&local_address);
        let socket = match std_addr {
            std::net::SocketAddr::V4(_) => tokio::net::TcpSocket::new_v4(),
            std::net::SocketAddr::V6(_) => tokio::net::TcpSocket::new_v6(),
        }
        .map_err(|ref e| io_error_to_wasi(e))?;

        socket.set_reuseaddr(true).map_err(|ref e| io_error_to_wasi(e))?;
        socket.bind(std_addr).map_err(|ref e| io_error_to_wasi(e))?;

        let local_addr = socket.local_addr().ok();
        let mut state = self_.write().block_on();
        state.local_addr = local_addr;
        state.raw_socket = Some(socket);
        Ok(())
    }

    fn finish_bind(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn start_connect(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _network: BorrowedResourceGuard<()>,
        remote_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        let std_addr = wasi_addr_to_std(&remote_address);
        let mut state = self_.write().block_on();
        // If start_bind was never called, create a fresh socket matching the
        // remote address family so that client-only connect works.
        let socket = match state.raw_socket.take() {
            Some(s) => s,
            None => {
                let s = match std_addr {
                    std::net::SocketAddr::V4(_) => tokio::net::TcpSocket::new_v4(),
                    std::net::SocketAddr::V6(_) => tokio::net::TcpSocket::new_v6(),
                }
                .map_err(|ref e| io_error_to_wasi(e))?;
                s
            }
        };
        state.remote_addr = Some(std_addr);
        // Spawn the connect asynchronously so start_connect returns immediately.
        let handle = async move { socket.connect(std_addr).await }.spawn();
        state.pending_connect = Some(handle);
        Ok(())
    }

    fn finish_connect(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<(Resource<Stream>, Resource<Stream>), ErrorCode> {
        let mut state = self_.write().block_on();
        let handle = state.pending_connect.take().ok_or(ErrorCode::InvalidState)?;
        let stream = async { handle.await }
            .block_on()
            .map_err(|_| ErrorCode::Unknown)? // JoinError
            .map_err(|ref e| io_error_to_wasi(e))?; // io::Error
        let tcp_stream = Arc::new(stream);
        state.local_addr = tcp_stream.local_addr().ok();
        let (input, output) = tcp_stream_pair(tcp_stream.clone());
        state.active_stream = Some(tcp_stream);
        Ok((input, output))
    }

    fn start_listen(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Result<(), ErrorCode> {
        let mut state = self_.write().block_on();
        let socket = state.raw_socket.take().ok_or(ErrorCode::InvalidState)?;
        let listener = socket.listen(128).map_err(|ref e| io_error_to_wasi(e))?;
        state.listener = Some(Arc::new(listener));
        state.is_listening = true;
        Ok(())
    }

    fn finish_listen(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn accept(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<(Self::T, Resource<Stream>, Resource<Stream>), ErrorCode> {
        let mut state = self_.write().block_on();
        let family = state.address_family;

        // Drain from the pre-accepted queue first (populated by subscribe).
        let (tcp_stream, peer_addr) = if let Some(queued) = state.accepted_queue.pop_front() {
            queued
        } else {
            // Non-blocking accept: try once, return WouldBlock if nothing pending.
            let listener = state.listener.as_ref().ok_or(ErrorCode::InvalidState)?;
            let waker = std::task::Waker::noop();
            let mut cx = std::task::Context::from_waker(&waker);
            match listener.poll_accept(&mut cx) {
                std::task::Poll::Ready(Ok(accepted)) => accepted,
                std::task::Poll::Ready(Err(ref e)) => return Err(io_error_to_wasi(e)),
                std::task::Poll::Pending => return Err(ErrorCode::WouldBlock),
            }
        };
        drop(state);

        let tcp_stream = Arc::new(tcp_stream);
        let local_addr = tcp_stream.local_addr().ok();
        let (input, output) = tcp_stream_pair(tcp_stream.clone());

        let new_state = TcpSocketState {
            raw_socket: None,
            listener: None,
            pending_connect: None,
            active_stream: Some(tcp_stream),
            accepted_queue: VecDeque::new(),
            address_family: family,
            is_listening: false,
            local_addr,
            remote_addr: Some(peer_addr),
        };

        Ok((Resource::new(new_state), input, output))
    }

    fn local_address(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<IpSocketAddress, ErrorCode> {
        let state = self_.read().block_on();
        if let Some(ref listener) = state.listener {
            let addr = listener.local_addr().map_err(|ref e| io_error_to_wasi(e))?;
            return Ok(std_addr_to_wasi_addr(addr));
        }
        if let Some(addr) = state.local_addr {
            return Ok(std_addr_to_wasi_addr(addr));
        }
        Err(ErrorCode::InvalidState)
    }

    fn remote_address(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<IpSocketAddress, ErrorCode> {
        let state = self_.read().block_on();
        let addr = state.remote_addr.ok_or(ErrorCode::InvalidState)?;
        Ok(std_addr_to_wasi_addr(addr))
    }

    fn is_listening(&mut self, self_: BorrowedResourceGuard<Self::T>) -> bool {
        self_.read().block_on().is_listening
    }

    fn address_family(&mut self, self_: BorrowedResourceGuard<Self::T>) -> IpAddressFamily {
        self_.read().block_on().address_family
    }

    fn set_listen_backlog_size(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u64,
    ) -> Result<(), ErrorCode> {
        // Backlog is set at listen time; just accept any value here.
        Ok(())
    }

    fn keep_alive_enabled(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<bool, ErrorCode> {
        Ok(false)
    }

    fn set_keep_alive_enabled(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: bool,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn keep_alive_idle_time(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u64, ErrorCode> {
        Ok(7_200_000_000_000) // 2 hours in nanoseconds
    }

    fn set_keep_alive_idle_time(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u64,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn keep_alive_interval(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u64, ErrorCode> {
        Ok(75_000_000_000) // 75 seconds in nanoseconds
    }

    fn set_keep_alive_interval(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u64,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn keep_alive_count(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u32, ErrorCode> {
        Ok(9)
    }

    fn set_keep_alive_count(
        &mut self,
        _self_: BorrowedResourceGuard<Self::T>,
        _value: u32,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn hop_limit(&mut self, _self_: BorrowedResourceGuard<Self::T>) -> Result<u8, ErrorCode> {
        Ok(64)
    }

    fn set_hop_limit(
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

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        let state = self_.read().block_on();
        // If listening, wait until a connection is ready to accept.
        if let Some(ref listener) = state.listener {
            let listener = listener.clone();
            return Resource::new(AnyPollable::future(async move {
                // Wait until the listener is ready to accept a connection.
                let _ = listener.accept().await;
            }));
        }
        // If connecting, wait until the connect task finishes.
        // Otherwise (bind lifecycle), always ready.
        Resource::new(AnyPollable::future(std::future::ready(())))
    }

    fn shutdown(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _shutdown_type: wasi::sockets::tcp::ShutdownType,
    ) -> Result<(), ErrorCode> {
        let mut state = self_.write().block_on();
        // Drop all socket resources so the OS file descriptors are closed.
        state.active_stream.take();
        state.listener.take();
        if let Some(handle) = state.pending_connect.take() {
            handle.abort();
        }
        state.raw_socket.take();
        state.is_listening = false;
        state.remote_addr.take();
        Ok(())
    }
}

impl wasi::sockets::TcpCreateSocket<ErrorCode, IpAddressFamily, Resource<TcpSocketState>>
    for WasiImpl
{
    fn create_tcp_socket(
        &mut self,
        address_family: IpAddressFamily,
    ) -> Result<Resource<TcpSocketState>, ErrorCode> {
        Ok(Resource::new(TcpSocketState {
            raw_socket: None,
            listener: None,
            pending_connect: None,
            active_stream: None,
            accepted_queue: VecDeque::new(),
            address_family,
            is_listening: false,
            local_addr: None,
            remote_addr: None,
        }))
    }
}

impl
    wasi::sockets::Tcp<
        u64,
        ErrorCode,
        Resource<Stream>,
        IpAddressFamily,
        IpSocketAddress,
        (),
        Resource<Stream>,
        Resource<AnyPollable>,
    > for WasiImpl
{
}
