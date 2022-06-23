use async_std::net;

pub(crate) struct IpAddr(pub(crate) async_std::net::IpAddr);

impl From<IpAddr> for net::IpAddr {
    fn from(ip: IpAddr) -> Self {
        ip.0
    }
}

impl From<net::IpAddr> for IpAddr {
    fn from(ip: net::IpAddr) -> Self {
        Self(ip)
    }
}

impl From<embedded_nal_async::IpAddr> for IpAddr {
    fn from(ip: embedded_nal_async::IpAddr) -> Self {
        match ip {
            embedded_nal_async::IpAddr::V4(v4) => Self(v4.octets().into()),
            embedded_nal_async::IpAddr::V6(v6) => Self(v6.octets().into()),
        }
    }
}

impl From<IpAddr> for embedded_nal_async::IpAddr {
    fn from(ip: IpAddr) -> Self {
        match ip.0 {
            net::IpAddr::V4(v4) => v4.octets().into(),
            net::IpAddr::V6(v6) => v6.octets().into(),
        }
    }
}

pub(crate) struct SocketAddr(pub(crate) net::SocketAddr);

impl From<SocketAddr> for net::SocketAddr {
    fn from(addr: SocketAddr) -> Self {
        addr.0
    }
}

impl From<embedded_nal_async::SocketAddr> for SocketAddr {
    fn from(addr: embedded_nal_async::SocketAddr) -> Self {
        let ip: IpAddr = addr.ip().into();
        let addr = net::SocketAddr::from((net::IpAddr::from(ip), addr.port()));
        Self(addr)
    }
}

impl From<SocketAddr> for embedded_nal_async::SocketAddr {
    fn from(addr: SocketAddr) -> Self {
        let ip: IpAddr = addr.0.ip().into();
        embedded_nal_async::SocketAddr::from((embedded_nal_async::IpAddr::from(ip), addr.0.port()))
    }
}
