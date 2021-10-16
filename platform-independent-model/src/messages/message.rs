//! Components of a [`Message`]

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

/// A [`Message`] header
#[derive(Debug)]
pub struct Header {
    protocol_id: ProtocolId,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
}

/// An extension to the [`Message`] header
/// 
/// The [`HeaderExtension`] was added in version 2.5 of the RTPS specification, and is compatible with, but ignored by earlier implementations.
#[derive(Debug)]
pub struct HeaderExtension;
