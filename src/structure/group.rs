//! Types associated with [`Group`]s of [`Endpoint`](super::Endpoint)s

use super::{entity::Entity, guid::Guid, Endpoint};

/// A [`Group`] represents a collection of [`Endpoint`](super::Endpoint)s
///
/// A [`Group`] of 'writers' is a [`Publisher`], while a [`Group`] of 'readers'
/// is a [`Subscriber`].
#[derive(Debug)]
pub struct Group<T> {
    guid: Guid,
    marker: std::marker::PhantomData<T>,
    endpoints: Vec<Endpoint>,
}

impl<T> Entity for Group<T> {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl<T> Group<T> {
    pub(crate) fn new(guid: Guid) -> Self {
        let marker = std::marker::PhantomData;
        let endpoints = Vec::new();
        Self {
            guid,
            marker,
            endpoints,
        }
    }
}

/// Marker type for denoting a subscriber
#[derive(Debug)]
pub struct Reader;

/// Marker type for denoting a publisher
#[derive(Debug)]
pub struct Writer;

/// A [`Group`] of readers
pub type Publisher = Group<Reader>;

/// A [`Group`] of writers
pub type Subscriber = Group<Writer>;
