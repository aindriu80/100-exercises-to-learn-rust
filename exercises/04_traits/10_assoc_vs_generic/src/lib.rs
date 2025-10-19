// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.
// use super::Power;

pub trait IntoU32 {
    fn into_u32(self) -> u32;
}

// Implement it for the three shapes of exponent used in the tests.
impl IntoU32 for u16 {
    fn into_u32(self) -> u32 {
        self as u32
    }
}
impl IntoU32 for u32 {
    fn into_u32(self) -> u32 {
        self
    }
}
impl<'a> IntoU32 for &'a u32 {
    fn into_u32(self) -> u32 {
        *self
    } // deref the reference
}

// ----- 2. Define the trait that we actually want users to call. ---------
pub trait Power {
    /// Raise `self` to the power of `n`.
    fn power<N>(self, n: N) -> Self
    where
        N: IntoU32;
}

// ----- 3. Implement it for the primitive integer type you need (u32). ---
impl Power for u32 {
    fn power<N>(self, n: N) -> Self
    where
        N: IntoU32,
    {
        // `pow` is defined on all primitive integer types and takes a
        // `u32` exponent.  The helper trait guarantees we can get a `u32`.
        self.pow(n.into_u32())
    }
}

#[cfg(test)]

mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
