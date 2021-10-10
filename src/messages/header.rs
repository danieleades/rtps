use crate::structure::{ProtocolVersion, VendorId};

pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: [u8 ; 12],
}
pub struct Extension;
struct ProtocolId;
