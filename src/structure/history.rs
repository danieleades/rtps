use super::guid::Guid;

/// A persisted cache of changes.
///
/// For details, see the [specification, pg. 25](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=25)
///
/// # Example
///
/// ```
///
pub trait Cache {
    type SqnN: Ord + Copy;
    type AddErr: std::error::Error;
    type RemErr: std::error::Error;

    /// Add a change to the [`HistoryCache`]
    ///
    /// Returns the 'sequence number' of that change.
    fn add(change: Change) -> Result<Self::SqnN, Self::AddErr>;

    /// Remove a change from the cache, indexed by 'sequence number'
    fn remove(sequence_number: Self::SqnN) -> Result<Change, Self::RemErr>;

    /// The maximum 'sequence number' stored in the cache
    fn max_sequence_number(&self) -> Self::SqnN;

    /// The minimum 'sequence number' stored in the cache
    fn min_sequence_number(&self) -> Self::SqnN;
}

pub struct Change {
    kind: Kind,
    writer_guid: Guid,
    instance_guid: Guid,
}

pub enum Kind {}
