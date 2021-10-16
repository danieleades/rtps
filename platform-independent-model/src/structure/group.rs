//! Types associated with [`Group`]s of [`Endpoint`](super::Endpoint)s

use super::{entity::Entity, guid::Guid};

/// A [`Group`] represents a collection of [`Endpoint`](super::Endpoint)s
///
/// A [`Group`] of 'writers' is a [`Publisher`], while a [`Group`] of 'readers'
/// is a [`Subscriber`].
#[derive(Debug)]
pub struct Group<T, P, Id>
where
    P: Copy,
    Id: Copy,
{
    guid: Guid<P, Id>,
    marker: std::marker::PhantomData<T>,
}

impl<T, P, Id> Entity<P, Id> for Group<T, P, Id>
where
    P: Copy,
    Id: Copy,
{
    fn guid(&self) -> Guid<P, Id> {
        self.guid
    }
}

impl<T, P, Id> Group<T, P, Id>
where
    P: Copy,
    Id: Copy,
{
    pub(crate) fn new(guid_prefix: P, entity_id: Id) -> Self {
        let guid = Guid::new(guid_prefix, entity_id);
        let marker = std::marker::PhantomData;
        Self { guid, marker }
    }
}

/// Marker type for denoting a subscriber
#[derive(Debug)]
pub struct Reader;

/// Marker type for denoting a publisher
#[derive(Debug)]
pub struct Writer;

/// A [`Group`] of readers
pub type Publisher<P, Id> = Group<Reader, P, Id>;

/// A [`Group`] of writers
pub type Subscriber<P, Id> = Group<Writer, P, Id>;
