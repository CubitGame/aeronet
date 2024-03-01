//! Protocol-level implementation of lanes and associated features such as
//! message acknowledgements and ordering.
//!
//! The most important type to users here is [`Lanes`], which allows processing
//! incoming packets and building up outgoing packets. It handles all the lane
//! guarantees such as reliability and ordering.
//!
//! The API aims to minimize allocations - it will attempt to reuse the user's
//! allocations by using [`Bytes`]:
//! * as input parameters to the API
//! * internally, to store message payloads
//! * as output items for e.g. the outgoing packet iterator
//!
//! # Usage
//!
//! * API user creates [`Lanes`] with a user-defined config
//! * When the user wants to send a message, they call [`buffer_send`] to buffer
//!   up the message for sending
//!   * The message is sent by giving ownership of [`Bytes`], allowing the lane
//!     to reuse the user's allocation
//!   * This will not immediately send the message out, but will buffer it
//! * On app update, the user calls these functions in this specific order:
//!   * [`recv`] to forward transport packets to the lanes
//!   * [`poll`] to update the lanes' internal states
//!   * [`flush`] to forward the lanes' packets to the transport
//! * For each incoming packet from the lower-level transport (i.e. Steamworks,
//!   WebTransport, etc.), the packet is passed to [`recv`]
//!   * The lane processes this packet and returns an iterator over the messages
//!     that it contains - a single packet may contain between 0 or more
//!     messages
//! * [`poll`] is called to update the internal state
//!   * If this returns an error, the connection must be terminated
//! * [`flush`] returns an iterator over all packets to forward to the
//!   lower-level transport
//!   * All packets must be sent down the transport
//!
//! [`buffer_send`]: Lanes::buffer_send
//! [`recv`]: Lanes::recv
//! [`poll`]: Lanes::poll
//! [`flush`]: Lanes::flush
//!
//! # Encoded layout
//!
//! Types:
//! * [`Varint`](octets::varint_len) - a `u64` encoded using between 1 and 10
//!   bytes, depending on the value. Smaller values are encoded more
//!   efficiently.
//! * [`Seq`](crate::Seq)
//! * [`FragmentHeader`](crate::frag::FragmentHeader)
//!
//! ```ignore
//! struct Packet {
//!     /// Sequence number of this packet.
//!     packet_seq: Seq,
//!     /// Acknowledgement response data informing the receiver of this packet
//!     /// which packets this sender has received.
//!     ack_header: AckHeader,
//!     /// Variable-length array of fragments that this packet contains.
//!     frags: [Fragment],
//! }
//!
//! struct Fragment {
//!     /// Which lane this fragment's message should be received on.
//!     lane_id: Varint,
//!     /// Metadata about this fragment.
//!     frag_header: FragHeader,
//!     /// Length of the upcoming payload.
//!     payload_len: Varint,
//!     /// User-defined message payload.
//!     payload: [u8],
//! }
//!
//! struct FragHeader {
//!     /// Sequence number of the message that this fragment belongs to.
//!     msg_seq: Seq,
//!     /// Number of fragments that this message was originally split up into.
//!     num_frags: u8,
//!     /// Index of this fragment.
//!     frag_id: u8,
//! }
//! ```

#[derive(Debug)]
pub struct Lanes {
    lanes: Box<[Lane]>,
}

#[derive(Debug)]
enum Lane {}
