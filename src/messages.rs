pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec<SubMessage>,
}

pub struct Header;
pub struct HeaderExtension;

use submessage::SubMessage;
mod submessage {
    pub struct SubMessage {
        header: Header,
        elements: Vec<Element>,
    }

    pub struct Header;
    pub struct Element;
}
