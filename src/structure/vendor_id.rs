#[derive(Debug, Clone, Copy, Default)]
pub enum VendorId {
    #[default]
    Unknown,
    Some([u8; 2]),
}
