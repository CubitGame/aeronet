//! See [`Acknowledge`].

use aeronet::octs;
use arbitrary::Arbitrary;

use crate::seq::Seq;

/// Tracks which packets, that we have sent, have been successfully received by
/// the peer (acknowledgements).
///
/// This uses a modification of the strategy described in [*Gaffer On Games*],
/// where we store two pieces of info:
/// * the last received packet sequence number (`last_recv`)
/// * a bitfield of which packets before `last_recv` have been acked
///   (`ack_bits`)
///
/// If a bit at index `N` is set in `ack_bits`, then the packet with sequence
/// `last_recv - N` has been acked. For example,
/// ```text
/// last_recv: 40
///  ack_bits: 0b0000..00001001
///                    ^   ^  ^
///                    |   |  +- seq 40 (40 - 0) has been acked
///                    |   +---- seq 37 (40 - 3) has been acked
///                    +-------- seq 33 (40 - 7) has NOT been acked
/// ```
///
/// This info is sent with every packet, and the last 32 packet acknowledgements
/// are sent, giving a lot of reliability and redundancy for acks.
///
/// [*Gaffer On Games*]: https://gafferongames.com/post/reliable_ordered_messages/#packet-levelacks
#[derive(Debug, Clone, Default, PartialEq, Eq, Arbitrary)]
pub struct Acknowledge {
    /// Last received packet sequence number.
    pub last_recv: Seq,
    /// Bitfield of which packets before `last_recv` have been acknowledged.
    pub ack_bits: u32,
}

impl Acknowledge {
    /// Creates a new value with no packets acknowledged.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a packet sequence as acknowledged.
    ///
    /// # Example
    ///
    /// ```
    /// # use aeronet_proto::{ack::Acknowledge, seq::Seq};
    /// let mut acks = Acknowledge::new();
    /// acks.ack(Seq(0));
    /// assert!(acks.is_acked(Seq(0)));
    /// assert!(!acks.is_acked(Seq(1)));
    ///
    /// acks.ack(Seq(1));
    /// assert!(acks.is_acked(Seq(1)));
    ///
    /// acks.ack(Seq(2));
    /// assert!(acks.is_acked(Seq(2)));
    ///
    /// acks.ack(Seq(5));
    /// assert!(acks.is_acked(Seq(0)));
    /// assert!(acks.is_acked(Seq(1)));
    /// assert!(acks.is_acked(Seq(2)));
    /// assert!(acks.is_acked(Seq(5)));
    ///
    /// // acknowledgement is an idempotent operation
    /// let acks_clone = acks.clone();
    /// acks.ack(Seq(2));
    /// assert_eq!(acks, acks_clone);
    /// ```
    #[allow(clippy::missing_panics_doc)] // won't panic
    pub fn ack(&mut self, seq: Seq) {
        let dist = seq.dist_to(self.last_recv);
        if let Ok(dist) = u32::try_from(dist) {
            // `seq` is before or equal to `last_recv`,
            // so we only set a bit in the bitfield
            self.ack_bits |= shl(1, dist);
        } else {
            // `seq` is after `last_recv`,
            // make that the new `last_recv`
            self.last_recv = seq;
            let shift_by = u32::try_from(-dist).expect(
                "`dist` should be negative, so `-dist` should be positive and within range",
            );
            //    seq: 8
            //    last_recv: 3
            // -> shift_by: 8 - 3 = 5
            //    old recv_bits: 0b00..000000001000
            //                                 ^  ^ seq: 3
            //                                 | seq: 0
            //                                 | shifted `shift_by` (5) places
            //                            v----+
            //    new recv_bits: 0b00..000100000000
            //                            ^
            self.ack_bits = shl(self.ack_bits, shift_by);
            // then also set the `last_recv` in the bitfield
            self.ack_bits |= 1;
        }
    }

