//! Component elements of a [`SubMessage`](super::SubMessage)

use std::{collections::BTreeSet, convert::TryInto, num::NonZeroU64};

/// [`SequenceNumberSet`] submessage elements are used as parts of several
/// messages to provide binary information about individual sequence numbers
/// within a range.
///
/// The sequence numbers represented in the [`SequenceNumberSet`] are limited
/// to belong to an interval with a range no bigger than 256. This restriction
/// allows a [`SequenceNumberSet`] to be represented in an efficient and compact
/// way using bitmaps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceNumberSet {
    base: NonZeroU64,
    offsets: BTreeSet<u8>,
}

impl SequenceNumberSet {
    /// Create a new [`SequenceNumberSet`]
    /// 
    /// The 'base' of the set is a lower bound for all values in the set. All values in the range are calculated as offsets from the base.
    #[must_use]
    pub fn new(base: NonZeroU64) -> Self {
        let offsets = BTreeSet::default();
        Self { base, offsets }
    }

    /// Returns the 'base' of the set
    ///
    /// Values in the set are stored as a 'base' value, and a set of offsets
    /// from that base.
    #[must_use]
    pub fn base(&self) -> u64 {
        self.base.get()
    }

    /// Return an iterator over the offsets in this set
    pub fn offsets(&self) -> impl Iterator<Item = u8> + '_ {
        self.offsets.iter().copied()
    }

    /// Return an iterator over the values in this set
    pub fn values(&self) -> impl Iterator<Item = u64> + '_ {
        self.offsets().map(move |offset| {
            let offset_u64: u64 = offset.into();
            self.base() + offset_u64
        })
    }

    /// Inserts a new offset into the set.
    ///
    /// `true` is returned if the set did not already contain the value.
    pub fn insert_offset(&mut self, offset: u8) -> bool {
        self.offsets.insert(offset)
    }

    /// Attempt to insert a new value into the set
    ///
    /// # Errors
    ///
    /// - this method will fail if the provided value is smaller than the 'base'
    ///   of the set
    pub fn insert_value(&mut self, value: u64) -> Result<bool, OutOfBoundsError> {
        if value < self.base() {
            return Err(OutOfBoundsError::LessThanBase);
        }

        let offset = (value - self.base())
            .try_into()
            .map_err(|_| OutOfBoundsError::OffsetTooLarge)?;

        Ok(self.insert_offset(offset))
    }

    /// Returns true if the value is contained in the set, or false otherwise
    #[must_use]
    pub fn contains(&self, value: u64) -> bool {
        if value < self.base() {
            return false;
        }

        if let Ok(offset) = (value - self.base()).try_into() {
            self.offsets.contains(&offset)
        } else {
            false
        }
    }

    /// Return the largest offset in the set. Returns 0 if the set is empty
    #[must_use]
    pub fn max_offset(&self) -> u8 {
        self.offsets.iter().next_back().copied().unwrap_or_default()
    }
}

/// Errors that can occur when inserting a new value into a [`SequenceNumberSet`]
#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum OutOfBoundsError {
    /// The provided value is smaller than the base
    /// 
    /// This is not allowed, since all values are stored as a positive offset from the case
    #[error("the provided number is less than the base value of the set")]
    LessThanBase,

    /// The provided value is too far from the base
    /// 
    /// The maximum offset is 255, so N - base <= 255
    #[error("the provided number is too far from the offset")]
    OffsetTooLarge,
}

#[cfg(test)]
mod tests {
    use super::{OutOfBoundsError, SequenceNumberSet};
    use std::convert::TryInto;
    use test_case::test_case;

    #[allow(clippy::bool_assert_comparison)]
    #[test_case(0 => true; "trivial case")]
    fn insert_offset(offset: u8) -> bool {
        let mut set = SequenceNumberSet::new(100.try_into().unwrap());
        set.insert_offset(offset)
    }

    #[test_case(101 => Ok(true); "valid")]
    #[test_case(99 => Err(OutOfBoundsError::LessThanBase); "less than base")]
    #[test_case(1000 => Err(OutOfBoundsError::OffsetTooLarge); "offset too large")]
    fn insert_value(value: u64) -> Result<bool, OutOfBoundsError> {
        let mut set = SequenceNumberSet::new(100.try_into().unwrap());
        set.insert_value(value)
    }
}
