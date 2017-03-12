use core::num::FpCategory;
use core::{mem, intrinsics, f32, f64};

use approx_eq::ApproxEq;
use signed::Signed;

use libc::{c_float, c_double};

#[link_name = "m"]
extern {
    pub fn cbrtf(n: c_float) -> c_float;
    pub fn expm1f(n: c_float) -> c_float;
    pub fn hypotf(x: c_float, y: c_float) -> c_float;
    pub fn log1pf(n: c_float) -> c_float;

    pub fn cbrt(n: c_double) -> c_double;
    pub fn expm1(n: c_double) -> c_double;
    pub fn hypot(x: c_double, y: c_double) -> c_double;
    pub fn log1p(n: c_double) -> c_double;
}


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
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: &Self) -> Self;
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn ln(&self) -> Self;
    fn log(&self, base: &Self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn hypot(&self, other: &Self) -> Self;
    fn exp_m1(&self) -> Self;
    fn ln_1p(&self) -> Self;
    fn integer_decode(&self) -> (u64, i16, i8);
}


macro_rules! impl_core_float {
    ($T:ident) => (
        #[inline(always)]
        fn nan() -> Self {
            ::core::$T::NAN
        }
        #[inline(always)]
        fn infinity() -> Self {
            ::core::$T::INFINITY
        }
        #[inline(always)]
        fn neg_infinity() -> Self {
            ::core::$T::NEG_INFINITY
        }
        #[inline(always)]
        fn neg_zero() -> Self {
            -0.0
        }
        #[inline(always)]
        fn epsilon() -> Self {
            ::core::$T::EPSILON
        }
        #[inline(always)]
        fn is_nan(&self) -> bool {
             *self != *self
        }
        #[inline(always)]
        fn is_infinite(&self) -> bool {
             *self == ::core::$T::INFINITY || *self == ::core::$T::NEG_INFINITY
        }
        #[inline(always)]
        fn is_finite(&self) -> bool {
            !(self.is_nan() || self.is_infinite())
        }
        #[inline(always)]
        fn is_normal(&self) -> bool {
            self.classify() == FpCategory::Normal
        }
        #[inline(always)]
        fn fract(&self) -> Self {
            *self - self.trunc()
        }
        #[inline(always)]
        fn is_sign_positive(&self) -> bool {
            *self > 0.0 || (1.0 / *self) == ::core::$T::INFINITY
        }
        #[inline(always)]
        fn is_sign_negative(&self) -> bool {
            *self < 0.0 || (1.0 / *self) == ::core::$T::NEG_INFINITY
        }
        #[inline(always)]
        fn recip(&self) -> Self {
            1.0 / *self
        }
        #[inline(always)]
        fn log(&self, base: &Self) -> Self {
            self.ln() / base.ln()
        }
    )
}


impl Float for f32 {
    impl_core_float!(f32);

