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
    type SqnN: Ord + Copy;
    type AddErr: std::error::Error;
    type RemErr: std::error::Error;

    /// Add a change to the [`HistoryCache`]
    ///
    /// Returns the 'sequence number' of that change.
    fn add(change: Change<Data>) -> Result<Self::SqnN, Self::AddErr>;

    /// Remove a change from the cache, indexed by 'sequence number'
    fn remove(sequence_number: Self::SqnN) -> Result<Change<Data>, Self::RemErr>;

    /// The maximum 'sequence number' stored in the cache
    fn max_sequence_number(&self) -> Self::SqnN;

    /// The minimum 'sequence number' stored in the cache
    fn min_sequence_number(&self) -> Self::SqnN;
}

pub struct Change<Data> {
    kind: Kind<Data>,
    writer_guid: Guid,
    instance_guid: Guid,
}

pub enum Kind<Data> {
    Alive(Data),
    AliveFiltered,
    NotAliveDisposed,
    NotAliveUnregistered,
}
