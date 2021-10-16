use super::ProtocolId;
use crate::structure::{ProtocolVersion, VendorId};

#[derive(Debug)]
pub struct Header<P> {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: P,
}

#[derive(Debug)]
pub struct Extension;
