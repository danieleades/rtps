use crate::structure::{ProtocolVersion, VendorId};
use vec1::Vec1;

use super::{ProtocolId, SubMessage};

/// General structure of any message within the RTPS protocol
#[derive(Debug)]
pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec1<SubMessage>,
}

#[derive(Debug)]
pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
}

#[derive(Debug)]
pub struct HeaderExtension;
