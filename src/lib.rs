#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod dns;
pub mod tcp;
pub mod udp;

pub struct Stack;

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
