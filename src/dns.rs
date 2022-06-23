use async_std::{
    io,
    net::{self, ToSocketAddrs},
};
use futures::{future, Future};

use embedded_nal_async::heapless::String;

#[derive(Debug)]
struct NotFound;

impl embedded_nal_async::Dns for crate::Stack {
    type Error = io::Error;

    type GetHostByNameFuture<'m> = impl Future<Output = Result<embedded_nal_async::IpAddr, Self::Error>>
	where
		Self: 'm;

    fn get_host_by_name<'m>(
        &'m self,
        host: &'m str,
        addr_type: embedded_nal_async::AddrType,
    ) -> Self::GetHostByNameFuture<'m> {
        async move {
            let fake_port = match host.find(':') {
                Some(_) => format!("[{}]:1234", host),
                None => format!("{}:1234", host),
            };

            for addr in fake_port.to_socket_addrs().await? {
                match addr {
                    net::SocketAddr::V4(v4) if addr_type != embedded_nal_async::AddrType::IPv6 => {
                        return Ok(embedded_nal_async::IpAddr::from(v4.ip().octets()));
                    }
                    net::SocketAddr::V6(v6) if addr_type != embedded_nal_async::AddrType::IPv4 => {
                        return Ok(embedded_nal_async::IpAddr::from(v6.ip().octets()));
                    }
                    _ => continue,
                }
            }
            Err(io::ErrorKind::NotFound.into())
        }
    }

    type GetHostByAddressFuture<'m> = future::Ready<Result<String<256>, Self::Error>>
	where
		Self: 'm;

    fn get_host_by_address(
        &self,
        _addr: embedded_nal_async::IpAddr,
    ) -> Self::GetHostByAddressFuture<'_> {
        future::ready(Err(io::ErrorKind::NotFound.into()))
    }
}
