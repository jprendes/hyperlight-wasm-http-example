#![allow(
    clippy::all,
    dead_code,
    unused_variables,
    non_snake_case,
    unused_braces,
    unused_mut
)]
pub mod r#root {
    pub mod r#component {
        pub trait RootExports<I: RootImports + ::core::marker::Send> {
            type IncomingHandler: super::super::r#wasi::r#http::IncomingHandler<
                    <I::Types as super::super::r#wasi::r#http::r#types::IncomingRequest<
                        <I::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                        <I::Types as super::super::r#wasi::r#http::r#types::IncomingBody<
                            <I::Types as super::super::r#wasi::r#http::r#types::FutureTrailers<
                                <I::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                                <I::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                            >>::T,
                            <I::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                                <I::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                <I::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                            >>::T,
                        >>::T,
                    >>::T,
                    <I::Types as super::super::r#wasi::r#http::r#types::ResponseOutparam<
                        <I::Types as super::super::r#wasi::r#http::r#types::OutgoingResponse<
                            <I::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                            <I::Types as super::super::r#wasi::r#http::r#types::OutgoingBody<
                                <I::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                                <I::Streams as super::super::r#wasi::r#io::r#streams::OutputStream<
                                    <I::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                    <I::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                                        <I::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                        <I::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                                    >>::T,
                                    <I::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                                >>::T,
                            >>::T,
                        >>::T,
                    >>::T,
                >;
            fn r#incoming_handler(
                &mut self,
            ) -> impl ::core::borrow::BorrowMut<Self::IncomingHandler>;
        }
        pub trait RootImports {
            type Poll: super::super::r#wasi::r#io::Poll;
            fn r#poll(&mut self) -> impl ::core::borrow::BorrowMut<Self::Poll>;
            type MonotonicClock: super::super::r#wasi::r#clocks::MonotonicClock<
                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                >;
            fn r#monotonic_clock(
                &mut self,
            ) -> impl ::core::borrow::BorrowMut<Self::MonotonicClock>;
            type Random: super::super::r#wasi::r#random::Random;
            fn r#random(&mut self) -> impl ::core::borrow::BorrowMut<Self::Random>;
            type Error: super::super::r#wasi::r#io::Error;
            fn r#error(&mut self) -> impl ::core::borrow::BorrowMut<Self::Error>;
            type Streams: super::super::r#wasi::r#io::Streams<
                    <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                >;
            fn r#streams(&mut self) -> impl ::core::borrow::BorrowMut<Self::Streams>;
            type Stdout: super::super::r#wasi::r#cli::Stdout<
                    <Self::Streams as super::super::r#wasi::r#io::r#streams::OutputStream<
                        <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                        <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                            <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                            <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >;
            fn r#stdout(&mut self) -> impl ::core::borrow::BorrowMut<Self::Stdout>;
            type Stderr: super::super::r#wasi::r#cli::Stderr<
                    <Self::Streams as super::super::r#wasi::r#io::r#streams::OutputStream<
                        <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                        <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                            <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                            <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >;
            fn r#stderr(&mut self) -> impl ::core::borrow::BorrowMut<Self::Stderr>;
            type Stdin: super::super::r#wasi::r#cli::Stdin<
                    <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                        <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >;
            fn r#stdin(&mut self) -> impl ::core::borrow::BorrowMut<Self::Stdin>;
            type Types: super::super::r#wasi::r#http::Types<
                    super::super::r#wasi::r#clocks::r#monotonic_clock::Duration,
                    <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                    <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                        <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <Self::Streams as super::super::r#wasi::r#io::r#streams::OutputStream<
                        <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                        <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                            <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                            <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                >;
            fn r#types(&mut self) -> impl ::core::borrow::BorrowMut<Self::Types>;
            type OutgoingHandler: super::super::r#wasi::r#http::OutgoingHandler<
                    super::super::r#wasi::r#http::r#types::ErrorCode,
                    <Self::Types as super::super::r#wasi::r#http::r#types::FutureIncomingResponse<
                        <Self::Types as super::super::r#wasi::r#http::r#types::IncomingResponse<
                            <Self::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                            <Self::Types as super::super::r#wasi::r#http::r#types::IncomingBody<
                                <Self::Types as super::super::r#wasi::r#http::r#types::FutureTrailers<
                                    <Self::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                                >>::T,
                                <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                                    <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                                >>::T,
                            >>::T,
                        >>::T,
                        <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <Self::Types as super::super::r#wasi::r#http::r#types::OutgoingRequest<
                        <Self::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                        <Self::Types as super::super::r#wasi::r#http::r#types::OutgoingBody<
                            <Self::Types as super::super::r#wasi::r#http::r#types::Fields>::T,
                            <Self::Streams as super::super::r#wasi::r#io::r#streams::OutputStream<
                                <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                <Self::Streams as super::super::r#wasi::r#io::r#streams::InputStream<
                                    <Self::Error as super::super::r#wasi::r#io::r#error::Error>::T,
                                    <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                                >>::T,
                                <Self::Poll as super::super::r#wasi::r#io::r#poll::Pollable>::T,
                            >>::T,
                        >>::T,
                    >>::T,
                    <Self::Types as super::super::r#wasi::r#http::r#types::RequestOptions<
                        super::super::r#wasi::r#clocks::r#monotonic_clock::Duration,
                    >>::T,
                >;
            fn r#outgoing_handler(
                &mut self,
            ) -> impl ::core::borrow::BorrowMut<Self::OutgoingHandler>;
        }
        pub trait Root {
            type Exports<I: RootImports + ::core::marker::Send>: RootExports<I>;
            fn instantiate<I: RootImports + ::core::marker::Send + 'static>(
                self,
                imports: I,
            ) -> Self::Exports<I>;
        }
    }
}
pub mod r#wasi {
    pub mod r#cli {
        pub mod r#stderr {
            pub type OutputStream<OutputStream> = OutputStream;
        }
        pub mod r#stdin {
            pub type InputStream<InputStream> = InputStream;
        }
        pub mod r#stdout {
            pub type OutputStream<OutputStream> = OutputStream;
        }
        pub trait Stderr<OutputStream> {
            fn r#get_stderr(&mut self) -> r#stderr::OutputStream<OutputStream>;
        }
        pub trait Stdin<InputStream> {
            fn r#get_stdin(&mut self) -> r#stdin::InputStream<InputStream>;
        }
        pub trait Stdout<OutputStream> {
            fn r#get_stdout(&mut self) -> r#stdout::OutputStream<OutputStream>;
        }
    }
    pub mod r#clocks {
        pub mod r#monotonic_clock {
            pub type Pollable<Pollable> = Pollable;
            pub type Instant = u64;
            pub type Duration = u64;
        }
        pub trait MonotonicClock<Pollable> {
            fn r#now(&mut self) -> r#monotonic_clock::Instant;
            fn r#resolution(&mut self) -> r#monotonic_clock::Duration;
            fn r#subscribe_instant(
                &mut self,
                r#when: r#monotonic_clock::Instant,
            ) -> r#monotonic_clock::Pollable<Pollable>;
            fn r#subscribe_duration(
                &mut self,
                r#when: r#monotonic_clock::Duration,
            ) -> r#monotonic_clock::Pollable<Pollable>;
        }
    }
    pub mod r#http {
        pub mod r#incoming_handler {
            pub type IncomingRequest<IncomingRequest> = IncomingRequest;
            pub type ResponseOutparam<ResponseOutparam> = ResponseOutparam;
        }
        pub mod r#outgoing_handler {
            pub type OutgoingRequest<OutgoingRequest> = OutgoingRequest;
            pub type RequestOptions<RequestOptions> = RequestOptions;
            pub type FutureIncomingResponse<FutureIncomingResponse> = FutureIncomingResponse;
            pub type ErrorCode<ErrorCode> = ErrorCode;
        }
        pub mod r#types {
            pub trait Fields {
                type T: ::core::marker::Send;
                fn new(&mut self) -> Self::T;
                fn r#from_list(
                    &mut self,
                    r#entries: alloc::vec::Vec<(self::FieldName, self::FieldValue)>,
                ) -> ::core::result::Result<Self::T, self::HeaderError>;
                fn r#get(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#name: self::FieldName,
                ) -> alloc::vec::Vec<self::FieldValue>;
                fn r#has(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#name: self::FieldName,
                ) -> bool;
                fn r#set(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#name: self::FieldName,
                    r#value: alloc::vec::Vec<self::FieldValue>,
                ) -> ::core::result::Result<(), self::HeaderError>;
                fn r#delete(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#name: self::FieldName,
                ) -> ::core::result::Result<(), self::HeaderError>;
                fn r#append(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#name: self::FieldName,
                    r#value: self::FieldValue,
                ) -> ::core::result::Result<(), self::HeaderError>;
                fn r#entries(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> alloc::vec::Vec<(self::FieldName, self::FieldValue)>;
                fn r#clone(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> Self::T;
            }
            pub trait FutureIncomingResponse<IncomingResponse, Pollable> {
                type T: ::core::marker::Send;
                fn r#subscribe(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Pollable<Pollable>;
                fn r#get(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::core::result::Result<IncomingResponse, self::ErrorCode>,
                        (),
                    >,
                >;
            }
            pub trait FutureTrailers<Fields, Pollable> {
                type T: ::core::marker::Send;
                fn r#subscribe(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Pollable<Pollable>;
                fn r#get(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::core::result::Result<
                            ::core::option::Option<self::Trailers<Fields>>,
                            self::ErrorCode,
                        >,
                        (),
                    >,
                >;
            }
            pub trait IncomingBody<FutureTrailers, InputStream> {
                type T: ::core::marker::Send;
                fn r#stream(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<self::InputStream<InputStream>, ()>;
                fn r#finish(&mut self, r#this: Self::T) -> FutureTrailers;
            }
            pub trait IncomingRequest<Fields, IncomingBody> {
                type T: ::core::marker::Send;
                fn r#method(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Method;
                fn r#path_with_query(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<alloc::string::String>;
                fn r#scheme(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<self::Scheme>;
                fn r#authority(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<alloc::string::String>;
                fn r#headers(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Headers<Fields>;
                fn r#consume(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<IncomingBody, ()>;
            }
            pub trait IncomingResponse<Fields, IncomingBody> {
                type T: ::core::marker::Send;
                fn r#status(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::StatusCode;
                fn r#headers(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Headers<Fields>;
                fn r#consume(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<IncomingBody, ()>;
            }
            pub trait OutgoingBody<Fields, OutputStream> {
                type T: ::core::marker::Send;
                fn r#write(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<self::OutputStream<OutputStream>, ()>;
                fn r#finish(
                    &mut self,
                    r#this: Self::T,
                    r#trailers: ::core::option::Option<self::Trailers<Fields>>,
                ) -> ::core::result::Result<(), self::ErrorCode>;
            }
            pub trait OutgoingRequest<Fields, OutgoingBody> {
                type T: ::core::marker::Send;
                fn new(&mut self, r#headers: self::Headers<Fields>) -> Self::T;
                fn r#body(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<OutgoingBody, ()>;
                fn r#method(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Method;
                fn r#set_method(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#method: self::Method,
                ) -> ::core::result::Result<(), ()>;
                fn r#path_with_query(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<alloc::string::String>;
                fn r#set_path_with_query(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#path_with_query: ::core::option::Option<alloc::string::String>,
                ) -> ::core::result::Result<(), ()>;
                fn r#scheme(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<self::Scheme>;
                fn r#set_scheme(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#scheme: ::core::option::Option<self::Scheme>,
                ) -> ::core::result::Result<(), ()>;
                fn r#authority(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<alloc::string::String>;
                fn r#set_authority(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#authority: ::core::option::Option<alloc::string::String>,
                ) -> ::core::result::Result<(), ()>;
                fn r#headers(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Headers<Fields>;
            }
            pub trait OutgoingResponse<Fields, OutgoingBody> {
                type T: ::core::marker::Send;
                fn new(&mut self, r#headers: self::Headers<Fields>) -> Self::T;
                fn r#status_code(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::StatusCode;
                fn r#set_status_code(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#status_code: self::StatusCode,
                ) -> ::core::result::Result<(), ()>;
                fn r#headers(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Headers<Fields>;
                fn r#body(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<OutgoingBody, ()>;
            }
            pub trait RequestOptions<Duration> {
                type T: ::core::marker::Send;
                fn new(&mut self) -> Self::T;
                fn r#connect_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<self::Duration<Duration>>;
                fn r#set_connect_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#duration: ::core::option::Option<self::Duration<Duration>>,
                ) -> ::core::result::Result<(), ()>;
                fn r#first_byte_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<self::Duration<Duration>>;
                fn r#set_first_byte_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#duration: ::core::option::Option<self::Duration<Duration>>,
                ) -> ::core::result::Result<(), ()>;
                fn r#between_bytes_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::option::Option<self::Duration<Duration>>;
                fn r#set_between_bytes_timeout(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#duration: ::core::option::Option<self::Duration<Duration>>,
                ) -> ::core::result::Result<(), ()>;
            }
            pub trait ResponseOutparam<OutgoingResponse> {
                type T: ::core::marker::Send;
                fn r#set(
                    &mut self,
                    r#param: Self::T,
                    r#response: ::core::result::Result<OutgoingResponse, self::ErrorCode>,
                ) -> ();
            }
            pub type Duration<Duration> = Duration;
            pub type InputStream<InputStream> = InputStream;
            pub type Pollable<Pollable> = Pollable;
            pub type OutputStream<OutputStream> = OutputStream;
            pub type IoError<Error> = Error;
            pub type FieldKey = alloc::string::String;
            pub type FieldName = self::FieldKey;
            pub type FieldValue = alloc::vec::Vec<u8>;
            #[derive(Debug)]
            pub enum HeaderError {
                InvalidSyntax,
                Forbidden,
                Immutable,
            }
            #[derive(Debug)]
            pub enum Method {
                Get,
                Head,
                Post,
                Put,
                Delete,
                Connect,
                Options,
                Trace,
                Patch,
                Other(alloc::string::String),
            }
            #[derive(Debug)]
            pub enum Scheme {
                HTTP,
                HTTPS,
                Other(alloc::string::String),
            }
            pub type Headers<Fields> = Fields;
            #[derive(Debug)]
            pub struct DNSErrorPayload {
                pub r#rcode: ::core::option::Option<alloc::string::String>,
                pub r#info_code: ::core::option::Option<u16>,
            }
            #[derive(Debug)]
            pub struct TLSAlertReceivedPayload {
                pub r#alert_id: ::core::option::Option<u8>,
                pub r#alert_message: ::core::option::Option<alloc::string::String>,
            }
            #[derive(Debug)]
            pub struct FieldSizePayload {
                pub r#field_name: ::core::option::Option<alloc::string::String>,
                pub r#field_size: ::core::option::Option<u32>,
            }
            #[derive(Debug)]
            pub enum ErrorCode {
                DNSTimeout,
                DNSError(self::DNSErrorPayload),
                DestinationNotFound,
                DestinationUnavailable,
                DestinationIPProhibited,
                DestinationIPUnroutable,
                ConnectionRefused,
                ConnectionTerminated,
                ConnectionTimeout,
                ConnectionReadTimeout,
                ConnectionWriteTimeout,
                ConnectionLimitReached,
                TLSProtocolError,
                TLSCertificateError,
                TLSAlertReceived(self::TLSAlertReceivedPayload),
                HTTPRequestDenied,
                HTTPRequestLengthRequired,
                HTTPRequestBodySize(::core::option::Option<u64>),
                HTTPRequestMethodInvalid,
                HTTPRequestURIInvalid,
                HTTPRequestURITooLong,
                HTTPRequestHeaderSectionSize(::core::option::Option<u32>),
                HTTPRequestHeaderSize(::core::option::Option<self::FieldSizePayload>),
                HTTPRequestTrailerSectionSize(::core::option::Option<u32>),
                HTTPRequestTrailerSize(self::FieldSizePayload),
                HTTPResponseIncomplete,
                HTTPResponseHeaderSectionSize(::core::option::Option<u32>),
                HTTPResponseHeaderSize(self::FieldSizePayload),
                HTTPResponseBodySize(::core::option::Option<u64>),
                HTTPResponseTrailerSectionSize(::core::option::Option<u32>),
                HTTPResponseTrailerSize(self::FieldSizePayload),
                HTTPResponseTransferCoding(
                    ::core::option::Option<alloc::string::String>,
                ),
                HTTPResponseContentCoding(::core::option::Option<alloc::string::String>),
                HTTPResponseTimeout,
                HTTPUpgradeFailed,
                HTTPProtocolError,
                LoopDetected,
                ConfigurationError,
                InternalError(::core::option::Option<alloc::string::String>),
            }
            pub type StatusCode = u16;
            pub type Trailers<Fields> = Fields;
        }
        pub trait IncomingHandler<IncomingRequest, ResponseOutparam> {
            fn r#handle(
                &mut self,
                r#request: r#incoming_handler::IncomingRequest<IncomingRequest>,
                r#response_out: r#incoming_handler::ResponseOutparam<ResponseOutparam>,
            ) -> ();
        }
        pub trait OutgoingHandler<
            ErrorCode,
            FutureIncomingResponse,
            OutgoingRequest,
            RequestOptions,
        > {
            fn r#handle(
                &mut self,
                r#request: r#outgoing_handler::OutgoingRequest<OutgoingRequest>,
                r#options: ::core::option::Option<
                    r#outgoing_handler::RequestOptions<RequestOptions>,
                >,
            ) -> ::core::result::Result<
                r#outgoing_handler::FutureIncomingResponse<FutureIncomingResponse>,
                r#outgoing_handler::ErrorCode<ErrorCode>,
            >;
        }
        pub trait Types<
            Duration,
            Error,
            InputStream,
            OutputStream,
            Pollable,
        >: r#types::Fields + r#types::FutureIncomingResponse<
                <Self as r#types::IncomingResponse<
                    <Self as r#types::Fields>::T,
                    <Self as r#types::IncomingBody<
                        <Self as r#types::FutureTrailers<
                            <Self as r#types::Fields>::T,
                            Pollable,
                        >>::T,
                        InputStream,
                    >>::T,
                >>::T,
                Pollable,
            > + r#types::FutureTrailers<
                <Self as r#types::Fields>::T,
                Pollable,
            > + r#types::IncomingBody<
                <Self as r#types::FutureTrailers<
                    <Self as r#types::Fields>::T,
                    Pollable,
                >>::T,
                InputStream,
            > + r#types::IncomingRequest<
                <Self as r#types::Fields>::T,
                <Self as r#types::IncomingBody<
                    <Self as r#types::FutureTrailers<
                        <Self as r#types::Fields>::T,
                        Pollable,
                    >>::T,
                    InputStream,
                >>::T,
            > + r#types::IncomingResponse<
                <Self as r#types::Fields>::T,
                <Self as r#types::IncomingBody<
                    <Self as r#types::FutureTrailers<
                        <Self as r#types::Fields>::T,
                        Pollable,
                    >>::T,
                    InputStream,
                >>::T,
            > + r#types::OutgoingBody<
                <Self as r#types::Fields>::T,
                OutputStream,
            > + r#types::OutgoingRequest<
                <Self as r#types::Fields>::T,
                <Self as r#types::OutgoingBody<
                    <Self as r#types::Fields>::T,
                    OutputStream,
                >>::T,
            > + r#types::OutgoingResponse<
                <Self as r#types::Fields>::T,
                <Self as r#types::OutgoingBody<
                    <Self as r#types::Fields>::T,
                    OutputStream,
                >>::T,
            > + r#types::RequestOptions<
                Duration,
            > + r#types::ResponseOutparam<
                <Self as r#types::OutgoingResponse<
                    <Self as r#types::Fields>::T,
                    <Self as r#types::OutgoingBody<
                        <Self as r#types::Fields>::T,
                        OutputStream,
                    >>::T,
                >>::T,
            > {
            fn r#http_error_code(
                &mut self,
                r#err: ::hyperlight_common::resource::BorrowedResourceGuard<
                    r#types::IoError<Error>,
                >,
            ) -> ::core::option::Option<r#types::ErrorCode>;
        }
    }
    pub mod r#io {
        pub mod r#error {
            pub trait Error {
                type T: ::core::marker::Send;
                fn r#to_debug_string(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> alloc::string::String;
            }
        }
        pub mod r#poll {
            pub trait Pollable {
                type T: ::core::marker::Send;
                fn r#ready(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> bool;
                fn r#block(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ();
            }
        }
        pub mod r#streams {
            pub trait InputStream<Error, Pollable> {
                type T: ::core::marker::Send;
                fn r#read(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<
                    alloc::vec::Vec<u8>,
                    self::StreamError<Error>,
                >;
                fn r#blocking_read(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<
                    alloc::vec::Vec<u8>,
                    self::StreamError<Error>,
                >;
                fn r#skip(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<u64, self::StreamError<Error>>;
                fn r#blocking_skip(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<u64, self::StreamError<Error>>;
                fn r#subscribe(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Pollable<Pollable>;
            }
            pub trait OutputStream<Error, InputStream, Pollable> {
                type T: ::core::marker::Send;
                fn r#check_write(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<u64, self::StreamError<Error>>;
                fn r#write(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#contents: alloc::vec::Vec<u8>,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#blocking_write_and_flush(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#contents: alloc::vec::Vec<u8>,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#flush(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#blocking_flush(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#subscribe(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                ) -> self::Pollable<Pollable>;
                fn r#write_zeroes(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#blocking_write_zeroes_and_flush(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#len: u64,
                ) -> ::core::result::Result<(), self::StreamError<Error>>;
                fn r#splice(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#src: ::hyperlight_common::resource::BorrowedResourceGuard<
                        InputStream,
                    >,
                    r#len: u64,
                ) -> ::core::result::Result<u64, self::StreamError<Error>>;
                fn r#blocking_splice(
                    &mut self,
                    self_: ::hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
                    r#src: ::hyperlight_common::resource::BorrowedResourceGuard<
                        InputStream,
                    >,
                    r#len: u64,
                ) -> ::core::result::Result<u64, self::StreamError<Error>>;
            }
            pub type Error<Error> = Error;
            pub type Pollable<Pollable> = Pollable;
            #[derive(Debug)]
            pub enum StreamError<Error> {
                LastOperationFailed(self::Error<Error>),
                Closed,
            }
        }
        pub trait Error: r#error::Error {}
        pub trait Poll: r#poll::Pollable {
            fn r#poll(
                &mut self,
                r#in: alloc::vec::Vec<
                    ::hyperlight_common::resource::BorrowedResourceGuard<
                        <Self as r#poll::Pollable>::T,
                    >,
                >,
            ) -> alloc::vec::Vec<u32>;
        }
        pub trait Streams<
            Error,
            Pollable,
        >: r#streams::InputStream<
                Error,
                Pollable,
            > + r#streams::OutputStream<
                Error,
                <Self as r#streams::InputStream<Error, Pollable>>::T,
                Pollable,
            > {}
    }
    pub mod r#random {
        pub trait Random {
            fn r#get_random_bytes(&mut self, r#len: u64) -> alloc::vec::Vec<u8>;
        }
    }
}
pub(crate) struct RootResources<I: r#root::r#component::RootImports> {
    resource0: (),
    resource1: (),
    resource2: (),
    resource3: (),
    resource4: (),
    resource5: (),
    resource6: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::FutureIncomingResponse<
                <I::Types as r#wasi::r#http::r#types::IncomingResponse<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Types as r#wasi::r#http::r#types::IncomingBody<
                        <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                            <I::Types as r#wasi::r#http::r#types::Fields>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <I::Streams as r#wasi::r#io::r#streams::InputStream<
                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                    >>::T,
                >>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
        >,
    >,
    resource7: (),
    resource8: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
        >,
    >,
    resource9: (),
    resource10: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::IncomingResponse<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Types as r#wasi::r#http::r#types::IncomingBody<
                    <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                        <I::Types as r#wasi::r#http::r#types::Fields>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <I::Streams as r#wasi::r#io::r#streams::InputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource11: (),
    resource12: (),
    resource13: (),
    resource14: (),
    resource15: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::OutgoingResponse<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Streams as r#wasi::r#io::r#streams::InputStream<
                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource16: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::ResponseOutparam<
                <I::Types as r#wasi::r#http::r#types::OutgoingResponse<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                        <I::Types as r#wasi::r#http::r#types::Fields>::T,
                        <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                            <I::Streams as r#wasi::r#io::r#streams::InputStream<
                                <I::Error as r#wasi::r#io::r#error::Error>::T,
                                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                            >>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource17: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::RequestOptions<
                r#wasi::r#clocks::r#monotonic_clock::Duration,
            >>::T,
        >,
    >,
    resource18: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                    <I::Error as r#wasi::r#io::r#error::Error>::T,
                    <I::Streams as r#wasi::r#io::r#streams::InputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource19: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::OutgoingRequest<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Streams as r#wasi::r#io::r#streams::InputStream<
                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource20: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::IncomingBody<
                <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                >>::T,
                <I::Streams as r#wasi::r#io::r#streams::InputStream<
                    <I::Error as r#wasi::r#io::r#error::Error>::T,
                    <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource21: (),
    resource22: (),
    resource23: (),
    resource24: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::IncomingRequest<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Types as r#wasi::r#http::r#types::IncomingBody<
                    <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                        <I::Types as r#wasi::r#http::r#types::Fields>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <I::Streams as r#wasi::r#io::r#streams::InputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    >,
    resource25: (),
    resource26: (),
    resource27: (),
    resource28: (),
    resource29: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Types as r#wasi::r#http::r#types::Fields>::T,
        >,
    >,
    resource30: (),
    resource31: (),
    resource32: (),
    resource33: (),
    resource34: (),
    resource35: (),
    resource36: (),
    resource37: (),
    resource38: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                <I::Error as r#wasi::r#io::r#error::Error>::T,
                <I::Streams as r#wasi::r#io::r#streams::InputStream<
                    <I::Error as r#wasi::r#io::r#error::Error>::T,
                    <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                >>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
        >,
    >,
    resource39: (),
    resource40: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Streams as r#wasi::r#io::r#streams::InputStream<
                <I::Error as r#wasi::r#io::r#error::Error>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
        >,
    >,
    resource41: (),
    resource42: (),
    resource43: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Error as r#wasi::r#io::r#error::Error>::T,
        >,
    >,
    resource44: (),
    resource45: (),
    resource46: (),
    resource47: ::std::collections::VecDeque<
        ::hyperlight_common::resource::ResourceEntry<
            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
        >,
    >,
    _phantomI: ::core::marker::PhantomData<I>,
}
impl<I: r#root::r#component::RootImports> RootResources<I> {
    fn new() -> Self {
        RootResources {
            resource0: (),
            resource1: (),
            resource2: (),
            resource3: (),
            resource4: (),
            resource5: (),
            resource6: ::std::collections::VecDeque::new(),
            resource7: (),
            resource8: ::std::collections::VecDeque::new(),
            resource9: (),
            resource10: ::std::collections::VecDeque::new(),
            resource11: (),
            resource12: (),
            resource13: (),
            resource14: (),
            resource15: ::std::collections::VecDeque::new(),
            resource16: ::std::collections::VecDeque::new(),
            resource17: ::std::collections::VecDeque::new(),
            resource18: ::std::collections::VecDeque::new(),
            resource19: ::std::collections::VecDeque::new(),
            resource20: ::std::collections::VecDeque::new(),
            resource21: (),
            resource22: (),
            resource23: (),
            resource24: ::std::collections::VecDeque::new(),
            resource25: (),
            resource26: (),
            resource27: (),
            resource28: (),
            resource29: ::std::collections::VecDeque::new(),
            resource30: (),
            resource31: (),
            resource32: (),
            resource33: (),
            resource34: (),
            resource35: (),
            resource36: (),
            resource37: (),
            resource38: ::std::collections::VecDeque::new(),
            resource39: (),
            resource40: ::std::collections::VecDeque::new(),
            resource41: (),
            resource42: (),
            resource43: ::std::collections::VecDeque::new(),
            resource44: (),
            resource45: (),
            resource46: (),
            resource47: ::std::collections::VecDeque::new(),
            _phantomI: ::core::marker::PhantomData,
        }
    }
}
impl<
    I: r#root::r#component::RootImports,
    S: ::hyperlight_host::sandbox::Callable,
