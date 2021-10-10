#[derive(Debug, Clone, Copy)]
pub enum ProtocolVersion {
    Latest,
    Specified { major: u16, minor: u16 },
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::Latest
    }
}
