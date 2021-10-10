#[derive(Debug, Clone, Copy, Default)]
pub enum ProtocolVersion {
    #[default]
    Latest,
    Specified {
        major: u16,
        minor: u16,
    },
}