    #[inline]
    fn classify(&self) -> FpCategory {
        const EXP_MASK: u32 = 0x7f800000;
        const MAN_MASK: u32 = 0x007fffff;

        let bits: u32 = unsafe { mem::transmute(*self) };
        match (bits & MAN_MASK, bits & EXP_MASK) {
            (0, 0) => FpCategory::Zero,
            (_, 0) => FpCategory::Subnormal,
            (0, EXP_MASK) => FpCategory::Infinite,
            (_, EXP_MASK) => FpCategory::Nan,
            _ => FpCategory::Normal,
        }
    }
    #[inline(always)]
    fn trunc(&self) -> Self {
        unsafe {
            intrinsics::truncf32(*self)
        }
    }
    #[inline(always)]
    fn powi(&self, n: i32) -> Self {
         unsafe {
             intrinsics::powif32(*self, n)
         }
    }
    #[inline(always)]
    fn powf(&self, n: &Self) -> Self {
        unsafe {
            intrinsics::powf32(*self, *n)
        }
    }
    #[cfg(target_env = "msvc")]
    #[inline(always)]
    fn exp(&self) -> Self {
        (*self as f64).exp() as f32
    }
    #[cfg(not(target_env = "msvc"))]
    #[inline(always)]
    fn exp(&self) -> Self {
        unsafe {
            intrinsics::expf32(*self)
        }
    }
    #[inline(always)]
    fn exp2(&self) -> Self {
        unsafe {
            intrinsics::exp2f32(*self)
        }
    }
    #[cfg(target_env = "msvc")]
    #[inline(always)]
    fn ln(&self) -> Self {
        (*self as f64).ln() as f32
    }
    #[cfg(not(target_env = "msvc"))]
    #[inline(always)]
    fn ln(&self) -> Self {
        unsafe {
            intrinsics::logf32(*self)
        }
    }
    #[cfg(target_os = "android")]
    #[inline(always)]
    fn log2(&self) -> Self {
        ::sys::android::log2f32(*self)
    }
    #[cfg(not(target_os = "android"))]
    #[inline(always)]
    fn log2(&self) -> Self {
        unsafe {
            intrinsics::log2f32(*self)
        }
    }
    #[cfg(target_env = "msvc")]
    #[inline(always)]
    fn log10(&self) -> Self {
        (*self as f64).log10() as f32
    }
    #[cfg(not(target_env = "msvc"))]
    #[inline(always)]
    fn log10(&self) -> Self {
        unsafe {
            intrinsics::log10f32(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f32.cbrt(), 1.0_f32);
    /// ```
    #[inline(always)]
    fn cbrt(&self) -> Self {
        unsafe {
            cbrtf(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f32.hypot(1.0_f32), 1.4142135_f32);
    /// ```
    #[inline(always)]
    fn hypot(&self, other: &Self) -> Self {
        unsafe {
            hypotf(*self, *other)
        }
    }
    /// ```
    /// assert_eq!(1.0_f32.exp_m1(), 1.7182817_f32);
    /// ```
    #[inline(always)]
    fn exp_m1(&self) -> Self {
        unsafe {
            expm1f(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f32.ln_1p(), 0.6931472_f32);
    /// ```
    #[inline(always)]
    fn ln_1p(&self) -> Self {
        unsafe {
            log1pf(*self)
        }
    }
    #[inline(always)]
    fn integer_decode(&self) -> (u64, i16, i8) {
        // TODO: write f32 specific integer decode
        Float::integer_decode(&(*self as f64))
    }
}

impl Float for f64 {
    impl_core_float!(f64);

    #[inline]
    fn classify(&self) -> FpCategory {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        const MAN_MASK: u64 = 0x000fffffffffffff;

        let bits: u64 = unsafe { mem::transmute(*self) };
        match (bits & MAN_MASK, bits & EXP_MASK) {
            (0, 0) => FpCategory::Zero,
            (_, 0) => FpCategory::Subnormal,
            (0, EXP_MASK) => FpCategory::Infinite,
            (_, EXP_MASK) => FpCategory::Nan,
            _ => FpCategory::Normal,
        }
    }
    #[inline(always)]
    fn trunc(&self) -> Self {
        unsafe {
            intrinsics::truncf64(*self)
        }
    }
    #[inline(always)]
    fn powi(&self, n: i32) -> Self {
         unsafe {
             intrinsics::powif64(*self as f64, n)
         }
    }
    #[inline(always)]
    fn powf(&self, n: &Self) -> Self {
        unsafe {
            intrinsics::powf64(*self as f64, *n)
        }
    }
    #[inline(always)]
    fn exp(&self) -> Self {
        unsafe {
            intrinsics::expf64(*self)
        }
    }
    #[inline(always)]
    fn exp2(&self) -> Self {
        unsafe {
            intrinsics::exp2f64(*self)
        }
    }
    #[inline(always)]
    fn ln(&self) -> Self {
        unsafe {
            intrinsics::logf64(*self)
        }
    }
    #[inline(always)]
    fn log2(&self) -> Self {
        unsafe {
            intrinsics::log2f64(*self)
        }
    }
    #[inline(always)]
    fn log10(&self) -> Self {
        unsafe {
            intrinsics::log10f64(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f64.cbrt(), 1.0_f64);
    /// ```
    #[inline(always)]
    fn cbrt(&self) -> Self {
        unsafe {
            cbrt(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f64.hypot(1.0_f64), 1.4142135623730951_f64);
    /// ```
    #[inline(always)]
    fn hypot(&self, other: &Self) -> Self {
        unsafe {
            hypot(*self, *other)
        }
    }
    /// ```
    /// assert_eq!(1.0_f64.exp_m1(), 1.718281828459045_f64);
    /// ```
    #[inline(always)]
    fn exp_m1(&self) -> Self {
        unsafe {
            expm1(*self)
        }
    }
    /// ```
    /// assert_eq!(1.0_f64.ln_1p(), 0.6931471805599453_f64);
    /// ```
    #[inline(always)]
    fn ln_1p(&self) -> Self {
        unsafe {
            log1p(*self)
        }
    }
    #[inline]
    fn integer_decode(&self) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(self) };
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
