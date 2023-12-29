use std::error::Error;

pub trait Message: Send + Sync + 'static {}

pub trait TryAsBytes {
    type Output<'a>: AsRef<[u8]> + 'a
    where
        Self: 'a;

    type Error: Error + Send + Sync + 'static;

    fn try_as_bytes(&self) -> Result<Self::Output<'_>, Self::Error>;
}

pub trait TryFromBytes {
    type Error: Error + Send + Sync + 'static;

    fn try_from_bytes(buf: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageTicket(u16);

impl MessageTicket {
    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageState {
    Unsent,
    Sent,
    Ack,
    Nack,
}
