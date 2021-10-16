/// The vendor ID associated with this implementation of the RTPS.
#[derive(Debug, Clone, Copy, Default)]
pub enum VendorId {
    /// Represents and unknown or unspecified vendor ID
    #[default]
    Unknown,

    /// A specified vendor ID
    Known([u8; 2]),
}
