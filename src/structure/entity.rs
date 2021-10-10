use super::guid::Guid;

pub trait Entity {
    fn guid(&self) -> Guid;
    fn guid_prefix(&self) -> [u8; 12] {
        self.guid().prefix()
    }
    fn entity_id(&self) -> [u8; 4] {
        self.guid().entity_id()
    }
}
