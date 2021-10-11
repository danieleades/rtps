use crate::{messages::ByteOrder, structure::EntityId};


/// The DataFrag Submessage extends the Data Submessage by enabling the serializedData to be fragmented
/// and sent as multiple DataFrag Submessages. The fragments contained in the DataFrag Submessages are
/// then re-assembled by the RTPSReader.
/// 
/// see [specification pg. 59](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=59)
pub struct Data {
    endianess: ByteOrder,
    inline_qos: Vec<Param>,
    non_standard_payload: bool,
    payload: Payload,
    reader: EntityId,
    writer: EntityId,
    writer_sequence_number: i64,
}

enum Payload {
    Data(Vec<u8>),
    Key(Vec<u8>),
}

pub struct Param;