//! See [`PacketHeader`].

use std::convert::Infallible;

use octs::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

use crate::ty::{Acknowledge, PacketHeader, PacketSeq};

impl FixedEncodeLen for PacketHeader {
    const ENCODE_LEN: usize = PacketSeq::ENCODE_LEN + PacketSeq::ENCODE_LEN + u32::ENCODE_LEN;
}

impl Encode for PacketHeader {
    type Error = Infallible;

    fn encode(&self, mut dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        dst.write(&self.seq)?;
        dst.write(&self.acks.last_recv)?;
        dst.write(&self.acks.bits)?;
        Ok(())
    }
}

impl Decode for PacketHeader {
    type Error = Infallible;

    fn decode(mut src: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        Ok(Self {
            seq: src.read()?,
            acks: Acknowledge {
                last_recv: src.read()?,
                bits: src.read()?,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use octs::test::*;

    use crate::ty::Seq;

    use super::*;

    #[test]
    fn encode_decode() {
        hint_round_trip(&PacketHeader {
            seq: PacketSeq::new(0),
            acks: Acknowledge {
                last_recv: PacketSeq::new(0),
                bits: 0,
            },
        });
        hint_round_trip(&PacketHeader {
            seq: PacketSeq(Seq::MAX),
            acks: Acknowledge {
                last_recv: PacketSeq(Seq::MAX),
                bits: u32::MAX,
            },
        });
    }
}
