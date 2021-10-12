use std::time::SystemTime;

use crate::structure::{ProtocolVersion, VendorId};
use vec1::Vec1;

use super::{ProtocolId, SubMessage};

/// An RTPS [`Message`] consists of a fixed-size leading RTPS [`Header`]
/// followed by a variable number of RTPS [`SubMessage`] parts.
pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec1<SubMessage>,
}

/// The [`Header`] identifies the [`Message`] as belonging to the RTPS protocol.
/// The [`Header`] identifies the version of the protocol and the vendor that
/// sent the [`Message`].
pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: [u8; 12],
}

/// The HeaderExtension may optionally appear immediately following the
/// [`Header`]. It extends the information provided in the [`Header`] without
/// breaking interoperability with earlier versions of the RTPS protocol.
pub struct HeaderExtension {
    submessage: SubMessage,
    timestamp: Option<SystemTime>,
    uextension4: Option<UExtension4>,
    wextension8: Option<WExtension8>,
    checksum: Option<CheckSum>,
    parameters: Option<ParameterList>,
}

// to be defined...
pub type UExtension4 = ();
pub type WExtension8 = ();
pub type CheckSum = ();
pub type ParameterList = ();
