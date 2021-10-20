use std::convert::TryInto;

use bit_vec::BitVec;
use rtps_pim::messages::{submessage::elements::SequenceNumberSet, ByteOrder};
use safer_bytes::{BufMut, SafeBuf};

use crate::cdr::{FromCdrEndian, IntoCdrEndian};

impl IntoCdrEndian for SequenceNumberSet {
    fn to_buffer_endian<B>(&self, endianess: ByteOrder, mut buffer: B)
    where
        B: BufMut,
    {
        let mut bitmask = BitVec::from_elem((self.max_offset() + 1) as usize, false);

        for offset in self.offsets() {
            bitmask.set(offset as usize, true);
        }

        let blocks = bitmask.blocks();
        let n_bits = blocks.len() * 32;

        match endianess {
            ByteOrder::BigEndian => {
                buffer.put_u64(self.base());
                buffer.put_u32(n_bits.try_into().unwrap());
                for block in blocks {
                    buffer.put_u32(block);
                }
            }
            ByteOrder::LittleEndian => {
                buffer.put_u64_le(self.base());
                buffer.put_u32_le(n_bits.try_into().unwrap());
                for block in blocks {
                    buffer.put_u32_le(block);
                }
            }
        }
    }
}

impl FromCdrEndian for SequenceNumberSet {
    type DecodeErr = DecodeError;

    fn from_bytes_endian<B>(endianess: ByteOrder, mut buffer: B) -> Result<Self, Self::DecodeErr>
    where
        Self: Sized,
        B: SafeBuf,
    {
        let (base, blocks) = match endianess {
            ByteOrder::BigEndian => {
                let base = buffer.try_get_u64()?;
                let n_blocks = buffer.try_get_u32()? / 32;
                let mut blocks = Vec::with_capacity(n_blocks.try_into().unwrap());
                for _ in 0..n_blocks {
                    blocks.push(buffer.try_get_u32()?);
                }

                (base, blocks)
            }
            ByteOrder::LittleEndian => {
                let base = buffer.try_get_u64_le()?;
                let n_blocks = buffer.try_get_u32_le()? / 32;
                let mut blocks = Vec::with_capacity(n_blocks.try_into().unwrap());
                for _ in 0..n_blocks {
                    blocks.push(buffer.try_get_u32_le()?);
                }

                (base, blocks)
            }
        };

        let mut set = Self::new(base.try_into().map_err(|_| DecodeError::ZeroBase)?);

        for (i, block) in blocks.into_iter().enumerate() {
            let offset_base = i * 32;

            for n in 0..32 {
                if block & (1 << n) != 0 {
                    set.insert_offset((offset_base + n).try_into().unwrap());
                }
            }
        }

        Ok(set)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("the base of the sequence cannot be 0")]
    ZeroBase,

    #[error("not enough bytes left in the buffer")]
    Truncated(#[from] safer_bytes::error::Truncated),
}

#[cfg(test)]
mod tests {
    use rtps_pim::messages::{submessage::elements::SequenceNumberSet, ByteOrder};
    use std::num::NonZeroU64;
    use test_case::test_case;

    use crate::cdr::{FromCdrEndian, IntoCdrEndian};

    #[test_case(ByteOrder::BigEndian)]
    #[test_case(ByteOrder::LittleEndian)]
    fn round_trip(endianess: ByteOrder) {
        let mut expected = SequenceNumberSet::new(NonZeroU64::new(100).unwrap());

        expected.insert_offset(1);
        expected.insert_offset(10);
        expected.insert_offset(100);

        let bytes = expected.as_bytes_endian(endianess);

        let actual = SequenceNumberSet::from_bytes_endian(endianess, bytes.as_slice()).unwrap();

        assert_eq!(expected, actual);
    }
}
