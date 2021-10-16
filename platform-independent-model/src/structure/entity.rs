use super::Guid;

/// Trait implemented by every actor in the protocol 'structure'
pub trait Entity<P, Id>
where
    P: Copy,
    Id: Copy,
{
    /// Return the [`Guid`] associated with this entity
    fn guid(&self) -> Guid<P, Id>;

    /// Return the [`Guid`] prefix associated with this entity
    ///
    /// The prefix is shared between all entities within a
    /// [`Participant`](super::Participant)
    fn guid_prefix(&self) -> P {
        self.guid().prefix()
    }

    /// Return the entity ID associated with this entity
    ///
    /// The entity ID is unique within a [`Participant`](super::Participant)
    fn entity_id(&self) -> Id {
        self.guid().entity_id()
    }
}
