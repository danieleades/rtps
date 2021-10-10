use vec1::Vec1;
use crate::structure::{ProtocolVersion, VendorId};

use super::{ProtocolId, SubMessage};

pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec1<SubMessage>,
}

pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
}
pub struct HeaderExtension;