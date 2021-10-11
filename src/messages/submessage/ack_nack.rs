use crate::{messages::ByteOrder, structure::EntityId};

pub struct AckNack {
    header: Header,
    reader: EntityId,
    writer: EntityId,
    reader_sequence_number_state: SequenceNumberState,
    count: u32,
}

pub struct Header {
    endianess: ByteOrder,
    response_required: bool,
}

pub struct SequenceNumberState;
