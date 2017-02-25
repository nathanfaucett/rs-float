use core::num::FpCategory;
use core::{f32, f64};
use core::mem;

use approx_eq::ApproxEq;
use signed::Signed;


pub trait Float: ApproxEq + Signed {
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn neg_zero() -> Self;
    fn epsilon() -> Self;
    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_normal(&self) -> bool;
    fn classify(&self) -> FpCategory;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
    fn recip(&self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: &Self) -> Self;
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn ln(&self) -> Self;
    fn log(self, base: &Self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn hypot(self, other: &Self) -> Self;
    fn exp_m1(&self) -> Self;
    fn ln_1p(&self) -> Self;
    fn integer_decode(&self) -> (u64, i16, i8);
}


macro_rules! impl_core_float {
    ($T:ident) => (
        fn nan() -> Self {
            ::core::$T::NAN
        }
        fn infinity() -> Self {
            ::core::$T::INFINITY
        }
        fn neg_infinity() -> Self {
            ::core::$T::NEG_INFINITY
        }
        fn neg_zero() -> Self {
            -0.0
        }
        fn epsilon() -> Self {
            ::core::$T::EPSILON
        }
        fn is_nan(&self) -> bool {
             <$T>::is_nan(*self)
        }
        fn is_infinite(&self) -> bool {
             <$T>::is_infinite(*self)
        }
        fn is_finite(&self) -> bool {
            <$T>::is_finite(*self)
        }
        fn is_normal(&self) -> bool {
            <$T>::is_normal(*self)
        }
        fn classify(&self) -> FpCategory {
            <$T>::classify(*self)
        }
        fn trunc(&self) -> Self {
            <$T>::trunc(*self)
        }
        fn fract(&self) -> Self {
            <$T>::fract(*self)
        }
        fn is_sign_positive(&self) -> bool {
            <$T>::is_sign_positive(*self)
        }
        fn is_sign_negative(&self) -> bool {
            <$T>::is_sign_negative(*self)
        }
        fn recip(&self) -> Self {
            <$T>::recip(*self)
        }
        fn powi(self, n: i32) -> Self {
            <$T>::powi(self, n)
        }
        fn powf(self, n: &Self) -> Self {
            <$T>::powf(self, *n)
        }
        fn exp(&self) -> Self {
            <$T>::exp(*self)
        }
        fn exp2(&self) -> Self {
            <$T>::exp2(*self)
        }
        fn ln(&self) -> Self {
            <$T>::ln(*self)
        }
        fn log(self, base: &Self) -> Self {
            <$T>::log(self, *base)
        }
        fn log2(&self) -> Self {
            <$T>::log2(*self)
        }
        fn log10(&self) -> Self {
            <$T>::log10(*self)
        }
        fn cbrt(&self) -> Self {
            <$T>::cbrt(*self)
        }
        fn hypot(self, other: &Self) -> Self {
            <$T>::hypot(self, *other)
        }
        fn exp_m1(&self) -> Self {
            <$T>::exp_m1(*self)
        }
        fn ln_1p(&self) -> Self {
            <$T>::ln_1p(*self)
        }
    )
}


impl Float for f32 {
    impl_core_float!(f32);
    fn integer_decode(&self) -> (u64, i16, i8) {
        // TODO: write f32 specific integer decode
        Float::integer_decode(&(*self as f64))
    }
}

impl Float for f64 {
    impl_core_float!(f64);
    fn integer_decode(&self) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(*self) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }
}
