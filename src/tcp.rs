use async_std::{io::ErrorKind, net};
use futures::{future, AsyncReadExt, AsyncWriteExt, Future};

use crate::{conversions::SocketAddr, SocketState};

/// A TCP socket
pub struct TcpSocket {
    state: SocketState<net::TcpStream, net::TcpListener>,
}

impl TcpSocket {
    /// Create a new closed socket
    pub(crate) fn new() -> Self {
        Self {
            state: SocketState::Closed,
        }
    }

    /// Create a new already connected socket
    /// Useful for socket features not exposed by embedded-nal
    pub fn connected(stream: net::TcpStream) -> Self {
        Self {
            state: SocketState::Connected(stream),
        }
    }

    /// Create a new already connected socket
    /// Useful for socket features not exposed by embedded-nal
    pub fn bound(listener: net::TcpListener) -> Self {
        Self {
            state: SocketState::Bound(listener),
        }
    }
}

impl From<net::TcpStream> for TcpSocket {
    fn from(stream: net::TcpStream) -> Self {
        TcpSocket::connected(stream)
    }
}

impl From<net::TcpListener> for TcpSocket {
    fn from(listener: net::TcpListener) -> Self {
        TcpSocket::bound(listener)
    }
}

impl embedded_nal_async::TcpClientStack for crate::Stack {
    type TcpSocket = TcpSocket;

    type Error = async_std::io::Error;

    type SocketFuture<'m> = future::Ready<Result<TcpSocket, Self::Error>>
    where
        Self: 'm;

    fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m> {
        future::ready(Ok(TcpSocket::new()))
    }

    type ConnectFuture<'m> = impl Future<Output = Result<(), Self::Error>>
    where
        Self: 'm;

    fn connect<'m>(
        &'m mut self,
        socket: &'m mut Self::TcpSocket,
        remote: embedded_nal_async::SocketAddr,
    ) -> Self::ConnectFuture<'m> {
        async move {
            let addrs: SocketAddr = remote.into();
            let s = net::TcpStream::connect(addrs.0).await?;
            socket.state = SocketState::Connected(s);
            Ok(())
        }
    }

    type IsConnectedFuture<'m> = future::Ready<Result<bool, Self::Error>>
    where
        Self: 'm;

    fn is_connected<'m>(&'m mut self, socket: &'m Self::TcpSocket) -> Self::IsConnectedFuture<'m> {
        future::ready(Ok(matches!(socket.state, SocketState::Connected(_))))
    }

    type SendFuture<'m> = impl Future<Output = Result<usize, Self::Error>>
    where
        Self: 'm;

    fn send<'m>(
        &'m mut self,
        socket: &'m mut Self::TcpSocket,
        buffer: &'m [u8],
    ) -> Self::SendFuture<'m> {
        async move {
            let n = socket.state.get_connected()?.write(buffer).await?;
            Ok(n)
        }
    }

    type ReceiveFuture<'m> = impl Future<Output = Result<usize, Self::Error>>
    where
        Self: 'm;

    fn receive<'m>(
        &'m mut self,
        socket: &'m mut Self::TcpSocket,
        buffer: &'m mut [u8],
    ) -> Self::ReceiveFuture<'m> {
        async move { socket.state.get_connected()?.read(buffer).await }
    }

    type CloseFuture<'m> = future::Ready<Result<(), Self::Error>>
    where
        Self: 'm;

    fn close<'m>(&'m mut self, _socket: Self::TcpSocket) -> Self::CloseFuture<'m> {
        future::ready(Ok(()))
    }
}

impl embedded_nal_async::TcpFullStack for crate::Stack {
    type BindFuture<'m> = impl Future<Output = Result<(), Self::Error>>
	where
		Self: 'm;

    fn bind<'m>(
        &'m mut self,
        socket: &'m mut Self::TcpSocket,
        local_port: u16,
    ) -> Self::BindFuture<'m> {
        async move {
            let b = net::TcpListener::bind((net::Ipv4Addr::UNSPECIFIED, local_port)).await?;
            socket.state = SocketState::Bound(b);
            Ok(())
        }
    }

    type ListenFuture<'m> = future::Ready<Result<(), Self::Error>>
	where
		Self: 'm;

    fn listen<'m>(&'m mut self, _socket: &'m mut Self::TcpSocket) -> Self::ListenFuture<'m> {
        future::ready(Ok(()))
    }

    type AcceptFuture<'m> = impl Future<Output = Result<(Self::TcpSocket, embedded_nal_async::SocketAddr), Self::Error>>
	where
		Self: 'm;

    fn accept<'m>(&'m mut self, socket: &'m mut Self::TcpSocket) -> Self::AcceptFuture<'m> {
        async move {
            let (stream, addr) = socket.state.get_bound()?.accept().await?;
            let socket = TcpSocket::connected(stream);
            let peer: embedded_nal_async::SocketAddr = SocketAddr(addr).into();
            Ok((socket, peer))
        }
    }
}
