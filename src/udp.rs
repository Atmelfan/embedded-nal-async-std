#![allow(clippy::needless_lifetimes)]

use async_std::{io, net};
use futures::{future, Future};

use crate::{conversions::SocketAddr, SocketState};

/// A UDP socket
pub struct UdpSocket {
    state: SocketState<net::UdpSocket, net::UdpSocket>,
}

impl UdpSocket {
    /// New unconnected socket
    pub(crate) fn new() -> Self {
        Self {
            state: SocketState::Closed,
        }
    }

    /// Create a new already connected socket
    /// Useful for socket features not exposed by embedded-nal
    pub fn connected(socket: net::UdpSocket) -> Self {
        Self {
            state: SocketState::Connected(socket),
        }
    }

    /// Create an already bounded socket
    /// Useful for socket features not exposed by embedded-nal
    pub fn bound(socket: net::UdpSocket) -> Self {
        Self {
            state: SocketState::Bound(socket),
        }
    }
}

impl embedded_nal_async::UdpClientStack for crate::Stack {
    type UdpSocket = UdpSocket;

    type Error = io::Error;

    type SocketFuture<'m> = future::Ready<io::Result<Self::UdpSocket>>
	where
		Self: 'm;

    fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m> {
        future::ready(Ok(UdpSocket::new()))
    }

    type ConnectFuture<'m> = impl Future<Output = Result<(), Self::Error>>
    where
        Self: 'm;

    fn connect<'m>(
        &'m mut self,
        socket: &'m mut Self::UdpSocket,
        remote: embedded_nal_async::SocketAddr,
    ) -> Self::ConnectFuture<'m> {
        async move {
            let unspecified = match remote {
                embedded_nal_async::SocketAddr::V4(_) => {
                    net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::UNSPECIFIED), 0)
                }
                embedded_nal_async::SocketAddr::V6(_) => {
                    net::SocketAddr::new(net::IpAddr::V6(net::Ipv6Addr::UNSPECIFIED), 0)
                }
            };
            let addrs: SocketAddr = remote.into();
            let s = net::UdpSocket::bind(unspecified).await?;
            s.connect(addrs.0).await?;
            socket.state = SocketState::Connected(s);
            Ok(())
        }
    }

    type SendFuture<'m> = impl Future<Output = Result<(), Self::Error>>
    where
        Self: 'm;

    fn send<'m>(
        &'m mut self,
        socket: &'m mut Self::UdpSocket,
        buffer: &'m [u8],
    ) -> Self::SendFuture<'m> {
        async move {
            socket.state.get_either()?.send(buffer).await?;
            Ok(())
        }
    }

    type ReceiveFuture<'m> = impl Future<Output = Result<(usize, embedded_nal_async::SocketAddr), Self::Error>>
    where
        Self: 'm;

    fn receive<'m>(
        &'m mut self,
        socket: &'m mut Self::UdpSocket,
        buffer: &'m mut [u8],
    ) -> Self::ReceiveFuture<'m> {
        async move {
            let (len, addr) = socket.state.get_either()?.recv_from(buffer).await?;
            let addr = SocketAddr(addr);
            Ok((len, addr.into()))
        }
    }

    type CloseFuture<'m> = futures::future::Ready<Result<(), Self::Error>>
    where
        Self: 'm;

    fn close<'m>(&'m mut self, _socket: Self::UdpSocket) -> Self::CloseFuture<'m> {
        future::ready(Ok(()))
    }
}

impl embedded_nal_async::UdpFullStack for crate::Stack {
    type BindFuture<'m> = impl Future<Output = Result<(), Self::Error>>
	where
		Self: 'm;

    fn bind<'m>(
        &'m mut self,
        socket: &'m mut Self::UdpSocket,
        local_port: u16,
    ) -> Self::BindFuture<'m> {
        async move {
            let unspecified = net::SocketAddr::new(self.ip, local_port);
            let s = net::UdpSocket::bind(unspecified).await?;
            socket.state = SocketState::Bound(s);
            Ok(())
        }
    }

    type SendToFuture<'m> = impl Future<Output = Result<(), Self::Error>>
	where
		Self: 'm;

    fn send_to<'m>(
        &'m mut self,
        socket: &'m mut Self::UdpSocket,
        remote: embedded_nal_async::SocketAddr,
        buffer: &'m [u8],
    ) -> Self::SendToFuture<'m> {
        async move {
            let addrs: SocketAddr = remote.into();
            socket.state.get_bound()?.send_to(buffer, addrs.0).await?;
            Ok(())
        }
    }
}
