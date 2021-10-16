//! Contains the [`Participant`] and its [`Builder`]

use super::{
    entity::Entity,
    group::{Group, Publisher, Subscriber},
    guid::Guid,
    locator::Locator,
    protocol_version::ProtocolVersion,
    vendor_id::VendorId,
};

/// an [`Entity`] which represents a collection of publishers and subscribers
/// participating in an RTPS network
#[derive(Debug)]
pub struct Participant<P, Id>
where
    P: Copy,
    Id: Copy,
{
    guid: Guid<P, Id>,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    default_unicast_locators: Vec<Locator>,
    default_multicast_locators: Vec<Locator>,
}

/// A builder for a [`Participant`]
///
/// See the [`Participant`] docs for details
#[derive(Debug)]
#[must_use]
pub struct Builder<P, Id>
where
    P: Copy,
    Id: Copy,
{
    guid: Guid<P, Id>,
    protocol_version: Option<ProtocolVersion>,
    vendor_id: Option<VendorId>,
    default_unicast_locators: Vec<Locator>,
    default_multicast_locators: Vec<Locator>,
}

impl<P, Id> Builder<P, Id>
where
    P: Copy,
    Id: Copy,
{
    fn new(guid_prefix: P, entity_id: Id) -> Self {
        let guid = Guid::new(guid_prefix, entity_id);
        let protocol_version = None;
        let vendor_id = None;
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

    /// Set the protocol version of the RTPS participant
    pub fn protocol_version(mut self, major: u16, minor: u16) -> Self {
        self.protocol_version = Some(ProtocolVersion::Specified { major, minor });
        self
    }

    /// Set the vendor ID of the implementor
    ///
    /// See <https://www.dds-foundation.org/dds-rtps-vendor-and-product-ids/> for a list of vendor IDs
    pub fn vendor_id(mut self, id: [u8; 2]) -> Self {
        self.vendor_id = Some(VendorId::Known(id));
        self
    }

    /// If configured, the default unicast locators will be used when creating
    /// new [`Group`]s
    pub fn default_unicast_locators<I, L>(mut self, locators: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Locator>,
    {
        let default_unicast_locators = locators.into_iter().map(Into::into).collect();
        self.default_unicast_locators = default_unicast_locators;
        self
    }

    /// If configured, the default multicast locators will be used when creating
    /// new [`Group`]s
    pub fn default_multicast_locators<I, L>(mut self, locators: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Locator>,
    {
        let default_multicast_locators = locators.into_iter().map(Into::into).collect();
        self.default_multicast_locators = default_multicast_locators;
        self
    }

    /// Consume the [`Builder`] and return a configured [`Participant`]
    #[must_use]
    pub fn build(self) -> Participant<P, Id> {
        Participant {
            guid: self.guid,
            protocol_version: self.protocol_version.unwrap_or_default(),
            vendor_id: self.vendor_id.unwrap_or_default(),
            default_unicast_locators: self.default_unicast_locators,
            default_multicast_locators: self.default_multicast_locators,
        }
    }
}

impl<P, Id> Entity<P, Id> for Participant<P, Id>
where
    P: Copy,
    Id: Copy,
{
    fn guid(&self) -> Guid<P, Id> {
        self.guid
    }
}

impl<P, Id> Participant<P, Id>
where
    P: Copy,
    Id: Copy,
{
    /// Construct a new [`Participant`]
    ///
    /// For additional options, use [`Participant::builder`] instead.
    ///
    /// # Example
    ///
    /// ```
    /// use rtps_pim::structure::Participant;
    ///
    /// let guid_prefix = [0; 12];
    /// let entity_id = [0, 0, 0, 1];
    ///
    /// let participant = Participant::new(guid_prefix, entity_id);
    /// ```
    #[must_use]
    pub fn new(guid_prefix: P, entity_id: Id) -> Self {
        Builder::new(guid_prefix, entity_id).build()
    }

    /// Construct a new [`Participant`] with additional options
    ///
    /// # Example
    ///
    /// ```
    /// use rtps_pim::structure::Participant;
    /// use std::net::{Ipv4Addr, SocketAddrV4};
    ///
    /// let vendor_id = [0x12, 0x12];
    /// let guid_prefix = [0; 12];
    /// let entity_id = [0, 0, 0, 1];
    /// let unicast_locators = vec![SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)];
    ///
    /// let participant = Participant::builder(guid_prefix, entity_id)
    ///     .vendor_id(vendor_id)
    ///     .default_unicast_locators(unicast_locators)
    ///     .build();
    /// ```
    pub fn builder(guid_prefix: P, entity_id: Id) -> Builder<P, Id> {
        Builder::new(guid_prefix, entity_id)
    }

    /// Query the protocol version supported by this domain participant
    #[must_use]
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    /// Query the vendor ID associated with the user of this library
    #[must_use]
    pub fn vendor_id(&self) -> VendorId {
        self.vendor_id
    }

    /// A set of default unicast locators used when constructing new
    /// [`Endpoint`](super::endpoint::Endpoint)s
    #[must_use]
    pub fn default_unicast_locators(&self) -> &Vec<Locator> {
        &self.default_unicast_locators
    }

    /// A set of default multicast locators used when constructing new
    /// [`Endpoint`](super::endpoint::Endpoint)s
    #[must_use]
    pub fn default_multicast_locators(&self) -> &Vec<Locator> {
        &self.default_multicast_locators
    }

    /// Construct a new [`Publisher`] [`Group`] using the [`Guid`] prefix of the
    /// [`Participant`] and a given entity ID
    ///
    /// # Example
    ///
    /// ```
    /// # use rtps_pim::structure::Participant;
    /// #
    /// # let guid_prefix = [0; 12];
    /// # let entity_id = [0, 0, 0, 1];
    /// #
    /// # let participant = Participant::new(guid_prefix, entity_id);
    /// let publisher_id = [0, 0, 0, 2];
    /// let publisher = participant.publisher(publisher_id);
    /// ```
    #[must_use]
    pub fn publisher(&self, entity_id: Id) -> Publisher<P, Id> {
        Group::new(self.guid_prefix(), entity_id)
    }

    /// Construct a new [`Subscriber`] [`Group`] using the [`Guid`] prefix of
    /// the [`Participant`] and a given entity ID
    ///
    /// # Example
    #[must_use]
    pub fn subscriber(&self, entity_id: Id) -> Subscriber<P, Id> {
        Group::new(self.guid_prefix(), entity_id)
    }
}
