use super::{entity::Entity, guid::Guid, locator::Locator};

pub struct Endpoint {
    guid: Guid,
    unicast_locators: Vec<Locator>,
    multicast_locators: Vec<Locator>,
}

impl Endpoint {
    pub fn unicast_locators(&self) -> &Vec<Locator> {
        &self.unicast_locators
    }

    pub fn multicast_locators(&self) -> &Vec<Locator> {
        &self.multicast_locators
    }
}

impl Entity for Endpoint {
    fn guid(&self) -> Guid {
        self.guid
    }
}
