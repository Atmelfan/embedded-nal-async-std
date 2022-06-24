#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
//! This crate implements [embedded-nal-async](https://crates.io/crates/embedded-nal-async) for platforms supported by async-std.
//! It is basically the async version of [std-embedded-nal](https://crates.io/crates/std-embedded-nal) which implements [embedded-nal](https://crates.io/crates/embedded-nal) traits for std platforms.
//!
//! **Note:** This crate is just as experimental as [embedded-nal-async](https://crates.io/crates/embedded-nal-async) is. Breaking changes are expected.
//!

pub(crate) mod conversions;
pub(crate) mod dns;
pub(crate) mod tcp;
pub(crate) mod udp;

use async_std::net;

/// Async-std stack for embedded-nal
/// Doesn't actually do or contain anything
#[derive(Clone)]
pub struct Stack {
    ip: net::IpAddr,
}

impl Stack {
    pub fn new(ip: net::IpAddr) -> Self {
        Self { ip }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            ip: net::Ipv4Addr::UNSPECIFIED.into(),
        }
    }
}

/// State of a TCP/UDP socket
pub(crate) enum SocketState<C, B> {
    Closed,
    Connected(C),
    Bound(B),
}

impl<C, B> SocketState<C, B> {
    pub(crate) fn get_connected(&mut self) -> async_std::io::Result<&mut C> {
        match self {
            SocketState::Connected(c) => Ok(c),
            _ => Err(async_std::io::ErrorKind::NotConnected.into()),
        }
    }

    pub(crate) fn get_bound(&mut self) -> async_std::io::Result<&mut B> {
        match self {
            SocketState::Bound(b) => Ok(b),
            _ => Err(async_std::io::ErrorKind::NotConnected.into()),
        }
    }
}

impl<T> SocketState<T, T> {
    pub(crate) fn get_either(&mut self) -> async_std::io::Result<&mut T> {
        match self {
            SocketState::Connected(t) => Ok(t),
            SocketState::Bound(t) => Ok(t),
            _ => Err(async_std::io::ErrorKind::NotConnected.into()),
        }
    }
}
