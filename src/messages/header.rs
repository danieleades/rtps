use crate::structure::{GuidPrefix, ProtocolVersion, VendorId};

pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: GuidPrefix,
}
pub struct Extension;
struct ProtocolId;