    /// Gets if a certain sequence has been marked as acknowledged.
    ///
    /// # Example
    ///
    /// ```
    /// # use aeronet_proto::{ack::Acknowledge, seq::Seq};
    /// let mut acks = Acknowledge::new();
    /// acks.ack(Seq(1));
    /// assert!(acks.is_acked(Seq(1)));
    ///
    /// acks.ack(Seq(2));
    /// assert!(acks.is_acked(Seq(1)));
    /// assert!(acks.is_acked(Seq(2)));
    /// assert!(!acks.is_acked(Seq(3)));
    ///
    /// acks.ack(Seq(50));
    /// assert!(acks.is_acked(Seq(50)));
    /// assert!(!acks.is_acked(Seq(10)));
    /// ```
    #[must_use]
    pub fn is_acked(&self, seq: Seq) -> bool {
        let dist = seq.dist_to(self.last_recv);
        match u32::try_from(dist) {
            Ok(delta) => {
                // `seq` is before or equal to `last_recv`,
                // so we check the bitfield
                self.ack_bits & shl(1, delta) != 0
            }
            Err(_) => {
                // `seq` is after `last_recv`,
                // there's no way it could have been set
                false
            }
        }
    }

    /// Converts this into an iterator over all [`Seq`]s this header contains.
    ///
    /// # Example
    ///
    /// ```
    /// # use aeronet_proto::{seq::Seq, ack::Acknowledge};
    /// let acks = Acknowledge {
    ///     last_recv: Seq(50),
    ///     ack_bits: 0b0010010,
    /// };
    /// let mut iter = acks.seqs();
    /// assert_eq!(Seq(49), iter.next().unwrap());
    /// assert_eq!(Seq(46), iter.next().unwrap());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn seqs(self) -> impl Iterator<Item = Seq> {
        // explicitly don't ack `last_recv` *unless* bit 0 is set
        // we may be in a situation where we literally haven't received any of
        // the last 32 packets, so it'd be invalid to ack the `last_recv`
        (0..32).filter_map(move |bit_index| {
            let packet_seq = self.last_recv - Seq(bit_index);
            if self.ack_bits & shl(1, u32::from(bit_index)) == 0 {
                None
            } else {
                Some(packet_seq)
            }
        })
    }
}

impl octs::ConstEncodeLen for Acknowledge {
    const ENCODE_LEN: usize = Seq::ENCODE_LEN + u32::ENCODE_LEN;
}

impl octs::Encode for Acknowledge {
    fn encode(&self, buf: &mut impl octs::WriteBytes) -> octs::Result<()> {
        buf.write(&self.last_recv)?;
        buf.write(&self.ack_bits)?;
        Ok(())
    }
}

impl octs::Decode for Acknowledge {
    fn decode(buf: &mut impl octs::ReadBytes) -> octs::Result<Self> {
        Ok(Self {
            last_recv: buf.read()?,
            ack_bits: buf.read()?,
        })
    }
}

fn shl(n: u32, by: u32) -> u32 {
    // if None, then `rhs >= 32`
    // so all the bits get moved out anyway
    // so the result ends up just being 0
    n.checked_shl(by).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use aeronet::octs::{ConstEncodeLen, ReadBytes, WriteBytes};

    use super::{shl, *};

    #[test]
    fn shl_in_range() {
        assert_eq!(0b10, shl(0b01, 1));
        assert_eq!(0b1010, shl(0b101, 1));

        assert_eq!(0b10100, shl(0b101, 2));
        assert_eq!(0b10100000, shl(0b101, 5));
    }

    #[test]
    fn shl_out_of_range() {
        assert_eq!(0b0, shl(0b10101, 32));
        assert_eq!(0b0, shl(0b11111, 32));

        assert_eq!(0b0, shl(0b10101, 40));
        assert_eq!(0b0, shl(0b11111, 40));
    }

    #[test]
    fn encode_decode_header() {
        let v = Acknowledge {
            last_recv: Seq(12),
            ack_bits: 0b010101,
        };
        let mut buf = BytesMut::with_capacity(Acknowledge::ENCODE_LEN);

        buf.write(&v).unwrap();
        assert_eq!(Acknowledge::ENCODE_LEN, buf.len());

        assert_eq!(v, buf.freeze().read::<Acknowledge>().unwrap());
    }
}
