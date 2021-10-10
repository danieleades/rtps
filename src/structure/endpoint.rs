use super::{entity::Entity, guid::Guid, locator::Locator};

/// An [`Endpoint`] may be a subscriber or a publisher of 'changes'
#[derive(Debug)]
pub struct Endpoint {
    guid: Guid,
    unicast_locators: Vec<Locator>,
    multicast_locators: Vec<Locator>,
}

impl Endpoint {
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

impl Entity for Endpoint {
    fn guid(&self) -> Guid {
        self.guid
    }
}
