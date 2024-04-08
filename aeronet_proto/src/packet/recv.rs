use aeronet::{
    lane::{LaneIndex, LaneMapper},
    message::BytesMapper,
    octs::{BytesError, ReadBytes},
};
use ahash::AHashMap;
use bytes::{Buf, Bytes};

use crate::{
    ack::Acknowledge,
    frag::{Fragment, ReassembleError},
    seq::Seq,
};

use super::{FragmentKey, PacketManager, SentMessage};

#[derive(Debug, thiserror::Error)]
pub enum RecvError<E> {
    #[error("failed to read packet sequence")]
    ReadPacketSeq(#[source] BytesError),
    #[error("failed to read acks")]
    ReadAcks(#[source] BytesError),
    #[error("failed to read fragment")]
    ReadFragment(#[source] BytesError),
    #[error("failed to reassemble message")]
    Reassemble(#[source] ReassembleError),
    #[error("failed to create message from bytes")]
    FromBytes(#[source] E),
    #[error("invalid lane index {lane_index:?}")]
    InvalidLaneIndex { lane_index: LaneIndex },
}

impl<'m, S, R, M: BytesMapper<R> + LaneMapper<R>> PacketManager<'m, S, R, M> {
    /// Reads the [`Acknowledge`] header of a packet, and returns an iterator of
    /// all acknowledged **mesage** sequence numbers.
    ///
    /// # Errors
    ///
    /// Errors if the packet did not contain a valid acknowledge header.
    pub fn read_acks<'a>(
        &'a mut self,
        packet: &'a mut Bytes,
    ) -> Result<impl Iterator<Item = Seq> + 'a, RecvError<<M as BytesMapper<R>>::FromError>> {
        // mark this packet as acked;
        // this ack will later be sent out to the peer in `flush`
        let packet_seq = packet.read::<Seq>().map_err(RecvError::ReadPacketSeq)?;
        self.acks.ack(packet_seq);

        // read packet seqs the peer has reported they've acked..
        // ..turn those into message seqs via our mappings..
        // ..perform our internal bookkeeping..
        // ..and return those message seqs to the caller
        let acks = packet.read::<Acknowledge>().map_err(RecvError::ReadAcks)?;
        let iter =
            Self::packet_to_msg_acks(&self.flushed_packets, &mut self.sent_msgs, acks.seqs());
        Ok(iter)
    }

    fn packet_to_msg_acks<'a>(
        flushed_packets: &'a AHashMap<Seq, Box<[FragmentKey]>>,
        sent_msgs: &'a mut AHashMap<Seq, SentMessage>,
        acked_packet_seqs: impl Iterator<Item = Seq> + 'a,
    ) -> impl Iterator<Item = Seq> + 'a {
        acked_packet_seqs
            .filter_map(|acked_packet_seq| flushed_packets.get(&acked_packet_seq).map(|x| x.iter()))
            .flatten()
            .filter_map(|acked_frag| {
                let msg_seq = acked_frag.msg_seq;
                let unacked_msg = sent_msgs.get_mut(&msg_seq)?;

                // do internal bookkeeping
                if let Some(frag_slot) = unacked_msg
                    .frags
                    .get_mut(usize::from(acked_frag.frag_index))
                {
                    // mark this frag as acked
                    unacked_msg.num_unacked -= 1;
                    *frag_slot = None;
                }

                if unacked_msg.num_unacked == 0 {
                    // message is no longer unacked,
                    // we've just acked all the fragments
                    sent_msgs.remove(&msg_seq);
                    Some(msg_seq)
                } else {
                    None
                }
            })
    }

    /// Reads the next message fragment present in the given packet, and returns
    /// the reassembled message(s) that result from reassembling this fragment.
    ///
    /// This must be called in a loop on the same packet until this returns
    /// `Ok(None)` or `Err`.
    ///
    /// # Errors
    ///
    /// Errors if it could not read the next fragment in the packet.
    pub fn read_next_frag(
        &mut self,
        packet: &mut Bytes,
    ) -> Result<Option<impl Iterator<Item = R> + '_>, RecvError<<M as BytesMapper<R>>::FromError>>
    {
        while packet.has_remaining() {
            let frag = packet
                .read::<Fragment<Bytes>>()
                .map_err(RecvError::ReadFragment)?;

            // reassemble this fragment into a message
            let Some(msg_bytes) = self
                .frag_recv
                .reassemble(&frag.header, &frag.payload)
                .map_err(RecvError::Reassemble)?
            else {
                continue;
            };

            let msg_bytes = Bytes::from(msg_bytes);
            let msg = self
                .mapper
                .try_from_bytes(msg_bytes)
                .map_err(RecvError::FromBytes)?;

            // get what lane this message is received on
            let lane_index = self.mapper.lane_index(&msg);
            let lane = self
                .lanes_recv
                .get_mut(lane_index.into_raw())
                .ok_or(RecvError::InvalidLaneIndex { lane_index })?;

            // ask the lane what messages it wants to give us, in response to
            // receiving this message
            return Ok(Some(lane.recv(frag.header.msg_seq, msg)));
        }
        Ok(None)
    }
}
