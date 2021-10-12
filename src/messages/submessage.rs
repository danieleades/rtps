use super::ByteOrder;

pub struct SubMessage {
    header: Header,
    elements: Vec<Element>,
}

impl SubMessage {
    const ENDIANNESS_BIT: usize = 8;

    /// Indicates whether this [`SubMessage`] is the last in the
    /// [`Message`](super::Message).
    pub fn is_last(&self) -> bool {
        // In case submessageLength==0, the Submessage is the last Submessage in the
        // Message and extends up to the end of the Message.
        self.header.len == 0
    }

    /// Endianness of the [`SubMessage`] encoding.
    pub fn endianness(&self) -> ByteOrder {
        // The first flag, the EndiannessFlag, is present and located in the same
        // position in all Submessages and represents the endianness used to encode the
        // information in the Submessage. The literal ‘E’  is often used to refer to the
        // EndiannessFlag.

        // If the EndiannessFlag is set to FALSE, the Submessage is encoded in
        // big-endian format, EndiannessFlag set to TRUE means little-endian.
        let flag_set = (self.header.flags & (1 << Self::ENDIANNESS_BIT)) != 0;
        if flag_set {
            ByteOrder::LittleEndian
        } else {
            ByteOrder::BigEndian
        }
    }
}

pub enum SubMessageKind {
    RtpsHe,
    Data,
    Gap,
    Heartbeat,
    Acknack,
    Pad,
    InfoTs,
    InfoReply,
    InfoDst,
    InfoSrc,
    DataFrag,
    NackFrag,
    HeartbeatFrag,
}

/// Identifies the kind of [`SubMessage`] and the optional elements within that
/// [`SubMessage`]
pub struct Header {
    id: SubMessageKind,
    len: u16,
    flags: u8,
}
pub struct Element;
