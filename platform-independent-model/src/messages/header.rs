use super::ProtocolId;
use crate::structure::{ProtocolVersion, VendorId};

mod extension;
pub use extension::Extension;

/// A [`Message`](super::Message) header
#[derive(Debug)]
pub struct Header<P> {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: P,
}
