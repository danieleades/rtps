use super::{entity::Entity, guid::Guid};

pub struct Group {
    guid: Guid,
}

impl Entity for Group {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl Group {
    pub(crate) fn new(guid: Guid) -> Self {
        Self { guid }
    }
}
