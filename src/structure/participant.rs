use super::{
    entity::Entity, group::Group, guid::Guid, locator::Locator, protocol_version::ProtocolVersion,
    vendor_id::VendorId,
};

pub struct Participant {
    guid: Guid,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    default_unicast_locators: Vec<Locator>,
    default_multicast_locators: Vec<Locator>,
}

/// A builder for a [`Participant`]
///
/// See the [`Participant`] docs for details
pub struct Builder {
    guid: Guid,
    protocol_version: Option<ProtocolVersion>,
    vendor_id: VendorId,
    default_unicast_locators: Vec<Locator>,
    default_multicast_locators: Vec<Locator>,
}

impl Builder {
    fn new(vendor_id: VendorId, guid: Guid) -> Self {
        let protocol_version = None;
        let default_unicast_locators = Vec::default();
        let default_multicast_locators = Vec::default();
        Self {
            guid,
            protocol_version,
            vendor_id,
            default_unicast_locators,
            default_multicast_locators,
        }
    }

    fn protocol_version(mut self, version: ProtocolVersion) -> Self {
        self.protocol_version = Some(version);
        self
    }

    fn default_unicast_locators<I, L>(mut self, locators: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Locator>,
    {
        let default_unicast_locators = locators.into_iter().map(Into::into).collect();
        self.default_unicast_locators = default_unicast_locators;
        self
    }

    fn default_multicast_locators<I, L>(mut self, locators: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Locator>,
    {
        let default_multicast_locators = locators.into_iter().map(Into::into).collect();
        self.default_multicast_locators = default_multicast_locators;
        self
    }

    fn build(self) -> Participant {
        Participant {
            guid: self.guid,
            protocol_version: self.protocol_version.unwrap_or_default(),
            vendor_id: self.vendor_id,
            default_unicast_locators: self.default_unicast_locators,
            default_multicast_locators: self.default_multicast_locators,
        }
    }
}

impl Entity for Participant {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl Participant {
    /// Construct a new [`Participant`]
    ///
    /// For additional options, use [`Participant::builder`] instead.
    pub fn new(vendor_id: VendorId, guid: Guid) -> Self {
        Builder::new(vendor_id, guid).build()
    }

    pub fn builder(vendor_id: VendorId, guid: Guid) -> Builder {
        Builder::new(vendor_id, guid)
    }

    /// Query the protocol version supported by this domain participant
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    /// Query the vendor ID associated with the user of this library
    pub fn vendor_id(&self) -> VendorId {
        self.vendor_id
    }

    /// A set of default unicast locators used when constructing new
    /// [`Endpoint`](super::endpoint::Endpoint)s
    pub fn default_unicast_locators(&self) -> &Vec<Locator> {
        &self.default_unicast_locators
    }

    /// A set of default multicast locators used when constructing new
    /// [`Endpoint`](super::endpoint::Endpoint)s
    pub fn default_multicast_locators(&self) -> &Vec<Locator> {
        &self.default_multicast_locators
    }

    /// Construct a new [`Group`] using the [`Guid`] prefix of the
    /// [`Participant`] and a given entity ID
    ///
    /// # Example
    ///
    pub fn group(&self, entity_id: [u8; 4]) -> Group {
        let guid = Guid::new(self.guid_prefix(), entity_id);
        Group::new(guid)
    }
}
