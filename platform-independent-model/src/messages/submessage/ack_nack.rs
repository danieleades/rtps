use crate::messages::ByteOrder;

/// This Submessage is used to communicate the state of a Reader to a Writer.
/// The Submessage allows the Reader to inform the Writer about the sequence
/// numbers it has received and which ones it is still missing. This Submessage
/// can be used to do both positive and negative acknowledgments.
///
/// see [specification pg. 56](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=56)
pub struct AckNack<Id> {
    header: Header,
    reader: Id,
    writer: Id,
    reader_sequence_number_state: SequenceNumberState,
    count: u32,
}

pub struct Header {
    endianess: ByteOrder,
    response_required: bool,
}

pub struct SequenceNumberState;
