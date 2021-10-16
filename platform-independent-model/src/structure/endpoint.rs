use super::{entity::Entity, guid::Guid, locator::Locator};

/// An [`Endpoint`] may be a subscriber or a publisher of 'changes'
#[derive(Debug)]
pub struct Endpoint<P, Id>
where
    P: Copy,
    Id: Copy,
{
    guid: Guid<P, Id>,
    unicast_locators: Vec<Locator>,
    multicast_locators: Vec<Locator>,
}

impl<P, Id> Endpoint<P, Id>
where
    P: Copy,
    Id: Copy,
{
    /// Returns this endpoint's unicast [`Locator`]s
    #[must_use]
    pub fn unicast_locators(&self) -> &Vec<Locator> {
        &self.unicast_locators
    }

    /// Returns this endpoint's multicast [`Locator`]s
    #[must_use]
    pub fn multicast_locators(&self) -> &Vec<Locator> {
        &self.multicast_locators
    }
}

impl<P, Id> Entity<P, Id> for Endpoint<P, Id>
where
    P: Copy,
    Id: Copy,
{
    fn guid(&self) -> Guid<P, Id> {
        self.guid
    }
}
