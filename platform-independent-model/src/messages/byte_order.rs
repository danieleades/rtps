/// A marker object representing the byte order of encoded data
// todo: it's very likely that a dependency added in the near will export
// something like this, and then this can be removed
#[derive(Debug)]
pub enum ByteOrder {
    /// "Big Endian" byte order
    BigEndian,

    /// "Little Endian" byte order
    LittleEndian,
}
