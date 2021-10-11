use crate::{messages::ByteOrder, structure::EntityId};

pub struct Data {
    endianess: ByteOrder,
    payload: Payload,
    non_standard_payload: bool,
    reader: EntityId,
    writer: EntityId,
    writer_sequence_number: i64,
    inline_qos: Vec<Param>,
    fragment_starting_number: u32,
    fragments: u32,
    data_size: u64,
}

enum Payload {
    Data(Vec<u8>),
    Key(Vec<u8>),
}

pub struct Param;
