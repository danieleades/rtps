/// Trait which represents the ability to convert an object to and from raw
/// bytes in the Common Data Representation (CDR)
// TODO: instead of working with `Vec<u8>` (which will usually force the
// implementor to allocate), perhaps this trait should work with `impl
// AsyncRead`/`impl AsyncWrite` objects instead?
pub trait CDR {
    type DecodeErr: std::error::Error;
    fn as_bytes(&self, endianess: ByteOrder) -> Vec<u8>;
    fn from_bytes(endianess: ByteOrder, bytes: impl AsRef<[u8]>) -> Result<Self, Self::DecodeErr>;
}
