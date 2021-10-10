pub struct SubMessage {
    header: Header,
    elements: Vec<Element>,
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

pub struct Header;
pub struct Element;
