/// A unique identifier of an entity/actor within the RTPS protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Guid {
    prefix: Prefix,
    entity_id: EntityId,
}

/// The prefix component of a [`Guid`]
pub type Prefix = [u8; 12];

/// The entity ID component of a [`Guid`]
pub type EntityId = [u8; 4];

impl Guid {
    /// Construct a new [`Guid`] from a prefix and an entity ID
    ///
    /// # Example
    ///
    /// ```
    /// use rtps::structure::Guid;
    ///
    /// let guid_prefix = [0; 12];
    /// let entity_id = [0, 0, 0, 1];
    ///
    /// let guid = Guid::new(guid_prefix, entity_id);
    ///
    /// assert_eq!(guid.prefix(), guid_prefix);
    /// assert_eq!(guid.entity_id(), entity_id);
    /// ```
    #[must_use]
    pub fn new(prefix: Prefix, entity_id: EntityId) -> Self {
        Self { prefix, entity_id }
    }

    /// Return the [`Guid`] prefix
    #[must_use]
    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    /// Return the entity ID
    #[must_use]
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
