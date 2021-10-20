use rtps_pim::messages::ByteOrder;
use safer_bytes::{BufMut, SafeBuf};

/// Trait which represents the ability to convert an object to and from raw
/// bytes in the Common Data Representation (CDR)
// TODO: instead of working with `Vec<u8>` (which will usually force the
// implementor to allocate), perhaps this trait should work with `impl
// AsyncRead`/`impl AsyncWrite` objects instead?
pub(crate) trait Cdr: FromCdr + IntoCdr {}

impl<T> Cdr for T where T: FromCdr + IntoCdr {}

#[allow(clippy::module_name_repetitions)]
pub(crate) trait FromCdr {
    type DecodeErr: std::error::Error;
    fn from_bytes<B>(bytes: B) -> Result<Self, Self::DecodeErr>
    where
        Self: Sized,
        B: SafeBuf;
}

#[allow(clippy::module_name_repetitions)]
pub(crate) trait IntoCdr {
    fn to_buffer<B>(&self, buffer: B)
    where
        B: BufMut;

    fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::default();
        self.to_buffer(&mut buffer);
        buffer
    }
}

pub(crate) trait FromCdrEndian {
    type DecodeErr: std::error::Error;
    fn from_bytes_endian<B>(endianess: ByteOrder, bytes: B) -> Result<Self, Self::DecodeErr>
    where
        Self: Sized,
        B: SafeBuf;

    fn from_bytes_be<B>(bytes: B) -> Result<Self, Self::DecodeErr>
    where
        Self: Sized,
        B: SafeBuf,
    {
        Self::from_bytes_endian(ByteOrder::BigEndian, bytes)
    }

    fn from_bytes_le<B>(bytes: B) -> Result<Self, Self::DecodeErr>
    where
        Self: Sized,
        B: SafeBuf,
    {
        Self::from_bytes_endian(ByteOrder::LittleEndian, bytes)
    }
}

pub(crate) trait IntoCdrEndian {
    fn to_buffer_endian<B>(&self, endianess: ByteOrder, buffer: B)
    where
        B: BufMut;

    fn to_buffer_le<B>(&self, buffer: B)
    where
        B: BufMut,
    {
        self.to_buffer_endian(ByteOrder::LittleEndian, buffer);
    }

    fn to_buffer_be<B>(&self, buffer: B)
    where
        B: BufMut,
    {
        self.to_buffer_endian(ByteOrder::BigEndian, buffer);
    }

    fn as_bytes_endian(&self, endianess: ByteOrder) -> Vec<u8> {
        let mut buffer = Vec::default();
        self.to_buffer_endian(endianess, &mut buffer);
        buffer
    }

    fn as_bytes_be(&self) -> Vec<u8> {
        let mut buffer = Vec::default();
        self.to_buffer_endian(ByteOrder::BigEndian, &mut buffer);
        buffer
    }

    fn as_bytes_le(&self) -> Vec<u8> {
        let mut buffer = Vec::default();
        self.to_buffer_endian(ByteOrder::LittleEndian, &mut buffer);
        buffer
    }
}
