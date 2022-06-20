use async_std::{io, net};

use futures::{future, Future};

use crate::SocketState;

pub struct UdpSocket {
    state: SocketState<net::UdpSocket, net::UdpSocket>,
}

impl UdpSocket {
    pub fn new() -> Self {
        Self {
            state: SocketState::Closed,
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
            let addrs = match remote {
                embedded_nal_async::SocketAddr::V4(v4) => {
                    let ip = net::Ipv4Addr::from(v4.ip().octets());
                    net::SocketAddr::V4(net::SocketAddrV4::new(ip, v4.port()))
                }
                embedded_nal_async::SocketAddr::V6(v6) => {
                    let ip = net::Ipv6Addr::from(v6.ip().octets());
                    net::SocketAddr::V6(net::SocketAddrV6::new(
                        ip,
                        v6.port(),
                        v6.flowinfo(),
                        v6.scope_id(),
                    ))
                }
            };
            let s = net::UdpSocket::bind(unspecified).await?;
            s.connect(addrs).await?;
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
            socket.state.get_connected()?.send(buffer).await?;
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
            let (len, addr) = socket.state.get_connected()?.recv_from(buffer).await?;
            let addr = match addr {
                net::SocketAddr::V4(v4) => {
                    let ip = embedded_nal_async::Ipv4Addr::from(v4.ip().octets());
                    embedded_nal_async::SocketAddrV4::new(ip, v4.port()).into()
                }
                net::SocketAddr::V6(v6) => {
                    let ip = embedded_nal_async::Ipv6Addr::from(v6.ip().octets());
                    embedded_nal_async::SocketAddrV6::new(
                        ip,
                        v6.port(),
                        v6.flowinfo(),
                        v6.scope_id(),
                    )
                    .into()
                }
            };
            Ok((len, addr))
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
            let unspecified =
                net::SocketAddr::new(net::IpAddr::V6(net::Ipv6Addr::UNSPECIFIED), local_port);
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
            let addrs = match remote {
                embedded_nal_async::SocketAddr::V4(v4) => {
                    let ip = net::Ipv4Addr::from(v4.ip().octets());
                    net::SocketAddr::V4(net::SocketAddrV4::new(ip, v4.port()))
                }
                embedded_nal_async::SocketAddr::V6(v6) => {
                    let ip = net::Ipv6Addr::from(v6.ip().octets());
                    net::SocketAddr::V6(net::SocketAddrV6::new(
                        ip,
                        v6.port(),
                        v6.flowinfo(),
                        v6.scope_id(),
                    ))
                }
            };
            socket.state.get_bound()?.send_to(buffer, addrs).await?;
            Ok(())
        }
    }
}
