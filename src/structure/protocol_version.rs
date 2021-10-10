
/// A description of the protocol version supported by this implementation
#[derive(Debug, Clone, Copy, Default)]
pub enum ProtocolVersion {

    /// An alias to the latest available version
    #[default]
    Latest,

    /// A specified version
    Specified {
        /// major semantic version of the protocol
        major: u16,

        /// minor semantic version of the protocol
        minor: u16,
    },
}
