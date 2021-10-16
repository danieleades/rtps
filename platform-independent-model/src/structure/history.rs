//! Objects related to the persistent caching of RTPS messages

use super::guid::Guid;

/// A persisted cache of changes.
///
/// For details, see the [specification, pg. 25](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=25)
///
/// # Example
///
/// ```
///
pub trait Cache<Data> {
    /// The [`Guid`] prefix, shared by all entities in an RTPS participant
    type Prefix: Copy;

    /// The unique ID of an entity
    type EntityId: Copy;

    /// Type used to ascribe a unique, monotonically increasing ID to each [`Change`]
    type SqnN: Ord + Copy;

    /// Error type that may be returned when adding a [`Change`] to the [`Cache`]
    type AddErr: std::error::Error;

    /// Error type that may be returned when removing a [`Change`] from the [`Cache`]
    type RemErr: std::error::Error;

    /// Add a change to the [`HistoryCache`]
    ///
    /// Returns the 'sequence number' of that change.
    /// 
    /// # Errors
    /// 
    /// This method can fail. Specific failure modes will depend on the implementation.
    fn add(change: Change<Data, Self::Prefix, Self::EntityId>) -> Result<Self::SqnN, Self::AddErr>;

    /// Remove a change from the cache, indexed by 'sequence number'
    /// 
    /// # Errors
    /// 
    /// This method can fail. Specific failure modes will depend on the implementation.
    fn remove(
        sequence_number: Self::SqnN,
    ) -> Result<Change<Data, Self::Prefix, Self::EntityId>, Self::RemErr>;

    /// The maximum 'sequence number' stored in the cache
    fn max_sequence_number(&self) -> Self::SqnN;

    /// The minimum 'sequence number' stored in the cache
    fn min_sequence_number(&self) -> Self::SqnN;
}

/// A packet of information representing some change to the state of a data object in the [`Cache`]
#[derive(Debug)]
pub struct Change<Data, P, Id>
where
    P: Copy,
    Id: Copy,
{
    kind: Kind<Data>,
    writer_guid: Guid<P, Id>,
    instance_guid: Guid<P, Id>,
}

/// The type of [`Change`]
#[derive(Debug)]
pub enum Kind<Data> {
    Alive(Data),
    AliveFiltered,
    NotAliveDisposed,
    NotAliveUnregistered,
}
