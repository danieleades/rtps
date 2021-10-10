#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Guid {
    prefix: [u8; 12],
    entity_id: [u8; 4],
}

impl Guid {
    pub fn new(prefix: [u8; 12], entity_id: [u8; 4]) -> Self {
        Self { prefix, entity_id }
    }

    pub fn prefix(&self) -> [u8; 12] {
        self.prefix
    }

    pub fn entity_id(&self) -> [u8; 4] {
        self.entity_id
    }
}
