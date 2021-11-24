//! This module implements "auto-splats" for operators.
//! These should be handled with care due to the risks to type inference.
//! We require for every autosplat that:
//! - Simd<T, _> implements the operation for itself, returning Simd<T, _>
//! - T implements the same operation for T, returning T
//!
//! The second bound is not strictly necessary, but enforces symmetry
//! between a vector operation and repeating the scalar equivalent.
use super::*;

macro_rules! autosplat_rhs {
    ($(impl<T, const LANES: usize> $op:ident<$operand:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    })*) => {
        $(impl<T, const LANES: usize> $op<$operand> for Simd<$scalar, LANES>
        where
            Self: $op<Self, Output=Self>,
            $operand: SimdElement + $op<T, Output=T>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            #[must_use = "operator returns a new vector without mutating the inputs"]
            fn $call(self, rhs: $operand) -> Self::Output {
                self.$call(Self::splat(rhs))
            }
        })*
    }
}

autosplat_rhs! {
    // Arithmetic
    impl<T, const LANES: usize> Add<T> for Simd<T, LANES> {
        fn add
    }

    impl<T, const LANES: usize> Mul<T> for Simd<T, LANES> {
        fn mul
    }

    impl<T, const LANES: usize> Sub<T> for Simd<T, LANES> {
        fn sub
    }

    impl<T, const LANES: usize> Div<T> for Simd<T, LANES> {
        fn div
    }

    impl<T, const LANES: usize> Rem<T> for Simd<T, LANES> {
        fn rem
    }

    // Bitwise
    impl<T, const LANES: usize> BitAnd<T> for Simd<T, LANES> {
        fn bitand
    }

    impl<T, const LANES: usize> BitOr<T> for Simd<T, LANES> {
        fn bitor
    }

    impl<T, const LANES: usize> BitXor<T> for Simd<T, LANES> {
        fn bitxor
    }

    impl<T, const LANES: usize> Shl<T> for Simd<T, LANES> {
        fn shl
    }

    impl<T, const LANES: usize> Shr<T> for Simd<T, LANES> {
        fn shr
    }
}
