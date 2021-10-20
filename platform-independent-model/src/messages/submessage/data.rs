use super::Header;

pub struct Data<Id> {
    header: Header,
    payload: Payload,
    non_standard_payload: bool,
    reader: Id,
    writer: Id,
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
