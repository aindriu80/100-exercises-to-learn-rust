// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
// src/lib.rs

/// A wrapper around `u16` that saturates on addition.
///
/// All public items are exported because the tests live in a separate crate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SaturatingU16(pub u16);

impl From<u16> for SaturatingU16 {
    fn from(v: u16) -> Self {
        SaturatingU16(v)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(v: &u16) -> Self {
        SaturatingU16(*v)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(v: u8) -> Self {
        SaturatingU16(v as u16)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(v: &u8) -> Self {
        SaturatingU16(*v as u16)
    }
}

// ------------------------------------------------------------
// Helper – perform a saturating addition on the inner value
// ------------------------------------------------------------
impl SaturatingU16 {
    #[inline]
    pub const fn saturating_add_u16(self, rhs: u16) -> SaturatingU16 {
        SaturatingU16(self.0.saturating_add(rhs))
    }
}

// --------------------------------------
// Addition – all combinations used in tests
// --------------------------------------
use std::ops::Add;

// 1. Self + Self
impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        self.saturating_add_u16(rhs.0)
    }
}

// 2. &Self + Self
impl<'a> Add<SaturatingU16> for &'a SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        self.saturating_add_u16(rhs.0)
    }
}

// 3. Self + &Self
impl<'a> Add<&'a SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &'a SaturatingU16) -> SaturatingU16 {
        self.saturating_add_u16(rhs.0)
    }
}

// 4. &Self + &Self
impl<'a, 'b> Add<&'b SaturatingU16> for &'a SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &'b SaturatingU16) -> SaturatingU16 {
        self.saturating_add_u16(rhs.0)
    }
}

// 5. Self + u16
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> SaturatingU16 {
        self.saturating_add_u16(rhs)
    }
}

// 6. &Self + u16
impl<'a> Add<u16> for &'a SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> SaturatingU16 {
        self.saturating_add_u16(rhs)
    }
}

// 7. Self + &u16
impl<'a> Add<&'a u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &'a u16) -> SaturatingU16 {
        self.saturating_add_u16(*rhs)
    }
}

// 8. &Self + &u16
impl<'a, 'b> Add<&'b u16> for &'a SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &'b u16) -> SaturatingU16 {
        self.saturating_add_u16(*rhs)
    }
}

// 9. u16 + Self
impl Add<SaturatingU16> for u16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        SaturatingU16(self.saturating_add(rhs.0))
    }
}

// --------------------------------------
// Comparisons – allow `==` with a plain `u16`
// --------------------------------------
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.0
    }
}