> r#wasi::r#http::IncomingHandler<
    <I::Types as r#wasi::r#http::r#types::IncomingRequest<
        <I::Types as r#wasi::r#http::r#types::Fields>::T,
        <I::Types as r#wasi::r#http::r#types::IncomingBody<
            <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
            <I::Streams as r#wasi::r#io::r#streams::InputStream<
                <I::Error as r#wasi::r#io::r#error::Error>::T,
                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
            >>::T,
        >>::T,
    >>::T,
    <I::Types as r#wasi::r#http::r#types::ResponseOutparam<
        <I::Types as r#wasi::r#http::r#types::OutgoingResponse<
            <I::Types as r#wasi::r#http::r#types::Fields>::T,
            <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                    <I::Error as r#wasi::r#io::r#error::Error>::T,
                    <I::Streams as r#wasi::r#io::r#streams::InputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                >>::T,
            >>::T,
        >>::T,
    >>::T,
> for RootSandbox<I, S> {
    fn r#handle(
        &mut self,
        r#request: r#wasi::r#http::r#incoming_handler::IncomingRequest<
            <I::Types as r#wasi::r#http::r#types::IncomingRequest<
                <I::Types as r#wasi::r#http::r#types::Fields>::T,
                <I::Types as r#wasi::r#http::r#types::IncomingBody<
                    <I::Types as r#wasi::r#http::r#types::FutureTrailers<
                        <I::Types as r#wasi::r#http::r#types::Fields>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                    <I::Streams as r#wasi::r#io::r#streams::InputStream<
                        <I::Error as r#wasi::r#io::r#error::Error>::T,
                        <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
        r#response_out: r#wasi::r#http::r#incoming_handler::ResponseOutparam<
            <I::Types as r#wasi::r#http::r#types::ResponseOutparam<
                <I::Types as r#wasi::r#http::r#types::OutgoingResponse<
                    <I::Types as r#wasi::r#http::r#types::Fields>::T,
                    <I::Types as r#wasi::r#http::r#types::OutgoingBody<
                        <I::Types as r#wasi::r#http::r#types::Fields>::T,
                        <I::Streams as r#wasi::r#io::r#streams::OutputStream<
                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                            <I::Streams as r#wasi::r#io::r#streams::InputStream<
                                <I::Error as r#wasi::r#io::r#error::Error>::T,
                                <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                            >>::T,
                            <I::Poll as r#wasi::r#io::r#poll::Pollable>::T,
                        >>::T,
                    >>::T,
                >>::T,
            >>::T,
        >,
    ) -> () {
        let args = {
            let mut rts = self.rt.lock().unwrap();
            (
                {
                    let i = rts.resource24.len();
                    rts.resource24
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(r#request),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                },
                {
                    let i = rts.resource16.len();
                    rts.resource16
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(
                                r#response_out,
                            ),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                },
            )
        };
        let ret = ::hyperlight_host::sandbox::Callable::call::<
            ::std::vec::Vec<u8>,
        >(&mut self.sb, "r#wasi::r#http::handle", args);
        let ::std::result::Result::Ok(ret) = ret else {
            panic!("bad return from guest {:?}", ret)
        };
        #[allow(clippy::unused_unit)] ()
    }
}
pub struct RootSandbox<
    T: r#root::r#component::RootImports,
    S: ::hyperlight_host::sandbox::Callable,
> {
    pub(crate) sb: S,
    pub(crate) rt: ::std::sync::Arc<::std::sync::Mutex<RootResources<T>>>,
}
pub(crate) fn register_host_functions<
    I: r#root::r#component::RootImports + ::std::marker::Send + 'static,
    S: ::hyperlight_host::func::Registerable,
>(sb: &mut S, i: I) -> ::std::sync::Arc<::std::sync::Mutex<RootResources<I>>> {
    let rts = ::std::sync::Arc::new(::std::sync::Mutex::new(RootResources::new()));
    let imports = ::std::sync::Arc::new(::std::sync::Mutex::new(i));
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]pollable.ready",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#poll(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#poll::Pollable::r#ready(
                    ::std::borrow::BorrowMut::<I::Poll>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource47[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({ alloc::vec![if ret { 1u8 } else { 0u8 }] })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]pollable.block",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#poll(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#poll::Pollable::r#block(
                    ::std::borrow::BorrowMut::<I::Poll>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource47[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok(::alloc::vec::Vec::new())
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::poll",
            move |r#in: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#poll(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::Poll::r#poll(
                    ::std::borrow::BorrowMut::<I::Poll>::borrow_mut(&mut slf),
                    {
                        let n = u32::from_ne_bytes(r#in[0..4].try_into().unwrap())
                            as usize;
                        let mut in_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let in_elem = &r#in[cursor..];
                            let (x, b) = {
                                let i = u32::from_ne_bytes(
                                    in_elem[0..4].try_into().unwrap(),
                                );
                                let Some(v) = rts.resource47[i as usize].borrow() else {
                                    panic!("");
                                };
                                (v, 4)
                            };
                            cursor += b;
                            in_list.push(x);
                        }
                        (in_list, cursor)
                    }
                        .0,
                );
                Ok({
                    let mut ret_list = alloc::vec::Vec::new();
                    let n = ret.len();
                    ret_list.extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                    for ret_elem in ret {
                        ret_list
                            .extend({
                                alloc::vec::Vec::from(u32::to_ne_bytes(ret_elem))
                            })
                    }
                    ret_list
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#clocks::now",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#monotonic_clock(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#clocks::MonotonicClock::r#now(
                    ::std::borrow::BorrowMut::<I::MonotonicClock>::borrow_mut(&mut slf),
                );
                Ok({ alloc::vec::Vec::from(u64::to_ne_bytes(ret)) })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#clocks::resolution",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#monotonic_clock(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#clocks::MonotonicClock::r#resolution(
                    ::std::borrow::BorrowMut::<I::MonotonicClock>::borrow_mut(&mut slf),
                );
                Ok({ alloc::vec::Vec::from(u64::to_ne_bytes(ret)) })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#clocks::subscribe-instant",
            move |r#when: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#monotonic_clock(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#clocks::MonotonicClock::r#subscribe_instant(
                    ::std::borrow::BorrowMut::<I::MonotonicClock>::borrow_mut(&mut slf),
                    {
                        (
                            u64::from_ne_bytes(r#when[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#clocks::subscribe-duration",
            move |r#when: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#monotonic_clock(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#clocks::MonotonicClock::r#subscribe_duration(
                    ::std::borrow::BorrowMut::<I::MonotonicClock>::borrow_mut(&mut slf),
                    {
                        (
                            u64::from_ne_bytes(r#when[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#random::get-random-bytes",
            move |r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#random(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#random::Random::r#get_random_bytes(
                    ::std::borrow::BorrowMut::<I::Random>::borrow_mut(&mut slf),
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    let mut ret_list = alloc::vec::Vec::new();
                    let n = ret.len();
                    ret_list.extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                    for ret_elem in ret {
                        ret_list
                            .extend({ alloc::vec::Vec::from(u8::to_ne_bytes(ret_elem)) })
                    }
                    ret_list
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]error.to-debug-string",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#error(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#error::Error::r#to_debug_string(
                    ::std::borrow::BorrowMut::<I::Error>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource43[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let mut ret_string = alloc::vec::Vec::new();
                    let ret_bytes = ret.into_bytes();
                    ret_string
                        .extend(
                            alloc::vec::Vec::from(
                                u32::to_ne_bytes(ret_bytes.len() as u32),
                            ),
                        );
                    ret_string.extend(ret_bytes);
                    ret_string
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]input-stream.read",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::InputStream::r#read(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let mut ret_body_list = alloc::vec::Vec::new();
                                    let n = ret_body.len();
                                    ret_body_list
                                        .extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                                    for ret_body_elem in ret_body {
                                        ret_body_list
                                            .extend({
                                                alloc::vec::Vec::from(u8::to_ne_bytes(ret_body_elem))
                                            })
                                    }
                                    ret_body_list
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]input-stream.blocking-read",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::InputStream::r#blocking_read(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let mut ret_body_list = alloc::vec::Vec::new();
                                    let n = ret_body.len();
                                    ret_body_list
                                        .extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                                    for ret_body_elem in ret_body {
                                        ret_body_list
                                            .extend({
                                                alloc::vec::Vec::from(u8::to_ne_bytes(ret_body_elem))
                                            })
                                    }
                                    ret_body_list
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]input-stream.skip",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::InputStream::r#skip(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]input-stream.blocking-skip",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::InputStream::r#blocking_skip(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]input-stream.subscribe",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::InputStream::r#subscribe(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.check-write",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#check_write(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.write",
            move |self_: ::std::vec::Vec<u8>, r#contents: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#write(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#contents[0..4].try_into().unwrap())
                            as usize;
                        let mut contents_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let contents_elem = &r#contents[cursor..];
                            let (x, b) = {
                                (
                                    u8::from_ne_bytes(
                                        contents_elem[0..1usize].try_into().unwrap(),
                                    ),
                                    1usize,
                                )
                            };
                            cursor += b;
                            contents_list.push(x);
                        }
                        (contents_list, cursor)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.blocking-write-and-flush",
            move |self_: ::std::vec::Vec<u8>, r#contents: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#blocking_write_and_flush(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#contents[0..4].try_into().unwrap())
                            as usize;
                        let mut contents_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let contents_elem = &r#contents[cursor..];
                            let (x, b) = {
                                (
                                    u8::from_ne_bytes(
                                        contents_elem[0..1usize].try_into().unwrap(),
                                    ),
                                    1usize,
                                )
                            };
                            cursor += b;
                            contents_list.push(x);
                        }
                        (contents_list, cursor)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.flush",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#flush(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.blocking-flush",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#blocking_flush(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.subscribe",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#subscribe(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.write-zeroes",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#write_zeroes(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.blocking-write-zeroes-and-flush",
            move |self_: ::std::vec::Vec<u8>, r#len: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#blocking_write_zeroes_and_flush(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.splice",
            move |
                self_: ::std::vec::Vec<u8>,
                r#src: ::std::vec::Vec<u8>,
                r#len: ::std::vec::Vec<u8>|
            {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#splice(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let i = u32::from_ne_bytes(r#src[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#io::[method]output-stream.blocking-splice",
            move |
                self_: ::std::vec::Vec<u8>,
                r#src: ::std::vec::Vec<u8>,
                r#len: ::std::vec::Vec<u8>|
            {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#streams(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#io::r#streams::OutputStream::r#blocking_splice(
                    ::std::borrow::BorrowMut::<I::Streams>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource38[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let i = u32::from_ne_bytes(r#src[0..4].try_into().unwrap());
                        let Some(v) = rts.resource40[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u64::from_ne_bytes(r#len[0..8usize].try_into().unwrap()),
                            8usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::LastOperationFailed(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                            ret_body_ret
                                                .extend({
                                                    let i = rts.resource43.len();
                                                    rts.resource43
                                                        .push_back(
                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                ret_body_body,
                                                            ),
                                                        );
                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                })
                                        }
                                        r#wasi::r#io::r#streams::StreamError::<
                                            <I::Error as r#wasi::r#io::r#error::Error>::T,
                                        >::Closed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#cli::get-stdout",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#stdout(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#cli::Stdout::r#get_stdout(
                    ::std::borrow::BorrowMut::<I::Stdout>::borrow_mut(&mut slf),
                );
                Ok({
                    let i = rts.resource38.len();
                    rts.resource38
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#cli::get-stderr",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#stderr(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#cli::Stderr::r#get_stderr(
                    ::std::borrow::BorrowMut::<I::Stderr>::borrow_mut(&mut slf),
                );
                Ok({
                    let i = rts.resource38.len();
                    rts.resource38
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#cli::get-stdin",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#stdin(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#cli::Stdin::r#get_stdin(
                    ::std::borrow::BorrowMut::<I::Stdin>::borrow_mut(&mut slf),
                );
                Ok({
                    let i = rts.resource40.len();
                    rts.resource40
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[constructor]fields",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::new(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[static]fields.from-list",
            move |r#entries: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#from_list(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let n = u32::from_ne_bytes(r#entries[0..4].try_into().unwrap())
                            as usize;
                        let mut entries_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let entries_elem = &r#entries[cursor..];
                            let (x, b) = {
                                let entries_elem_elem = &entries_elem[0..];
                                let mut entries_elem_len = 0;
                                let (entries_elem_elem0, b) = {
                                    let n = u32::from_ne_bytes(
                                        entries_elem_elem[0..4].try_into().unwrap(),
                                    ) as usize;
                                    let s = ::alloc::string::ToString::to_string(
                                        ::core::str::from_utf8(&entries_elem_elem[4..4 + n])
                                            .unwrap(),
                                    );
                                    (s, n + 4)
                                };
                                entries_elem_len += b;
                                let entries_elem_elem = &entries_elem_elem[b..];
                                let (entries_elem_elem1, b) = {
                                    let n = u32::from_ne_bytes(
                                        entries_elem_elem[0..4].try_into().unwrap(),
                                    ) as usize;
                                    let mut entries_elem_elem_list = alloc::vec::Vec::new();
                                    let mut cursor = 4;
                                    for i in 0..n {
                                        let entries_elem_elem_elem = &entries_elem_elem[cursor..];
                                        let (x, b) = {
                                            (
                                                u8::from_ne_bytes(
                                                    entries_elem_elem_elem[0..1usize].try_into().unwrap(),
                                                ),
                                                1usize,
                                            )
                                        };
                                        cursor += b;
                                        entries_elem_elem_list.push(x);
                                    }
                                    (entries_elem_elem_list, cursor)
                                };
                                entries_elem_len += b;
                                let entries_elem_elem = &entries_elem_elem[b..];
                                ((entries_elem_elem0, entries_elem_elem1), entries_elem_len)
                            };
                            cursor += b;
                            entries_list.push(x);
                        }
                        (entries_list, cursor)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource29.len();
                                    rts.resource29
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::HeaderError::InvalidSyntax => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Forbidden => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Immutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.get",
            move |self_: ::std::vec::Vec<u8>, r#name: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#get(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#name[0..4].try_into().unwrap())
                            as usize;
                        let s = ::alloc::string::ToString::to_string(
                            ::core::str::from_utf8(&r#name[4..4 + n]).unwrap(),
                        );
                        (s, n + 4)
                    }
                        .0,
                );
                Ok({
                    let mut ret_list = alloc::vec::Vec::new();
                    let n = ret.len();
                    ret_list.extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                    for ret_elem in ret {
                        ret_list
                            .extend({
                                let mut ret_elem_list = alloc::vec::Vec::new();
                                let n = ret_elem.len();
                                ret_elem_list
                                    .extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                                for ret_elem_elem in ret_elem {
                                    ret_elem_list
                                        .extend({
                                            alloc::vec::Vec::from(u8::to_ne_bytes(ret_elem_elem))
                                        })
                                }
                                ret_elem_list
                            })
                    }
                    ret_list
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.has",
            move |self_: ::std::vec::Vec<u8>, r#name: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#has(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#name[0..4].try_into().unwrap())
                            as usize;
                        let s = ::alloc::string::ToString::to_string(
                            ::core::str::from_utf8(&r#name[4..4 + n]).unwrap(),
                        );
                        (s, n + 4)
                    }
                        .0,
                );
                Ok({ alloc::vec![if ret { 1u8 } else { 0u8 }] })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.set",
            move |
                self_: ::std::vec::Vec<u8>,
                r#name: ::std::vec::Vec<u8>,
                r#value: ::std::vec::Vec<u8>|
            {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#set(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#name[0..4].try_into().unwrap())
                            as usize;
                        let s = ::alloc::string::ToString::to_string(
                            ::core::str::from_utf8(&r#name[4..4 + n]).unwrap(),
                        );
                        (s, n + 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#value[0..4].try_into().unwrap())
                            as usize;
                        let mut value_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let value_elem = &r#value[cursor..];
                            let (x, b) = {
                                let n = u32::from_ne_bytes(
                                    value_elem[0..4].try_into().unwrap(),
                                ) as usize;
                                let mut value_elem_list = alloc::vec::Vec::new();
                                let mut cursor = 4;
                                for i in 0..n {
                                    let value_elem_elem = &value_elem[cursor..];
                                    let (x, b) = {
                                        (
                                            u8::from_ne_bytes(
                                                value_elem_elem[0..1usize].try_into().unwrap(),
                                            ),
                                            1usize,
                                        )
                                    };
                                    cursor += b;
                                    value_elem_list.push(x);
                                }
                                (value_elem_list, cursor)
                            };
                            cursor += b;
                            value_list.push(x);
                        }
                        (value_list, cursor)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::HeaderError::InvalidSyntax => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Forbidden => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Immutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.delete",
            move |self_: ::std::vec::Vec<u8>, r#name: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#delete(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#name[0..4].try_into().unwrap())
                            as usize;
                        let s = ::alloc::string::ToString::to_string(
                            ::core::str::from_utf8(&r#name[4..4 + n]).unwrap(),
                        );
                        (s, n + 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::HeaderError::InvalidSyntax => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Forbidden => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Immutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.append",
            move |
                self_: ::std::vec::Vec<u8>,
                r#name: ::std::vec::Vec<u8>,
                r#value: ::std::vec::Vec<u8>|
            {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#append(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#name[0..4].try_into().unwrap())
                            as usize;
                        let s = ::alloc::string::ToString::to_string(
                            ::core::str::from_utf8(&r#name[4..4 + n]).unwrap(),
                        );
                        (s, n + 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#value[0..4].try_into().unwrap())
                            as usize;
                        let mut value_list = alloc::vec::Vec::new();
                        let mut cursor = 4;
                        for i in 0..n {
                            let value_elem = &r#value[cursor..];
                            let (x, b) = {
                                (
                                    u8::from_ne_bytes(
                                        value_elem[0..1usize].try_into().unwrap(),
                                    ),
                                    1usize,
                                )
                            };
                            cursor += b;
                            value_list.push(x);
                        }
                        (value_list, cursor)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::HeaderError::InvalidSyntax => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Forbidden => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::HeaderError::Immutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.entries",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#entries(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let mut ret_list = alloc::vec::Vec::new();
                    let n = ret.len();
                    ret_list.extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                    for ret_elem in ret {
                        ret_list
                            .extend({
                                let mut ret_elem_tuple = alloc::vec::Vec::new();
                                let ret_elem_elem = ret_elem.0;
                                ret_elem_tuple
                                    .extend({
                                        let mut ret_elem_elem_string = alloc::vec::Vec::new();
                                        let ret_elem_elem_bytes = ret_elem_elem.into_bytes();
                                        ret_elem_elem_string
                                            .extend(
                                                alloc::vec::Vec::from(
                                                    u32::to_ne_bytes(ret_elem_elem_bytes.len() as u32),
                                                ),
                                            );
                                        ret_elem_elem_string.extend(ret_elem_elem_bytes);
                                        ret_elem_elem_string
                                    });
                                let ret_elem_elem = ret_elem.1;
                                ret_elem_tuple
                                    .extend({
                                        let mut ret_elem_elem_list = alloc::vec::Vec::new();
                                        let n = ret_elem_elem.len();
                                        ret_elem_elem_list
                                            .extend(alloc::vec::Vec::from(u32::to_ne_bytes(n as u32)));
                                        for ret_elem_elem_elem in ret_elem_elem {
                                            ret_elem_elem_list
                                                .extend({
                                                    alloc::vec::Vec::from(u8::to_ne_bytes(ret_elem_elem_elem))
                                                })
                                        }
                                        ret_elem_elem_list
                                    });
                                ret_elem_tuple
                            })
                    }
                    ret_list
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]fields.clone",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::Fields::r#clone(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.method",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#method(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let mut ret_ret = alloc::vec::Vec::new();
                    match ret {
                        r#wasi::r#http::r#types::Method::Get => {
                            ret_ret.extend(u32::to_ne_bytes(0u32));
                        }
                        r#wasi::r#http::r#types::Method::Head => {
                            ret_ret.extend(u32::to_ne_bytes(1u32));
                        }
                        r#wasi::r#http::r#types::Method::Post => {
                            ret_ret.extend(u32::to_ne_bytes(2u32));
                        }
                        r#wasi::r#http::r#types::Method::Put => {
                            ret_ret.extend(u32::to_ne_bytes(3u32));
                        }
                        r#wasi::r#http::r#types::Method::Delete => {
                            ret_ret.extend(u32::to_ne_bytes(4u32));
                        }
                        r#wasi::r#http::r#types::Method::Connect => {
                            ret_ret.extend(u32::to_ne_bytes(5u32));
                        }
                        r#wasi::r#http::r#types::Method::Options => {
                            ret_ret.extend(u32::to_ne_bytes(6u32));
                        }
                        r#wasi::r#http::r#types::Method::Trace => {
                            ret_ret.extend(u32::to_ne_bytes(7u32));
                        }
                        r#wasi::r#http::r#types::Method::Patch => {
                            ret_ret.extend(u32::to_ne_bytes(8u32));
                        }
                        r#wasi::r#http::r#types::Method::Other(ret_body) => {
                            ret_ret.extend(u32::to_ne_bytes(9u32));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                })
                        }
                    }
                    ret_ret
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.path-with-query",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#path_with_query(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.scheme",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#scheme(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::Scheme::HTTP => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::Scheme::HTTPS => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::Scheme::Other(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_string = alloc::vec::Vec::new();
                                                    let ret_body_body_bytes = ret_body_body.into_bytes();
                                                    ret_body_body_string
                                                        .extend(
                                                            alloc::vec::Vec::from(
                                                                u32::to_ne_bytes(ret_body_body_bytes.len() as u32),
                                                            ),
                                                        );
                                                    ret_body_body_string.extend(ret_body_body_bytes);
                                                    ret_body_body_string
                                                })
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.authority",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#authority(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.headers",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#headers(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-request.consume",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingRequest::r#consume(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource24[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource20.len();
                                    rts.resource20
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-body.stream",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingBody::r#stream(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource20[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource40.len();
                                    rts.resource40
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[static]incoming-body.finish",
            move |r#this: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingBody::r#finish(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#this[0..4].try_into().unwrap());
                        let Some(v) = rts.resource20[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource8.len();
                    rts.resource8
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[constructor]outgoing-request",
            move |r#headers: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::new(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#headers[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource19.len();
                    rts.resource19
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.body",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#body(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource18.len();
                                    rts.resource18
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.method",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#method(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let mut ret_ret = alloc::vec::Vec::new();
                    match ret {
                        r#wasi::r#http::r#types::Method::Get => {
                            ret_ret.extend(u32::to_ne_bytes(0u32));
                        }
                        r#wasi::r#http::r#types::Method::Head => {
                            ret_ret.extend(u32::to_ne_bytes(1u32));
                        }
                        r#wasi::r#http::r#types::Method::Post => {
                            ret_ret.extend(u32::to_ne_bytes(2u32));
                        }
                        r#wasi::r#http::r#types::Method::Put => {
                            ret_ret.extend(u32::to_ne_bytes(3u32));
                        }
                        r#wasi::r#http::r#types::Method::Delete => {
                            ret_ret.extend(u32::to_ne_bytes(4u32));
                        }
                        r#wasi::r#http::r#types::Method::Connect => {
                            ret_ret.extend(u32::to_ne_bytes(5u32));
                        }
                        r#wasi::r#http::r#types::Method::Options => {
                            ret_ret.extend(u32::to_ne_bytes(6u32));
                        }
                        r#wasi::r#http::r#types::Method::Trace => {
                            ret_ret.extend(u32::to_ne_bytes(7u32));
                        }
                        r#wasi::r#http::r#types::Method::Patch => {
                            ret_ret.extend(u32::to_ne_bytes(8u32));
                        }
                        r#wasi::r#http::r#types::Method::Other(ret_body) => {
                            ret_ret.extend(u32::to_ne_bytes(9u32));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                })
                        }
                    }
                    ret_ret
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.set-method",
            move |self_: ::std::vec::Vec<u8>, r#method: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#set_method(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u32::from_ne_bytes(r#method[0..4].try_into().unwrap());
                        let method_body = &r#method[4..];
                        match n {
                            0u32 => (r#wasi::r#http::r#types::Method::Get, 4),
                            1u32 => (r#wasi::r#http::r#types::Method::Head, 4),
                            2u32 => (r#wasi::r#http::r#types::Method::Post, 4),
                            3u32 => (r#wasi::r#http::r#types::Method::Put, 4),
                            4u32 => (r#wasi::r#http::r#types::Method::Delete, 4),
                            5u32 => (r#wasi::r#http::r#types::Method::Connect, 4),
                            6u32 => (r#wasi::r#http::r#types::Method::Options, 4),
                            7u32 => (r#wasi::r#http::r#types::Method::Trace, 4),
                            8u32 => (r#wasi::r#http::r#types::Method::Patch, 4),
                            9u32 => {
                                let (method_case_Other, b) = {
                                    let n = u32::from_ne_bytes(
                                        method_body[0..4].try_into().unwrap(),
                                    ) as usize;
                                    let s = ::alloc::string::ToString::to_string(
                                        ::core::str::from_utf8(&method_body[4..4 + n]).unwrap(),
                                    );
                                    (s, n + 4)
                                };
                                (
                                    r#wasi::r#http::r#types::Method::Other(method_case_Other),
                                    b + 4,
                                )
                            }
                            _ => panic!("invalid value for variant"),
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.path-with-query",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#path_with_query(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.set-path-with-query",
            move |self_: ::std::vec::Vec<u8>, r#path_with_query: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#set_path_with_query(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(
                            r#path_with_query[0..1].try_into().unwrap(),
                        );
                        if n != 0 {
                            let path_with_query_body = &r#path_with_query[1..];
                            let (x, b) = {
                                let n = u32::from_ne_bytes(
                                    path_with_query_body[0..4].try_into().unwrap(),
                                ) as usize;
                                let s = ::alloc::string::ToString::to_string(
                                    ::core::str::from_utf8(&path_with_query_body[4..4 + n])
                                        .unwrap(),
                                );
                                (s, n + 4)
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.scheme",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#scheme(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::Scheme::HTTP => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::Scheme::HTTPS => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                        }
                                        r#wasi::r#http::r#types::Scheme::Other(ret_body_body) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_string = alloc::vec::Vec::new();
                                                    let ret_body_body_bytes = ret_body_body.into_bytes();
                                                    ret_body_body_string
                                                        .extend(
                                                            alloc::vec::Vec::from(
                                                                u32::to_ne_bytes(ret_body_body_bytes.len() as u32),
                                                            ),
                                                        );
                                                    ret_body_body_string.extend(ret_body_body_bytes);
                                                    ret_body_body_string
                                                })
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.set-scheme",
            move |self_: ::std::vec::Vec<u8>, r#scheme: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#set_scheme(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#scheme[0..1].try_into().unwrap());
                        if n != 0 {
                            let scheme_body = &r#scheme[1..];
                            let (x, b) = {
                                let n = u32::from_ne_bytes(
                                    scheme_body[0..4].try_into().unwrap(),
                                );
                                let scheme_body_body = &scheme_body[4..];
                                match n {
                                    0u32 => (r#wasi::r#http::r#types::Scheme::HTTP, 4),
                                    1u32 => (r#wasi::r#http::r#types::Scheme::HTTPS, 4),
                                    2u32 => {
                                        let (scheme_body_case_Other, b) = {
                                            let n = u32::from_ne_bytes(
                                                scheme_body_body[0..4].try_into().unwrap(),
                                            ) as usize;
                                            let s = ::alloc::string::ToString::to_string(
                                                ::core::str::from_utf8(&scheme_body_body[4..4 + n]).unwrap(),
                                            );
                                            (s, n + 4)
                                        };
                                        (
                                            r#wasi::r#http::r#types::Scheme::Other(
                                                scheme_body_case_Other,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    _ => panic!("invalid value for variant"),
                                }
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.authority",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#authority(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_string = alloc::vec::Vec::new();
                                    let ret_body_bytes = ret_body.into_bytes();
                                    ret_body_string
                                        .extend(
                                            alloc::vec::Vec::from(
                                                u32::to_ne_bytes(ret_body_bytes.len() as u32),
                                            ),
                                        );
                                    ret_body_string.extend(ret_body_bytes);
                                    ret_body_string
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.set-authority",
            move |self_: ::std::vec::Vec<u8>, r#authority: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#set_authority(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#authority[0..1].try_into().unwrap());
                        if n != 0 {
                            let authority_body = &r#authority[1..];
                            let (x, b) = {
                                let n = u32::from_ne_bytes(
                                    authority_body[0..4].try_into().unwrap(),
                                ) as usize;
                                let s = ::alloc::string::ToString::to_string(
                                    ::core::str::from_utf8(&authority_body[4..4 + n]).unwrap(),
                                );
                                (s, n + 4)
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-request.headers",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingRequest::r#headers(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-body.write",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingBody::r#write(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource18[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource38.len();
                                    rts.resource38
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[static]outgoing-body.finish",
            move |r#this: ::std::vec::Vec<u8>, r#trailers: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingBody::r#finish(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#this[0..4].try_into().unwrap());
                        let Some(v) = rts.resource18[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#trailers[0..1].try_into().unwrap());
                        if n != 0 {
                            let trailers_body = &r#trailers[1..];
                            let (x, b) = {
                                let i = u32::from_ne_bytes(
                                    trailers_body[0..4].try_into().unwrap(),
                                );
                                let Some(v) = rts.resource29[i as usize].take() else {
                                    panic!("");
                                };
                                (v, 4)
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::ErrorCode::DNSTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DNSError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_rcode = ret_body_body.r#rcode;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_rcode {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_rcode_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_rcode_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_rcode_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_rcode_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_rcode_body_bytes = ret_body_body_field_rcode_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_rcode_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(ret_body_body_field_rcode_body_bytes);
                                                                            ret_body_body_field_rcode_body_string
                                                                        });
                                                                    ret_body_body_field_rcode_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_info_code = ret_body_body
                                                        .r#info_code;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_info_code {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_info_code_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_info_code_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_info_code_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u16::to_ne_bytes(ret_body_body_field_info_code_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_info_code_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationNotFound => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(3u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited => {
                                            ret_body_ret.extend(u32::to_ne_bytes(4u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(5u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionRefused => {
                                            ret_body_ret.extend(u32::to_ne_bytes(6u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated => {
                                            ret_body_ret.extend(u32::to_ne_bytes(7u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(8u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(9u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(10u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached => {
                                            ret_body_ret.extend(u32::to_ne_bytes(11u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(12u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSCertificateError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(13u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(14u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_alert_id = ret_body_body.r#alert_id;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_id {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_id_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_id_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_id_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u8::to_ne_bytes(ret_body_body_field_alert_id_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_alert_id_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_alert_message = ret_body_body
                                                        .r#alert_message;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_message {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_message_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_message_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_message_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_alert_message_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_alert_message_body_bytes = ret_body_body_field_alert_message_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_alert_message_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(ret_body_body_field_alert_message_body_bytes);
                                                                            ret_body_body_field_alert_message_body_string
                                                                        });
                                                                    ret_body_body_field_alert_message_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied => {
                                            ret_body_ret.extend(u32::to_ne_bytes(15u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired => {
                                            ret_body_ret.extend(u32::to_ne_bytes(16u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(17u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(18u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(19u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong => {
                                            ret_body_ret.extend(u32::to_ne_bytes(20u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(21u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(22u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_record = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_field_field_name = ret_body_body_body
                                                                        .r#field_name;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_name {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_name_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                        .extend({
                                                                                            let mut ret_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                            let ret_body_body_body_field_field_name_body_bytes = ret_body_body_body_field_field_name_body
                                                                                                .into_bytes();
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(
                                                                                                            ret_body_body_body_field_field_name_body_bytes.len() as u32,
                                                                                                        ),
                                                                                                    ),
                                                                                                );
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(ret_body_body_body_field_field_name_body_bytes);
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                        });
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    let ret_body_body_body_field_field_size = ret_body_body_body
                                                                        .r#field_size;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_size {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_size_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                        .extend({
                                                                                            alloc::vec::Vec::from(
                                                                                                u32::to_ne_bytes(ret_body_body_body_field_field_size_body),
                                                                                            )
                                                                                        });
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    ret_body_body_body_record
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(23u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(24u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete => {
                                            ret_body_ret.extend(u32::to_ne_bytes(25u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(26u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(27u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(28u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(29u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(30u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(31u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(32u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(33u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(34u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(35u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::LoopDetected => {
                                            ret_body_ret.extend(u32::to_ne_bytes(36u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConfigurationError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(37u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::InternalError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(38u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[constructor]request-options",
            move || {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::new(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                );
                Ok({
                    let i = rts.resource17.len();
                    rts.resource17
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.connect-timeout",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#connect_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.set-connect-timeout",
            move |self_: ::std::vec::Vec<u8>, r#duration: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#set_connect_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#duration[0..1].try_into().unwrap());
                        if n != 0 {
                            let duration_body = &r#duration[1..];
                            let (x, b) = {
                                (
                                    u64::from_ne_bytes(
                                        duration_body[0..8usize].try_into().unwrap(),
                                    ),
                                    8usize,
                                )
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.first-byte-timeout",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#first_byte_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.set-first-byte-timeout",
            move |self_: ::std::vec::Vec<u8>, r#duration: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#set_first_byte_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#duration[0..1].try_into().unwrap());
                        if n != 0 {
                            let duration_body = &r#duration[1..];
                            let (x, b) = {
                                (
                                    u64::from_ne_bytes(
                                        duration_body[0..8usize].try_into().unwrap(),
                                    ),
                                    8usize,
                                )
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.between-bytes-timeout",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#between_bytes_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body))
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]request-options.set-between-bytes-timeout",
            move |self_: ::std::vec::Vec<u8>, r#duration: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::RequestOptions::r#set_between_bytes_timeout(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource17[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#duration[0..1].try_into().unwrap());
                        if n != 0 {
                            let duration_body = &r#duration[1..];
                            let (x, b) = {
                                (
                                    u64::from_ne_bytes(
                                        duration_body[0..8usize].try_into().unwrap(),
                                    ),
                                    8usize,
                                )
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[static]response-outparam.set",
            move |r#param: ::std::vec::Vec<u8>, r#response: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::ResponseOutparam::r#set(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#param[0..4].try_into().unwrap());
                        let Some(v) = rts.resource16[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let i = u8::from_ne_bytes(r#response[0..1].try_into().unwrap());
                        let response_body = &r#response[1..];
                        if i == 0 {
                            let (x, b) = {
                                let i = u32::from_ne_bytes(
                                    response_body[0..4].try_into().unwrap(),
                                );
                                let Some(v) = rts.resource15[i as usize].take() else {
                                    panic!("");
                                };
                                (v, 4)
                            };
                            (::core::result::Result::Ok(x), b + 1)
                        } else {
                            let (x, b) = {
                                let n = u32::from_ne_bytes(
                                    response_body[0..4].try_into().unwrap(),
                                );
                                let response_body_body = &response_body[4..];
                                match n {
                                    0u32 => (r#wasi::r#http::r#types::ErrorCode::DNSTimeout, 4),
                                    1u32 => {
                                        let (response_body_case_DNSError, b) = {
                                            let mut response_body_body_cursor = 0;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_rcode, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        let n = u32::from_ne_bytes(
                                                            response_body_body_field_body[0..4].try_into().unwrap(),
                                                        ) as usize;
                                                        let s = ::alloc::string::ToString::to_string(
                                                            ::core::str::from_utf8(
                                                                    &response_body_body_field_body[4..4 + n],
                                                                )
                                                                .unwrap(),
                                                        );
                                                        (s, n + 4)
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_info_code, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        (
                                                            u16::from_ne_bytes(
                                                                response_body_body_field_body[0..2usize].try_into().unwrap(),
                                                            ),
                                                            2usize,
                                                        )
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            (
                                                r#wasi::r#http::r#types::DNSErrorPayload {
                                                    r#rcode: response_body_body_field_rcode,
                                                    r#info_code: response_body_body_field_info_code,
                                                },
                                                response_body_body_cursor,
                                            )
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::DNSError(
                                                response_body_case_DNSError,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    2u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::DestinationNotFound, 4)
                                    }
                                    3u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable,
                                            4,
                                        )
                                    }
                                    4u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited,
                                            4,
                                        )
                                    }
                                    5u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable,
                                            4,
                                        )
                                    }
                                    6u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::ConnectionRefused, 4)
                                    }
                                    7u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated,
                                            4,
                                        )
                                    }
                                    8u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout, 4)
                                    }
                                    9u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout,
                                            4,
                                        )
                                    }
                                    10u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout,
                                            4,
                                        )
                                    }
                                    11u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached,
                                            4,
                                        )
                                    }
                                    12u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::TLSProtocolError, 4)
                                    }
                                    13u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::TLSCertificateError, 4)
                                    }
                                    14u32 => {
                                        let (response_body_case_TLSAlertReceived, b) = {
                                            let mut response_body_body_cursor = 0;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_alert_id, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        (
                                                            u8::from_ne_bytes(
                                                                response_body_body_field_body[0..1usize].try_into().unwrap(),
                                                            ),
                                                            1usize,
                                                        )
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_alert_message, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        let n = u32::from_ne_bytes(
                                                            response_body_body_field_body[0..4].try_into().unwrap(),
                                                        ) as usize;
                                                        let s = ::alloc::string::ToString::to_string(
                                                            ::core::str::from_utf8(
                                                                    &response_body_body_field_body[4..4 + n],
                                                                )
                                                                .unwrap(),
                                                        );
                                                        (s, n + 4)
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            (
                                                r#wasi::r#http::r#types::TLSAlertReceivedPayload {
                                                    r#alert_id: response_body_body_field_alert_id,
                                                    r#alert_message: response_body_body_field_alert_message,
                                                },
                                                response_body_body_cursor,
                                            )
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                                response_body_case_TLSAlertReceived,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    15u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied, 4)
                                    }
                                    16u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired,
                                            4,
                                        )
                                    }
                                    17u32 => {
                                        let (response_body_case_HTTPRequestBodySize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u64::from_ne_bytes(
                                                            response_body_body_body[0..8usize].try_into().unwrap(),
                                                        ),
                                                        8usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                                response_body_case_HTTPRequestBodySize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    18u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid,
                                            4,
                                        )
                                    }
                                    19u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid,
                                            4,
                                        )
                                    }
                                    20u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong,
                                            4,
                                        )
                                    }
                                    21u32 => {
                                        let (response_body_case_HTTPRequestHeaderSectionSize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u32::from_ne_bytes(
                                                            response_body_body_body[0..4usize].try_into().unwrap(),
                                                        ),
                                                        4usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                                response_body_case_HTTPRequestHeaderSectionSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    22u32 => {
                                        let (response_body_case_HTTPRequestHeaderSize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    let mut response_body_body_body_cursor = 0;
                                                    let response_body_body_body_field = &response_body_body_body[response_body_body_body_cursor..];
                                                    let (response_body_body_body_field_field_name, b) = {
                                                        let n = u8::from_ne_bytes(
                                                            response_body_body_body_field[0..1].try_into().unwrap(),
                                                        );
                                                        if n != 0 {
                                                            let response_body_body_body_field_body = &response_body_body_body_field[1..];
                                                            let (x, b) = {
                                                                let n = u32::from_ne_bytes(
                                                                    response_body_body_body_field_body[0..4].try_into().unwrap(),
                                                                ) as usize;
                                                                let s = ::alloc::string::ToString::to_string(
                                                                    ::core::str::from_utf8(
                                                                            &response_body_body_body_field_body[4..4 + n],
                                                                        )
                                                                        .unwrap(),
                                                                );
                                                                (s, n + 4)
                                                            };
                                                            (::core::option::Option::Some(x), b + 1)
                                                        } else {
                                                            (::core::option::Option::None, 1)
                                                        }
                                                    };
                                                    response_body_body_body_cursor += b;
                                                    let response_body_body_body_field = &response_body_body_body[response_body_body_body_cursor..];
                                                    let (response_body_body_body_field_field_size, b) = {
                                                        let n = u8::from_ne_bytes(
                                                            response_body_body_body_field[0..1].try_into().unwrap(),
                                                        );
                                                        if n != 0 {
                                                            let response_body_body_body_field_body = &response_body_body_body_field[1..];
                                                            let (x, b) = {
                                                                (
                                                                    u32::from_ne_bytes(
                                                                        response_body_body_body_field_body[0..4usize]
                                                                            .try_into()
                                                                            .unwrap(),
                                                                    ),
                                                                    4usize,
                                                                )
                                                            };
                                                            (::core::option::Option::Some(x), b + 1)
                                                        } else {
                                                            (::core::option::Option::None, 1)
                                                        }
                                                    };
                                                    response_body_body_body_cursor += b;
                                                    (
                                                        r#wasi::r#http::r#types::FieldSizePayload {
                                                            r#field_name: response_body_body_body_field_field_name,
                                                            r#field_size: response_body_body_body_field_field_size,
                                                        },
                                                        response_body_body_body_cursor,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                                response_body_case_HTTPRequestHeaderSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    23u32 => {
                                        let (response_body_case_HTTPRequestTrailerSectionSize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u32::from_ne_bytes(
                                                            response_body_body_body[0..4usize].try_into().unwrap(),
                                                        ),
                                                        4usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                                response_body_case_HTTPRequestTrailerSectionSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    24u32 => {
                                        let (response_body_case_HTTPRequestTrailerSize, b) = {
                                            let mut response_body_body_cursor = 0;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_name, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        let n = u32::from_ne_bytes(
                                                            response_body_body_field_body[0..4].try_into().unwrap(),
                                                        ) as usize;
                                                        let s = ::alloc::string::ToString::to_string(
                                                            ::core::str::from_utf8(
                                                                    &response_body_body_field_body[4..4 + n],
                                                                )
                                                                .unwrap(),
                                                        );
                                                        (s, n + 4)
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_size, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        (
                                                            u32::from_ne_bytes(
                                                                response_body_body_field_body[0..4usize].try_into().unwrap(),
                                                            ),
                                                            4usize,
                                                        )
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            (
                                                r#wasi::r#http::r#types::FieldSizePayload {
                                                    r#field_name: response_body_body_field_field_name,
                                                    r#field_size: response_body_body_field_field_size,
                                                },
                                                response_body_body_cursor,
                                            )
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                                response_body_case_HTTPRequestTrailerSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    25u32 => {
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete,
                                            4,
                                        )
                                    }
                                    26u32 => {
                                        let (response_body_case_HTTPResponseHeaderSectionSize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u32::from_ne_bytes(
                                                            response_body_body_body[0..4usize].try_into().unwrap(),
                                                        ),
                                                        4usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                                response_body_case_HTTPResponseHeaderSectionSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    27u32 => {
                                        let (response_body_case_HTTPResponseHeaderSize, b) = {
                                            let mut response_body_body_cursor = 0;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_name, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        let n = u32::from_ne_bytes(
                                                            response_body_body_field_body[0..4].try_into().unwrap(),
                                                        ) as usize;
                                                        let s = ::alloc::string::ToString::to_string(
                                                            ::core::str::from_utf8(
                                                                    &response_body_body_field_body[4..4 + n],
                                                                )
                                                                .unwrap(),
                                                        );
                                                        (s, n + 4)
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_size, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        (
                                                            u32::from_ne_bytes(
                                                                response_body_body_field_body[0..4usize].try_into().unwrap(),
                                                            ),
                                                            4usize,
                                                        )
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            (
                                                r#wasi::r#http::r#types::FieldSizePayload {
                                                    r#field_name: response_body_body_field_field_name,
                                                    r#field_size: response_body_body_field_field_size,
                                                },
                                                response_body_body_cursor,
                                            )
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                                response_body_case_HTTPResponseHeaderSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    28u32 => {
                                        let (response_body_case_HTTPResponseBodySize, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u64::from_ne_bytes(
                                                            response_body_body_body[0..8usize].try_into().unwrap(),
                                                        ),
                                                        8usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                                response_body_case_HTTPResponseBodySize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    29u32 => {
                                        let (
                                            response_body_case_HTTPResponseTrailerSectionSize,
                                            b,
                                        ) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    (
                                                        u32::from_ne_bytes(
                                                            response_body_body_body[0..4usize].try_into().unwrap(),
                                                        ),
                                                        4usize,
                                                    )
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                                response_body_case_HTTPResponseTrailerSectionSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    30u32 => {
                                        let (response_body_case_HTTPResponseTrailerSize, b) = {
                                            let mut response_body_body_cursor = 0;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_name, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        let n = u32::from_ne_bytes(
                                                            response_body_body_field_body[0..4].try_into().unwrap(),
                                                        ) as usize;
                                                        let s = ::alloc::string::ToString::to_string(
                                                            ::core::str::from_utf8(
                                                                    &response_body_body_field_body[4..4 + n],
                                                                )
                                                                .unwrap(),
                                                        );
                                                        (s, n + 4)
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            let response_body_body_field = &response_body_body[response_body_body_cursor..];
                                            let (response_body_body_field_field_size, b) = {
                                                let n = u8::from_ne_bytes(
                                                    response_body_body_field[0..1].try_into().unwrap(),
                                                );
                                                if n != 0 {
                                                    let response_body_body_field_body = &response_body_body_field[1..];
                                                    let (x, b) = {
                                                        (
                                                            u32::from_ne_bytes(
                                                                response_body_body_field_body[0..4usize].try_into().unwrap(),
                                                            ),
                                                            4usize,
                                                        )
                                                    };
                                                    (::core::option::Option::Some(x), b + 1)
                                                } else {
                                                    (::core::option::Option::None, 1)
                                                }
                                            };
                                            response_body_body_cursor += b;
                                            (
                                                r#wasi::r#http::r#types::FieldSizePayload {
                                                    r#field_name: response_body_body_field_field_name,
                                                    r#field_size: response_body_body_field_field_size,
                                                },
                                                response_body_body_cursor,
                                            )
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                                response_body_case_HTTPResponseTrailerSize,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    31u32 => {
                                        let (response_body_case_HTTPResponseTransferCoding, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    let n = u32::from_ne_bytes(
                                                        response_body_body_body[0..4].try_into().unwrap(),
                                                    ) as usize;
                                                    let s = ::alloc::string::ToString::to_string(
                                                        ::core::str::from_utf8(&response_body_body_body[4..4 + n])
                                                            .unwrap(),
                                                    );
                                                    (s, n + 4)
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                                response_body_case_HTTPResponseTransferCoding,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    32u32 => {
                                        let (response_body_case_HTTPResponseContentCoding, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    let n = u32::from_ne_bytes(
                                                        response_body_body_body[0..4].try_into().unwrap(),
                                                    ) as usize;
                                                    let s = ::alloc::string::ToString::to_string(
                                                        ::core::str::from_utf8(&response_body_body_body[4..4 + n])
                                                            .unwrap(),
                                                    );
                                                    (s, n + 4)
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                                response_body_case_HTTPResponseContentCoding,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    33u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout, 4)
                                    }
                                    34u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed, 4)
                                    }
                                    35u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError, 4)
                                    }
                                    36u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::LoopDetected, 4)
                                    }
                                    37u32 => {
                                        (r#wasi::r#http::r#types::ErrorCode::ConfigurationError, 4)
                                    }
                                    38u32 => {
                                        let (response_body_case_InternalError, b) = {
                                            let n = u8::from_ne_bytes(
                                                response_body_body[0..1].try_into().unwrap(),
                                            );
                                            if n != 0 {
                                                let response_body_body_body = &response_body_body[1..];
                                                let (x, b) = {
                                                    let n = u32::from_ne_bytes(
                                                        response_body_body_body[0..4].try_into().unwrap(),
                                                    ) as usize;
                                                    let s = ::alloc::string::ToString::to_string(
                                                        ::core::str::from_utf8(&response_body_body_body[4..4 + n])
                                                            .unwrap(),
                                                    );
                                                    (s, n + 4)
                                                };
                                                (::core::option::Option::Some(x), b + 1)
                                            } else {
                                                (::core::option::Option::None, 1)
                                            }
                                        };
                                        (
                                            r#wasi::r#http::r#types::ErrorCode::InternalError(
                                                response_body_case_InternalError,
                                            ),
                                            b + 4,
                                        )
                                    }
                                    _ => panic!("invalid value for variant"),
                                }
                            };
                            (::core::result::Result::Err(x), b + 1)
                        }
                    }
                        .0,
                );
                Ok(::alloc::vec::Vec::new())
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[constructor]outgoing-response",
            move |r#headers: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingResponse::new(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#headers[0..4].try_into().unwrap());
                        let Some(v) = rts.resource29[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource15.len();
                    rts.resource15
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-response.status-code",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingResponse::r#status_code(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource15[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({ alloc::vec::Vec::from(u16::to_ne_bytes(ret)) })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-response.set-status-code",
            move |self_: ::std::vec::Vec<u8>, r#status_code: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingResponse::r#set_status_code(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource15[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        (
                            u16::from_ne_bytes(
                                r#status_code[0..2usize].try_into().unwrap(),
                            ),
                            2usize,
                        )
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-response.headers",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingResponse::r#headers(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource15[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]outgoing-response.body",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::OutgoingResponse::r#body(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource15[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource18.len();
                                    rts.resource18
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-response.status",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingResponse::r#status(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource10[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({ alloc::vec::Vec::from(u16::to_ne_bytes(ret)) })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-response.headers",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingResponse::r#headers(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource10[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource29.len();
                    rts.resource29
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]incoming-response.consume",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::IncomingResponse::r#consume(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource10[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource20.len();
                                    rts.resource20
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]future-trailers.subscribe",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::FutureTrailers::r#subscribe(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource8[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]future-trailers.get",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::FutureTrailers::r#get(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource8[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    match ret_body {
                                        ::core::result::Result::Ok(ret_body_body) => {
                                            let mut ret_body_ret = alloc::vec::Vec::from(
                                                u8::to_ne_bytes(0),
                                            );
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::result::Result::Ok(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(0),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    match ret_body_body_body {
                                                                        ::core::option::Option::Some(ret_body_body_body_body) => {
                                                                            let mut ret_body_body_body_ret = alloc::vec::Vec::from(
                                                                                u8::to_ne_bytes(1),
                                                                            );
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let i = rts.resource29.len();
                                                                                    rts.resource29
                                                                                        .push_back(
                                                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                                                ret_body_body_body_body,
                                                                                            ),
                                                                                        );
                                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                                                });
                                                                            ret_body_body_body_ret
                                                                        }
                                                                        ::core::option::Option::None => {
                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                        }
                                                                    }
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::result::Result::Err(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_ret = alloc::vec::Vec::new();
                                                                    match ret_body_body_body {
                                                                        r#wasi::r#http::r#types::ErrorCode::DNSTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(0u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DNSError(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(1u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_rcode = ret_body_body_body_body
                                                                                        .r#rcode;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_rcode {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_rcode_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_rcode_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_rcode_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_rcode_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_rcode_body_bytes = ret_body_body_body_body_field_rcode_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_rcode_body_bytes.len() as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                                .extend(ret_body_body_body_body_field_rcode_body_bytes);
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_rcode_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_info_code = ret_body_body_body_body
                                                                                        .r#info_code;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_info_code {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_info_code_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_info_code_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_info_code_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u16::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_info_code_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_info_code_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationNotFound => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(2u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(3u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(4u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(5u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionRefused => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(6u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(7u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(8u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(9u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(10u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(11u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSProtocolError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(12u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSCertificateError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(13u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(14u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_alert_id = ret_body_body_body_body
                                                                                        .r#alert_id;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_alert_id {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_alert_id_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_alert_id_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_alert_id_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u8::to_ne_bytes(ret_body_body_body_body_field_alert_id_body),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_alert_id_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_alert_message = ret_body_body_body_body
                                                                                        .r#alert_message;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_alert_message {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_alert_message_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_alert_message_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_alert_message_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_alert_message_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_alert_message_body_bytes = ret_body_body_body_body_field_alert_message_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_alert_message_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_alert_message_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_alert_message_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(15u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(16u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(17u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u64::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(18u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(19u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(20u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(21u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(22u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_field_field_name = ret_body_body_body_body_body
                                                                                                        .r#field_name;
                                                                                                    ret_body_body_body_body_body_record
                                                                                                        .extend({
                                                                                                            match ret_body_body_body_body_body_field_field_name {
                                                                                                                ::core::option::Option::Some(
                                                                                                                    ret_body_body_body_body_body_field_field_name_body,
                                                                                                                ) => {
                                                                                                                    let mut ret_body_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                                        u8::to_ne_bytes(1),
                                                                                                                    );
                                                                                                                    ret_body_body_body_body_body_field_field_name_ret
                                                                                                                        .extend({
                                                                                                                            let mut ret_body_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                                            let ret_body_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_body_field_field_name_body
                                                                                                                                .into_bytes();
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                                .extend(
                                                                                                                                    alloc::vec::Vec::from(
                                                                                                                                        u32::to_ne_bytes(
                                                                                                                                            ret_body_body_body_body_body_field_field_name_body_bytes
                                                                                                                                                .len() as u32,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                );
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                                .extend(
                                                                                                                                    ret_body_body_body_body_body_field_field_name_body_bytes,
                                                                                                                                );
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                        });
                                                                                                                    ret_body_body_body_body_body_field_field_name_ret
                                                                                                                }
                                                                                                                ::core::option::Option::None => {
                                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                                }
                                                                                                            }
                                                                                                        });
                                                                                                    let ret_body_body_body_body_body_field_field_size = ret_body_body_body_body_body
                                                                                                        .r#field_size;
                                                                                                    ret_body_body_body_body_body_record
                                                                                                        .extend({
                                                                                                            match ret_body_body_body_body_body_field_field_size {
                                                                                                                ::core::option::Option::Some(
                                                                                                                    ret_body_body_body_body_body_field_field_size_body,
                                                                                                                ) => {
                                                                                                                    let mut ret_body_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                                        u8::to_ne_bytes(1),
                                                                                                                    );
                                                                                                                    ret_body_body_body_body_body_field_field_size_ret
                                                                                                                        .extend({
                                                                                                                            alloc::vec::Vec::from(
                                                                                                                                u32::to_ne_bytes(
                                                                                                                                    ret_body_body_body_body_body_field_field_size_body,
                                                                                                                                ),
                                                                                                                            )
                                                                                                                        });
                                                                                                                    ret_body_body_body_body_body_field_field_size_ret
                                                                                                                }
                                                                                                                ::core::option::Option::None => {
                                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                                }
                                                                                                            }
                                                                                                        });
                                                                                                    ret_body_body_body_body_body_record
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(23u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(24u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(25u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(26u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(27u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(28u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u64::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(29u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(30u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(31u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(32u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(33u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(34u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(35u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::LoopDetected => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(36u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConfigurationError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(37u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::InternalError(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(38u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                    }
                                                                    ret_body_body_body_ret
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                    }
                                                });
                                            ret_body_ret
                                        }
                                        ::core::result::Result::Err(ret_body_body) => {
                                            let mut ret_body_ret = alloc::vec::Vec::from(
                                                u8::to_ne_bytes(1),
                                            );
                                            ret_body_ret
                                        }
                                    }
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]future-incoming-response.subscribe",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::FutureIncomingResponse::r#subscribe(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource6[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    let i = rts.resource47.len();
                    rts.resource47
                        .push_back(
                            ::hyperlight_common::resource::ResourceEntry::give(ret),
                        );
                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::[method]future-incoming-response.get",
            move |self_: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::r#types::FutureIncomingResponse::r#get(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(self_[0..4].try_into().unwrap());
                        let Some(v) = rts.resource6[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    match ret_body {
                                        ::core::result::Result::Ok(ret_body_body) => {
                                            let mut ret_body_ret = alloc::vec::Vec::from(
                                                u8::to_ne_bytes(0),
                                            );
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::result::Result::Ok(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(0),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let i = rts.resource10.len();
                                                                    rts.resource10
                                                                        .push_back(
                                                                            ::hyperlight_common::resource::ResourceEntry::give(
                                                                                ret_body_body_body,
                                                                            ),
                                                                        );
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::result::Result::Err(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_ret = alloc::vec::Vec::new();
                                                                    match ret_body_body_body {
                                                                        r#wasi::r#http::r#types::ErrorCode::DNSTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(0u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DNSError(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(1u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_rcode = ret_body_body_body_body
                                                                                        .r#rcode;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_rcode {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_rcode_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_rcode_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_rcode_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_rcode_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_rcode_body_bytes = ret_body_body_body_body_field_rcode_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_rcode_body_bytes.len() as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                                .extend(ret_body_body_body_body_field_rcode_body_bytes);
                                                                                                            ret_body_body_body_body_field_rcode_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_rcode_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_info_code = ret_body_body_body_body
                                                                                        .r#info_code;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_info_code {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_info_code_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_info_code_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_info_code_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u16::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_info_code_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_info_code_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationNotFound => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(2u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(3u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(4u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(5u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionRefused => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(6u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(7u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(8u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(9u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(10u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(11u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSProtocolError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(12u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSCertificateError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(13u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(14u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_alert_id = ret_body_body_body_body
                                                                                        .r#alert_id;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_alert_id {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_alert_id_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_alert_id_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_alert_id_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u8::to_ne_bytes(ret_body_body_body_body_field_alert_id_body),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_alert_id_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_alert_message = ret_body_body_body_body
                                                                                        .r#alert_message;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_alert_message {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_alert_message_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_alert_message_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_alert_message_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_alert_message_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_alert_message_body_bytes = ret_body_body_body_body_field_alert_message_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_alert_message_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_alert_message_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_alert_message_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_alert_message_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(15u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(16u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(17u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u64::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(18u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(19u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(20u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(21u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(22u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_field_field_name = ret_body_body_body_body_body
                                                                                                        .r#field_name;
                                                                                                    ret_body_body_body_body_body_record
                                                                                                        .extend({
                                                                                                            match ret_body_body_body_body_body_field_field_name {
                                                                                                                ::core::option::Option::Some(
                                                                                                                    ret_body_body_body_body_body_field_field_name_body,
                                                                                                                ) => {
                                                                                                                    let mut ret_body_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                                        u8::to_ne_bytes(1),
                                                                                                                    );
                                                                                                                    ret_body_body_body_body_body_field_field_name_ret
                                                                                                                        .extend({
                                                                                                                            let mut ret_body_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                                            let ret_body_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_body_field_field_name_body
                                                                                                                                .into_bytes();
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                                .extend(
                                                                                                                                    alloc::vec::Vec::from(
                                                                                                                                        u32::to_ne_bytes(
                                                                                                                                            ret_body_body_body_body_body_field_field_name_body_bytes
                                                                                                                                                .len() as u32,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                );
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                                .extend(
                                                                                                                                    ret_body_body_body_body_body_field_field_name_body_bytes,
                                                                                                                                );
                                                                                                                            ret_body_body_body_body_body_field_field_name_body_string
                                                                                                                        });
                                                                                                                    ret_body_body_body_body_body_field_field_name_ret
                                                                                                                }
                                                                                                                ::core::option::Option::None => {
                                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                                }
                                                                                                            }
                                                                                                        });
                                                                                                    let ret_body_body_body_body_body_field_field_size = ret_body_body_body_body_body
                                                                                                        .r#field_size;
                                                                                                    ret_body_body_body_body_body_record
                                                                                                        .extend({
                                                                                                            match ret_body_body_body_body_body_field_field_size {
                                                                                                                ::core::option::Option::Some(
                                                                                                                    ret_body_body_body_body_body_field_field_size_body,
                                                                                                                ) => {
                                                                                                                    let mut ret_body_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                                        u8::to_ne_bytes(1),
                                                                                                                    );
                                                                                                                    ret_body_body_body_body_body_field_field_size_ret
                                                                                                                        .extend({
                                                                                                                            alloc::vec::Vec::from(
                                                                                                                                u32::to_ne_bytes(
                                                                                                                                    ret_body_body_body_body_body_field_field_size_body,
                                                                                                                                ),
                                                                                                                            )
                                                                                                                        });
                                                                                                                    ret_body_body_body_body_body_field_field_size_ret
                                                                                                                }
                                                                                                                ::core::option::Option::None => {
                                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                                }
                                                                                                            }
                                                                                                        });
                                                                                                    ret_body_body_body_body_body_record
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(23u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(24u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(25u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(26u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(27u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(28u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u64::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(29u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(ret_body_body_body_body_body),
                                                                                                    )
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(30u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    let mut ret_body_body_body_body_record = alloc::vec::Vec::new();
                                                                                    let ret_body_body_body_body_field_field_name = ret_body_body_body_body
                                                                                        .r#field_name;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_name {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_name_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                        .extend({
                                                                                                            let mut ret_body_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                                            let ret_body_body_body_body_field_field_name_body_bytes = ret_body_body_body_body_field_field_name_body
                                                                                                                .into_bytes();
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    alloc::vec::Vec::from(
                                                                                                                        u32::to_ne_bytes(
                                                                                                                            ret_body_body_body_body_field_field_name_body_bytes.len()
                                                                                                                                as u32,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                                .extend(
                                                                                                                    ret_body_body_body_body_field_field_name_body_bytes,
                                                                                                                );
                                                                                                            ret_body_body_body_body_field_field_name_body_string
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_name_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    let ret_body_body_body_body_field_field_size = ret_body_body_body_body
                                                                                        .r#field_size;
                                                                                    ret_body_body_body_body_record
                                                                                        .extend({
                                                                                            match ret_body_body_body_body_field_field_size {
                                                                                                ::core::option::Option::Some(
                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                ) => {
                                                                                                    let mut ret_body_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                                        u8::to_ne_bytes(1),
                                                                                                    );
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                        .extend({
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_field_field_size_body,
                                                                                                                ),
                                                                                                            )
                                                                                                        });
                                                                                                    ret_body_body_body_body_field_field_size_ret
                                                                                                }
                                                                                                ::core::option::Option::None => {
                                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                                }
                                                                                            }
                                                                                        });
                                                                                    ret_body_body_body_body_record
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(31u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(32u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(33u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(34u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(35u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::LoopDetected => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(36u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::ConfigurationError => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(37u32));
                                                                        }
                                                                        r#wasi::r#http::r#types::ErrorCode::InternalError(
                                                                            ret_body_body_body_body,
                                                                        ) => {
                                                                            ret_body_body_body_ret.extend(u32::to_ne_bytes(38u32));
                                                                            ret_body_body_body_ret
                                                                                .extend({
                                                                                    match ret_body_body_body_body {
                                                                                        ::core::option::Option::Some(
                                                                                            ret_body_body_body_body_body,
                                                                                        ) => {
                                                                                            let mut ret_body_body_body_body_ret = alloc::vec::Vec::from(
                                                                                                u8::to_ne_bytes(1),
                                                                                            );
                                                                                            ret_body_body_body_body_ret
                                                                                                .extend({
                                                                                                    let mut ret_body_body_body_body_body_string = alloc::vec::Vec::new();
                                                                                                    let ret_body_body_body_body_body_bytes = ret_body_body_body_body_body
                                                                                                        .into_bytes();
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(
                                                                                                            alloc::vec::Vec::from(
                                                                                                                u32::to_ne_bytes(
                                                                                                                    ret_body_body_body_body_body_bytes.len() as u32,
                                                                                                                ),
                                                                                                            ),
                                                                                                        );
                                                                                                    ret_body_body_body_body_body_string
                                                                                                        .extend(ret_body_body_body_body_body_bytes);
                                                                                                    ret_body_body_body_body_body_string
                                                                                                });
                                                                                            ret_body_body_body_body_ret
                                                                                        }
                                                                                        ::core::option::Option::None => {
                                                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                        }
                                                                                    }
                                                                                })
                                                                        }
                                                                    }
                                                                    ret_body_body_body_ret
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                    }
                                                });
                                            ret_body_ret
                                        }
                                        ::core::result::Result::Err(ret_body_body) => {
                                            let mut ret_body_ret = alloc::vec::Vec::from(
                                                u8::to_ne_bytes(1),
                                            );
                                            ret_body_ret
                                        }
                                    }
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::http-error-code",
            move |r#err: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#types(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::Types::r#http_error_code(
                    ::std::borrow::BorrowMut::<I::Types>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#err[0..4].try_into().unwrap());
                        let Some(v) = rts.resource43[i as usize].borrow() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::option::Option::Some(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::ErrorCode::DNSTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DNSError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_rcode = ret_body_body.r#rcode;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_rcode {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_rcode_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_rcode_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_rcode_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_rcode_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_rcode_body_bytes = ret_body_body_field_rcode_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_rcode_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(ret_body_body_field_rcode_body_bytes);
                                                                            ret_body_body_field_rcode_body_string
                                                                        });
                                                                    ret_body_body_field_rcode_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_info_code = ret_body_body
                                                        .r#info_code;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_info_code {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_info_code_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_info_code_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_info_code_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u16::to_ne_bytes(ret_body_body_field_info_code_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_info_code_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationNotFound => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(3u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited => {
                                            ret_body_ret.extend(u32::to_ne_bytes(4u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(5u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionRefused => {
                                            ret_body_ret.extend(u32::to_ne_bytes(6u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated => {
                                            ret_body_ret.extend(u32::to_ne_bytes(7u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(8u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(9u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(10u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached => {
                                            ret_body_ret.extend(u32::to_ne_bytes(11u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(12u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSCertificateError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(13u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(14u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_alert_id = ret_body_body.r#alert_id;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_id {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_id_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_id_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_id_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u8::to_ne_bytes(ret_body_body_field_alert_id_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_alert_id_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_alert_message = ret_body_body
                                                        .r#alert_message;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_message {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_message_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_message_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_message_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_alert_message_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_alert_message_body_bytes = ret_body_body_field_alert_message_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_alert_message_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(ret_body_body_field_alert_message_body_bytes);
                                                                            ret_body_body_field_alert_message_body_string
                                                                        });
                                                                    ret_body_body_field_alert_message_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied => {
                                            ret_body_ret.extend(u32::to_ne_bytes(15u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired => {
                                            ret_body_ret.extend(u32::to_ne_bytes(16u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(17u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(18u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(19u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong => {
                                            ret_body_ret.extend(u32::to_ne_bytes(20u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(21u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(22u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_record = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_field_field_name = ret_body_body_body
                                                                        .r#field_name;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_name {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_name_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                        .extend({
                                                                                            let mut ret_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                            let ret_body_body_body_field_field_name_body_bytes = ret_body_body_body_field_field_name_body
                                                                                                .into_bytes();
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(
                                                                                                            ret_body_body_body_field_field_name_body_bytes.len() as u32,
                                                                                                        ),
                                                                                                    ),
                                                                                                );
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(ret_body_body_body_field_field_name_body_bytes);
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                        });
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    let ret_body_body_body_field_field_size = ret_body_body_body
                                                                        .r#field_size;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_size {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_size_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                        .extend({
                                                                                            alloc::vec::Vec::from(
                                                                                                u32::to_ne_bytes(ret_body_body_body_field_field_size_body),
                                                                                            )
                                                                                        });
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    ret_body_body_body_record
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(23u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(24u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete => {
                                            ret_body_ret.extend(u32::to_ne_bytes(25u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(26u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(27u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(28u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(29u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(30u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(31u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(32u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(33u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(34u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(35u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::LoopDetected => {
                                            ret_body_ret.extend(u32::to_ne_bytes(36u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConfigurationError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(37u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::InternalError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(38u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                        ::core::option::Option::None => {
                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                        }
                    }
                })
            },
        )
        .unwrap();
    let captured_imports = imports.clone();
    let captured_rts = rts.clone();
    sb.register_host_function(
            "r#wasi::r#http::handle",
            move |r#request: ::std::vec::Vec<u8>, r#options: ::std::vec::Vec<u8>| {
                let mut rts = captured_rts.lock().unwrap();
                let mut slf = captured_imports.lock().unwrap();
                let mut slf = ::std::ops::DerefMut::deref_mut(&mut slf);
                let mut slf = r#root::r#component::RootImports::r#outgoing_handler(
                    ::std::borrow::BorrowMut::<I>::borrow_mut(&mut slf),
                );
                let ret = r#wasi::r#http::OutgoingHandler::r#handle(
                    ::std::borrow::BorrowMut::<I::OutgoingHandler>::borrow_mut(&mut slf),
                    {
                        let i = u32::from_ne_bytes(r#request[0..4].try_into().unwrap());
                        let Some(v) = rts.resource19[i as usize].take() else {
                            panic!("");
                        };
                        (v, 4)
                    }
                        .0,
                    {
                        let n = u8::from_ne_bytes(r#options[0..1].try_into().unwrap());
                        if n != 0 {
                            let options_body = &r#options[1..];
                            let (x, b) = {
                                let i = u32::from_ne_bytes(
                                    options_body[0..4].try_into().unwrap(),
                                );
                                let Some(v) = rts.resource17[i as usize].take() else {
                                    panic!("");
                                };
                                (v, 4)
                            };
                            (::core::option::Option::Some(x), b + 1)
                        } else {
                            (::core::option::Option::None, 1)
                        }
                    }
                        .0,
                );
                Ok({
                    match ret {
                        ::core::result::Result::Ok(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(0));
                            ret_ret
                                .extend({
                                    let i = rts.resource6.len();
                                    rts.resource6
                                        .push_back(
                                            ::hyperlight_common::resource::ResourceEntry::give(ret_body),
                                        );
                                    alloc::vec::Vec::from(u32::to_ne_bytes(i as u32))
                                });
                            ret_ret
                        }
                        ::core::result::Result::Err(ret_body) => {
                            let mut ret_ret = alloc::vec::Vec::from(u8::to_ne_bytes(1));
                            ret_ret
                                .extend({
                                    let mut ret_body_ret = alloc::vec::Vec::new();
                                    match ret_body {
                                        r#wasi::r#http::r#types::ErrorCode::DNSTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(0u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DNSError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(1u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_rcode = ret_body_body.r#rcode;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_rcode {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_rcode_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_rcode_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_rcode_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_rcode_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_rcode_body_bytes = ret_body_body_field_rcode_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_rcode_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_rcode_body_string
                                                                                .extend(ret_body_body_field_rcode_body_bytes);
                                                                            ret_body_body_field_rcode_body_string
                                                                        });
                                                                    ret_body_body_field_rcode_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_info_code = ret_body_body
                                                        .r#info_code;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_info_code {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_info_code_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_info_code_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_info_code_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u16::to_ne_bytes(ret_body_body_field_info_code_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_info_code_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationNotFound => {
                                            ret_body_ret.extend(u32::to_ne_bytes(2u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationUnavailable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(3u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPProhibited => {
                                            ret_body_ret.extend(u32::to_ne_bytes(4u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::DestinationIPUnroutable => {
                                            ret_body_ret.extend(u32::to_ne_bytes(5u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionRefused => {
                                            ret_body_ret.extend(u32::to_ne_bytes(6u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTerminated => {
                                            ret_body_ret.extend(u32::to_ne_bytes(7u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(8u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionReadTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(9u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionWriteTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(10u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConnectionLimitReached => {
                                            ret_body_ret.extend(u32::to_ne_bytes(11u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(12u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSCertificateError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(13u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::TLSAlertReceived(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(14u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_alert_id = ret_body_body.r#alert_id;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_id {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_id_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_id_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_id_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u8::to_ne_bytes(ret_body_body_field_alert_id_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_alert_id_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_alert_message = ret_body_body
                                                        .r#alert_message;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_alert_message {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_alert_message_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_alert_message_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_alert_message_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_alert_message_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_alert_message_body_bytes = ret_body_body_field_alert_message_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_alert_message_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_alert_message_body_string
                                                                                .extend(ret_body_body_field_alert_message_body_bytes);
                                                                            ret_body_body_field_alert_message_body_string
                                                                        });
                                                                    ret_body_body_field_alert_message_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestDenied => {
                                            ret_body_ret.extend(u32::to_ne_bytes(15u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestLengthRequired => {
                                            ret_body_ret.extend(u32::to_ne_bytes(16u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(17u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestMethodInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(18u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURIInvalid => {
                                            ret_body_ret.extend(u32::to_ne_bytes(19u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestURITooLong => {
                                            ret_body_ret.extend(u32::to_ne_bytes(20u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(21u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(22u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_record = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_field_field_name = ret_body_body_body
                                                                        .r#field_name;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_name {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_name_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                        .extend({
                                                                                            let mut ret_body_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                                            let ret_body_body_body_field_field_name_body_bytes = ret_body_body_body_field_field_name_body
                                                                                                .into_bytes();
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(
                                                                                                    alloc::vec::Vec::from(
                                                                                                        u32::to_ne_bytes(
                                                                                                            ret_body_body_body_field_field_name_body_bytes.len() as u32,
                                                                                                        ),
                                                                                                    ),
                                                                                                );
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                                .extend(ret_body_body_body_field_field_name_body_bytes);
                                                                                            ret_body_body_body_field_field_name_body_string
                                                                                        });
                                                                                    ret_body_body_body_field_field_name_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    let ret_body_body_body_field_field_size = ret_body_body_body
                                                                        .r#field_size;
                                                                    ret_body_body_body_record
                                                                        .extend({
                                                                            match ret_body_body_body_field_field_size {
                                                                                ::core::option::Option::Some(
                                                                                    ret_body_body_body_field_field_size_body,
                                                                                ) => {
                                                                                    let mut ret_body_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                                        u8::to_ne_bytes(1),
                                                                                    );
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                        .extend({
                                                                                            alloc::vec::Vec::from(
                                                                                                u32::to_ne_bytes(ret_body_body_body_field_field_size_body),
                                                                                            )
                                                                                        });
                                                                                    ret_body_body_body_field_field_size_ret
                                                                                }
                                                                                ::core::option::Option::None => {
                                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                                }
                                                                            }
                                                                        });
                                                                    ret_body_body_body_record
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(23u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPRequestTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(24u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseIncomplete => {
                                            ret_body_ret.extend(u32::to_ne_bytes(25u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(26u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseHeaderSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(27u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseBodySize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(28u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u64::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSectionSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(29u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    alloc::vec::Vec::from(u32::to_ne_bytes(ret_body_body_body))
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTrailerSize(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(30u32));
                                            ret_body_ret
                                                .extend({
                                                    let mut ret_body_body_record = alloc::vec::Vec::new();
                                                    let ret_body_body_field_field_name = ret_body_body
                                                        .r#field_name;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_name {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_name_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_name_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_name_ret
                                                                        .extend({
                                                                            let mut ret_body_body_field_field_name_body_string = alloc::vec::Vec::new();
                                                                            let ret_body_body_field_field_name_body_bytes = ret_body_body_field_field_name_body
                                                                                .into_bytes();
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(
                                                                                    alloc::vec::Vec::from(
                                                                                        u32::to_ne_bytes(
                                                                                            ret_body_body_field_field_name_body_bytes.len() as u32,
                                                                                        ),
                                                                                    ),
                                                                                );
                                                                            ret_body_body_field_field_name_body_string
                                                                                .extend(ret_body_body_field_field_name_body_bytes);
                                                                            ret_body_body_field_field_name_body_string
                                                                        });
                                                                    ret_body_body_field_field_name_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    let ret_body_body_field_field_size = ret_body_body
                                                        .r#field_size;
                                                    ret_body_body_record
                                                        .extend({
                                                            match ret_body_body_field_field_size {
                                                                ::core::option::Option::Some(
                                                                    ret_body_body_field_field_size_body,
                                                                ) => {
                                                                    let mut ret_body_body_field_field_size_ret = alloc::vec::Vec::from(
                                                                        u8::to_ne_bytes(1),
                                                                    );
                                                                    ret_body_body_field_field_size_ret
                                                                        .extend({
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_field_field_size_body),
                                                                            )
                                                                        });
                                                                    ret_body_body_field_field_size_ret
                                                                }
                                                                ::core::option::Option::None => {
                                                                    alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                                }
                                                            }
                                                        });
                                                    ret_body_body_record
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTransferCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(31u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseContentCoding(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(32u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPResponseTimeout => {
                                            ret_body_ret.extend(u32::to_ne_bytes(33u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPUpgradeFailed => {
                                            ret_body_ret.extend(u32::to_ne_bytes(34u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::HTTPProtocolError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(35u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::LoopDetected => {
                                            ret_body_ret.extend(u32::to_ne_bytes(36u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::ConfigurationError => {
                                            ret_body_ret.extend(u32::to_ne_bytes(37u32));
                                        }
                                        r#wasi::r#http::r#types::ErrorCode::InternalError(
                                            ret_body_body,
                                        ) => {
                                            ret_body_ret.extend(u32::to_ne_bytes(38u32));
                                            ret_body_ret
                                                .extend({
                                                    match ret_body_body {
                                                        ::core::option::Option::Some(ret_body_body_body) => {
                                                            let mut ret_body_body_ret = alloc::vec::Vec::from(
                                                                u8::to_ne_bytes(1),
                                                            );
                                                            ret_body_body_ret
                                                                .extend({
                                                                    let mut ret_body_body_body_string = alloc::vec::Vec::new();
                                                                    let ret_body_body_body_bytes = ret_body_body_body
                                                                        .into_bytes();
                                                                    ret_body_body_body_string
                                                                        .extend(
                                                                            alloc::vec::Vec::from(
                                                                                u32::to_ne_bytes(ret_body_body_body_bytes.len() as u32),
                                                                            ),
                                                                        );
                                                                    ret_body_body_body_string.extend(ret_body_body_body_bytes);
                                                                    ret_body_body_body_string
                                                                });
                                                            ret_body_body_ret
                                                        }
                                                        ::core::option::Option::None => {
                                                            alloc::vec::Vec::from(u8::to_ne_bytes(0))
                                                        }
                                                    }
                                                })
                                        }
                                    }
                                    ret_body_ret
                                });
                            ret_ret
                        }
                    }
                })
            },
        )
        .unwrap();
    rts
}
impl<
    I: r#root::r#component::RootImports + ::std::marker::Send,
    S: ::hyperlight_host::sandbox::Callable,
> r#root::r#component::RootExports<I> for RootSandbox<I, S> {
    type IncomingHandler = Self;
    #[allow(refining_impl_trait)]
    fn r#incoming_handler<'a>(&'a mut self) -> &'a mut Self {
        self
    }
}
impl r#root::r#component::Root for ::hyperlight_host::sandbox::UninitializedSandbox {
    type Exports<I: r#root::r#component::RootImports + ::std::marker::Send> = RootSandbox<
        I,
        ::hyperlight_host::sandbox::initialized_multi_use::MultiUseSandbox,
    >;
    fn instantiate<I: r#root::r#component::RootImports + ::std::marker::Send + 'static>(
        mut self,
        i: I,
    ) -> Self::Exports<I> {
        let rts = register_host_functions(&mut self, i);
        let sb = self.evolve().unwrap();
        RootSandbox { sb, rt: rts }
    }
}
