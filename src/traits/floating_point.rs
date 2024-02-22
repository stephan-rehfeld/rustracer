use std::num::FpCategory;

use super::SignedNumber;

create_and_implement_proxy_trait! { with name Acos and function acos and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Acosh and function acosh and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Asin and function asin and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Asinh and function asinh and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Atan and function atan and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Atan2 and function atan2 and different output and one parameter for types [f32 f64] }
create_and_implement_proxy_trait! { with name Atanh and function atanh and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Ceil and function ceil and different output for types  [f32 f64] }
create_and_implement_proxy_trait! { with name Cbrt and function cbrt and different output for types  [f32 f64] }

pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, min: f32, max: f32) -> f32 {
        self.clamp(min, max)
    }
}

impl Clamp for f64 {
    fn clamp(self, min: f64, max: f64) -> f64 {
        self.clamp(min, max)
    }
}

create_and_implement_proxy_trait! { with name Classify and function classify and output FpCategory for types [f32 f64] }
create_and_implement_proxy_trait! { with name Copysign and function copysign and one parameter for types [f32 f64] }
create_and_implement_proxy_trait! { with name Cos and function cos and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Cosh and function cosh and different output for types [f32 f64] }




create_and_implement_proxy_trait! { with name Exp and function exp and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Exp2 and function exp2 and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name ExpM1 and function exp_m1 and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Floor and function floor for types [f32 f64] }
create_and_implement_proxy_trait! { with name Fract and function fract for types [f32 f64] }

pub trait FromBits {
    type Source;

    fn from_bits(v: Self::Source) -> Self;
}

impl FromBits for f32 {
    type Source = u32;

    fn from_bits(v: u32) -> f32 {
        f32::from_bits(v)
    }
}

impl FromBits for f64 {
    type Source = u64;

    fn from_bits(v: u64) -> f64 {
        f64::from_bits(v)
    }
}

create_and_implement_proxy_trait! { with name Hypot and function hypot and one parameter for types [f32 f64] }

create_and_implement_proxy_trait! { with name IsFinite and function is_finite and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsInfinite and function is_infinite and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsNaN and function is_nan and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsNormal and function is_normal and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsSignNegative and function is_sign_negative and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsSignPositive and function is_sign_positive and output bool for types [f32 f64] }
create_and_implement_proxy_trait! { with name IsSubnormal and function is_subnormal and output bool for types [f32 f64] }

// ln
// ln_1p
// ln_gamma
// log
// log10
// log2
// max
// maximum
// min
// mimimum
// mul_add
// next_down
// next_up
// powf
// powi
// recip
// round
// round_ties_even



create_and_implement_proxy_trait! { with name Sin and function sin and different output for types [f32 f64] }

pub trait SinCos {
    type Output;

    fn sin_cos(self) -> Self::Output;
}

impl SinCos for f32 {
    type Output = (f32,f32);

    fn sin_cos(self) -> Self::Output {
        self.sin_cos()
    }
}

impl SinCos for f64 {
    type Output = (f64,f64);

    fn sin_cos(self) -> Self::Output {
        self.sin_cos()
    }
}

create_and_implement_proxy_trait! { with name Sinh and function sinh and different output for types [f32 f64] }


create_and_implement_proxy_trait! { with name Sqrt and function sqrt and different output for types  [f32 f64] }

create_and_implement_proxy_trait! { with name Tan and function tan and different output for types [f32 f64] }
create_and_implement_proxy_trait! { with name Tanh and function tanh and different output for types [f32 f64] }

// to_bits

create_and_implement_proxy_trait! { with name ToDegrees and function to_degrees and different output for types [f32 f64] }

// to_int_unchecked

create_and_implement_proxy_trait! { with name ToRadians and function to_radians and different output for types [f32 f64] }

// total_cmp
// trunc

pub trait FloatingPoint<N=Self>: SignedNumber<N>
        + Acos
        + Acosh
        + Asin
        + Asinh
        + Atan
        + Atan2
        + Cbrt
        + Ceil
        + Clamp
        + Classify
        + Copysign
        + Cos
        + Cosh
        + Exp
        + Exp2
        + ExpM1
        + Floor
        + Fract
        + FromBits
        //+ Gamma
        + Hypot
        + IsFinite
        + IsInfinite
        + IsNaN
        + IsNormal
        + IsSignNegative
        + IsSignPositive
        + IsSubnormal  {
    const DIGITS: u32;
    const EPSILON: Self;
    const INFINITY: Self;
    const MANTISSA_DIGITS: u32;
    const MAX_10_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MIN_EXP: i32;
    const MIN_POSITIVE: Self;
    const NAN: Self;
    const NEG_INFINITY: Self;
    const RADIX: u32;
}

macro_rules! implement_floating_point_trait_for {
    ($($type: ty)*  ) => {
        $(
        impl FloatingPoint for $type  {
            const DIGITS: u32 = <$type>::DIGITS;
            const EPSILON: $type = <$type>::EPSILON;
            const INFINITY: $type = <$type>::INFINITY;
            const MANTISSA_DIGITS: u32 = <$type>::MANTISSA_DIGITS;
            const MAX_10_EXP: i32 = <$type>::MAX_10_EXP;
            const MAX_EXP: i32 = <$type>::MAX_EXP;
            const MIN_10_EXP: i32 = <$type>::MIN_10_EXP;
            const MIN_EXP: i32 = <$type>::MIN_EXP;
            const MIN_POSITIVE: $type = <$type>::MIN_POSITIVE;
            const NAN: $type = <$type>::NAN;
            const NEG_INFINITY: $type = <$type>::NEG_INFINITY;
            const RADIX: u32 = <$type>::RADIX;
        }
        )*
    }
}

implement_floating_point_trait_for! { f32 f64 }

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! implement_test_for {
        ($traitName: ident, $function: ident, $helperName: ident, $testValue: literal, [$($testName: ident),*], [$($type: ty),*] ) => {
            fn $helperName<T: $traitName>(v: T) -> <T as $traitName>::Output {
                v.$function()
            }

            $(
                #[test]
                fn $testName() {
                    assert_eq!($helperName($testValue as $type), ($testValue as $type).$function());
                }
            )*
        };
    }

    implement_test_for! { Acos, acos, acos_test, 1.0, [acos_f32, acos_f64], [f32, f64] }
    implement_test_for! { Acosh, acosh, acosh_test, 1.0, [acosh_f32, acosh_f64], [f32, f64] }
    implement_test_for! { Asin, asin, asin_test, 1.0, [asin_f32, asin_f64], [f32, f64] }
    implement_test_for! { Asinh, asinh, asinh_test, 1.0, [asinh_f32, asinh_f64], [f32, f64] }
    implement_test_for! { Atan, atan, atan_test, 1.0, [atan_f32, atan_f64], [f32, f64] }
    
    fn atan2_helper<T: Atan2>(v: T, other: T) -> <T as Atan2>::Output {
        v.atan2(other)
    }
    
    #[test]
    fn atan2_f32() {
        assert_eq!(atan2_helper(1 as f32, 1 as f32), (1 as f32).atan2(1 as f32));
    }

    #[test]
    fn atan2_f64() {
        assert_eq!(atan2_helper(1 as f64, 1 as f64), (1 as f64).atan2(1 as f64));
    }

    implement_test_for! { Atanh, atanh, atanh_test, 1.0, [atanh_f32, atanh_f64], [f32, f64] }
    implement_test_for! { Cbrt, cbrt, cbrt_test, 16.0, [cbrt_f32, cbrt_f64], [f32, f64] }
    implement_test_for! { Ceil, ceil, ceil_test, 3.1415, [ceil_f32, ceil_f64], [f32, f64] }

    fn clamp_helper<T: Clamp>(v: T, min: T, max: T) -> T {
        v.clamp(min, max)
    }
    
    #[test]
    fn clamp_f32() {
        assert_eq!(clamp_helper(5 as f32, 10 as f32, 20 as f32), (5 as f32).clamp(10 as f32, 20 as f32));
        assert_eq!(clamp_helper(10 as f32, 10 as f32, 20 as f32), (10 as f32).clamp(10 as f32, 20 as f32));
        assert_eq!(clamp_helper(15 as f32, 10 as f32, 20 as f32), (15 as f32).clamp(10 as f32, 20 as f32));
        assert_eq!(clamp_helper(20 as f32, 10 as f32, 20 as f32), (20 as f32).clamp(10 as f32, 20 as f32));
        assert_eq!(clamp_helper(25 as f32, 10 as f32, 20 as f32), (25 as f32).clamp(10 as f32, 20 as f32));
    }

    #[test]
    fn clamp_f64() {
        assert_eq!(clamp_helper(5 as f64, 10 as f64, 20 as f64), (5 as f64).clamp(10 as f64, 20 as f64));
        assert_eq!(clamp_helper(10 as f64, 10 as f64, 20 as f64), (10 as f64).clamp(10 as f64, 20 as f64));
        assert_eq!(clamp_helper(15 as f64, 10 as f64, 20 as f64), (15 as f64).clamp(10 as f64, 20 as f64));
        assert_eq!(clamp_helper(20 as f64, 10 as f64, 20 as f64), (20 as f64).clamp(10 as f64, 20 as f64));
        assert_eq!(clamp_helper(25 as f64, 10 as f64, 20 as f64), (25 as f64).clamp(10 as f64, 20 as f64));
    }

    fn classify_helper<T: Classify>(v: T) -> FpCategory {
        v.classify()
    }
    
    #[test]
    fn classify_f32() {
        assert_eq!(classify_helper(0 as f32), (0 as f32).classify());
        assert_eq!(classify_helper(5 as f32), (5 as f32).classify());
        assert_eq!(classify_helper(f32::NAN), (f32::NAN).classify());
    }

    #[test]
    fn classify_f64() {
        assert_eq!(classify_helper(0 as f64), (0 as f64).classify());
        assert_eq!(classify_helper(5 as f64), (5 as f64).classify());
        assert_eq!(classify_helper(f64::NAN), (f64::NAN).classify());
    }


    fn copysign_helper<T: Copysign>(v: T, rhs: T) -> T {
        v.copysign(rhs)
    }
    
    #[test]
    fn copysign_f32() {
        let f: f32 = 3.5;

        assert_eq!(copysign_helper(f, 0.42), f.copysign(0.42));
        assert_eq!(copysign_helper(f, -0.42), f.copysign(-0.42));
        assert_eq!(copysign_helper(-f, 0.42), (-f).copysign(0.42));
        assert_eq!(copysign_helper(-f, -0.42), (-f).copysign(-0.42));
    }

    #[test]
    fn copysign_f64() {
        let f: f64 = 3.5;

        assert_eq!(copysign_helper(f, 0.42), f.copysign(0.42));
        assert_eq!(copysign_helper(f, -0.42), f.copysign(-0.42));
        assert_eq!(copysign_helper(-f, 0.42), (-f).copysign(0.42));
        assert_eq!(copysign_helper(-f, -0.42), (-f).copysign(-0.42));
    }

    implement_test_for! { Cos, cos, cos_test, 1.0, [cos_f32, cos_f64], [f32, f64] }
    implement_test_for! { Cosh, cosh, cosh_test, 1.0, [cosh_f32, cosh_f64], [f32, f64] }


    implement_test_for! { Exp, exp, exp_test, 3.0, [exp_f32, exp_f64], [f32, f64] }
    implement_test_for! { Exp2, exp2, exp2_test, 3.0, [exp2_f32, exp2_f64], [f32, f64] }
    implement_test_for! { ExpM1, exp_m1, exp_m1_test, 3.0, [exp_m1_f32, exp_m1_f64], [f32, f64] }

    fn floor_helper<T: Floor>(v: T) -> T {
        v.floor()
    }
    
    #[test]
    fn floor_f32() {
        let f: f32 = 3.5;

        assert_eq!(floor_helper(f), f.floor());
    }

    #[test]
    fn floor_f64() {
        let f: f64 = 3.5;

        assert_eq!(floor_helper(f), f.floor());
    }

    fn fract_helper<T: Fract>(v: T) -> T {
        v.fract()
    }
    
    #[test]
    fn fract_f32() {
        let f: f32 = 3.67;

        assert_eq!(fract_helper(f), f.fract());
    }

    #[test]
    fn fract_f64() {
        let f: f64 = 3.67;

        assert_eq!(fract_helper(f), f.fract());
    }

    fn from_bits_helper<T: FromBits>(v: <T as FromBits>::Source) -> T {
        T::from_bits(v)
    }
    
    #[test]
    fn from_bits_f32() {
        let f: u32 = 134412;

        assert_eq!(from_bits_helper::<f32>(f), f32::from_bits(f));
    }

    #[test]
    fn from_bits_f64() {
        let f: u64 = 634634;

        assert_eq!(from_bits_helper::<f64>(f), f64::from_bits(f));
    }


    fn hypot_helper<T: Hypot>(v: T, rhs: T) -> T {
        v.hypot(rhs)
    }
    
    #[test]
    fn hypot_f32() {
        let a: f32 = 2.0;
        let b: f32 = 3.0;

        assert_eq!(hypot_helper(a, b), a.hypot(b));
    }

    #[test]
    fn hypot_f64() {
        let a: f64 = 2.0;
        let b: f64 = 3.0;

        assert_eq!(hypot_helper(a, b), a.hypot(b));
    }



    fn is_finite_helper<T: IsFinite>(v: T) -> bool {
        v.is_finite()
    }

    #[test]
    fn is_finite_f32() {
        let f = 2.0f32;
        let inf = f32::INFINITY;
        let neg_inf = f32::NEG_INFINITY;
        let nan = f32::NAN;

        assert_eq!(is_finite_helper(f), f.is_finite());
        assert_eq!(is_finite_helper(inf), inf.is_finite());
        assert_eq!(is_finite_helper(neg_inf), neg_inf.is_finite());
        assert_eq!(is_finite_helper(nan), nan.is_finite());
    }

    #[test]
    fn is_finite_f64() {
        let f = 2.0f64;
        let inf = f64::INFINITY;
        let neg_inf = f64::NEG_INFINITY;
        let nan = f64::NAN;

        assert_eq!(is_finite_helper(f), f.is_finite());
        assert_eq!(is_finite_helper(inf), inf.is_finite());
        assert_eq!(is_finite_helper(neg_inf), neg_inf.is_finite());
        assert_eq!(is_finite_helper(nan), nan.is_finite());
    }

    fn is_infinite_helper<T: IsInfinite>(v: T) -> bool {
        v.is_infinite()
    }

    #[test]
    fn is_infinite_f32() {
        let f = 2.0f32;
        let inf = f32::INFINITY;
        let neg_inf = f32::NEG_INFINITY;
        let nan = f32::NAN;

        assert_eq!(is_infinite_helper(f), f.is_infinite());
        assert_eq!(is_infinite_helper(inf), inf.is_infinite());
        assert_eq!(is_infinite_helper(neg_inf), neg_inf.is_infinite());
        assert_eq!(is_infinite_helper(nan), nan.is_infinite());
    }

    #[test]
    fn is_infinite_f64() {
        let f = 2.0f64;
        let inf = f64::INFINITY;
        let neg_inf = f64::NEG_INFINITY;
        let nan = f64::NAN;

        assert_eq!(is_infinite_helper(f), f.is_infinite());
        assert_eq!(is_infinite_helper(inf), inf.is_infinite());
        assert_eq!(is_infinite_helper(neg_inf), neg_inf.is_infinite());
        assert_eq!(is_infinite_helper(nan), nan.is_infinite());
    }

    fn is_nan_helper<T: IsNaN>(v: T) -> bool {
        v.is_nan()
    }

    #[test]
    fn is_nan_f32() {
        let f = 2.0f32;
        let nan = f32::NAN;

        assert_eq!(is_nan_helper(f), f.is_nan());
        assert_eq!(is_nan_helper(nan), nan.is_nan());
    }

    #[test]
    fn is_nan_f64() {
        let f = 2.0f64;
        let nan = f64::NAN;

        assert_eq!(is_nan_helper(f), f.is_nan());
        assert_eq!(is_nan_helper(nan), nan.is_nan());
    }

    fn is_normal_helper<T: IsNormal>(v: T) -> bool {
        v.is_normal()
    }

    #[test]
    fn is_normal_f32() {
        let min = f32::MIN_POSITIVE;
        let max = f32::MAX;
        let lower_than_min = 1.0e-40_f32;
        let zero = 0.0_f32;

        assert_eq!(is_normal_helper(min), min.is_normal());
        assert_eq!(is_normal_helper(max), max.is_normal());

        assert_eq!(is_normal_helper(zero), zero.is_normal());
        assert_eq!(is_normal_helper(f32::NAN), f32::NAN.is_normal());
        assert_eq!(is_normal_helper(f32::INFINITY), f32::INFINITY.is_normal());

        assert_eq!(is_normal_helper(lower_than_min), lower_than_min.is_normal());
    }

    #[test]
    fn is_normal_f64() {
        let min = f64::MIN_POSITIVE;
        let max = f64::MAX;
        let lower_than_min = 1.0e-308_f64;
        let zero = 0.0_f64;

        assert_eq!(is_normal_helper(min), min.is_normal());
        assert_eq!(is_normal_helper(max), max.is_normal());

        assert_eq!(is_normal_helper(zero), zero.is_normal());
        assert_eq!(is_normal_helper(f64::NAN), f64::NAN.is_normal());
        assert_eq!(is_normal_helper(f64::INFINITY), f64::INFINITY.is_normal());

        assert_eq!(is_normal_helper(lower_than_min), lower_than_min.is_normal());
    }

    fn is_sign_negative_helper<T: IsSignNegative>(v: T) -> bool {
        v.is_sign_negative()
    }

    #[test]
    fn is_sign_negative_f32() {
        let positive: f32 = 23.24;
        let negative: f32 = -23.24;

        assert_eq!(is_sign_negative_helper(positive), positive.is_sign_negative()); 
        assert_eq!(is_sign_negative_helper(negative), negative.is_sign_negative()); 
    }

    #[test]
    fn is_sign_negative_f64() {
        let positive: f64 = 23.24;
        let negative: f64 = -23.24;

        assert_eq!(is_sign_negative_helper(positive), positive.is_sign_negative()); 
        assert_eq!(is_sign_negative_helper(negative), negative.is_sign_negative()); 
    }

    fn is_sign_positive_helper<T: IsSignPositive>(v: T) -> bool {
        v.is_sign_positive()
    }

    #[test]
    fn is_sign_positive_f32() {
        let positive: f32 = 23.24;
        let negative: f32 = -23.24;

        assert_eq!(is_sign_positive_helper(positive), positive.is_sign_positive()); 
        assert_eq!(is_sign_positive_helper(negative), negative.is_sign_positive()); 
    }

    #[test]
    fn is_sign_positive_f64() {
        let positive: f64 = 23.24;
        let negative: f64 = -23.24;

        assert_eq!(is_sign_positive_helper(positive), positive.is_sign_positive()); 
        assert_eq!(is_sign_positive_helper(negative), negative.is_sign_positive()); 
    }

    fn is_subnormal_helper<T: IsSubnormal>(v: T) -> bool {
        v.is_subnormal()
    }

    #[test]
    fn is_subnormal_f32() {
        let min = f32::MIN_POSITIVE;
        let max = f32::MAX;
        let lower_than_min = 1.0e-40_f32;
        let zero = 0.0_f32;

        assert_eq!(is_subnormal_helper(min), min.is_subnormal());
        assert_eq!(is_subnormal_helper(max), max.is_subnormal());

        assert_eq!(is_subnormal_helper(zero), zero.is_subnormal());
        assert_eq!(is_subnormal_helper(f32::NAN), f32::NAN.is_subnormal());
        assert_eq!(is_subnormal_helper(f32::INFINITY), f32::INFINITY.is_subnormal());

        assert_eq!(is_subnormal_helper(lower_than_min), lower_than_min.is_subnormal());
    }

    #[test]
    fn is_subnormal_f64() {
        let min = f64::MIN_POSITIVE;
        let max = f64::MAX;
        let lower_than_min = 1.0e-308_f64;
        let zero = 0.0_f64;

        assert_eq!(is_subnormal_helper(min), min.is_subnormal());
        assert_eq!(is_subnormal_helper(max), max.is_subnormal());

        assert_eq!(is_subnormal_helper(zero), zero.is_subnormal());
        assert_eq!(is_subnormal_helper(f64::NAN), f64::NAN.is_subnormal());
        assert_eq!(is_subnormal_helper(f64::INFINITY), f64::INFINITY.is_subnormal());

        assert_eq!(is_subnormal_helper(lower_than_min), lower_than_min.is_subnormal());
    }

    implement_test_for! { Sin, sin, sin_test, 1.0, [sin_f32, sin_f64], [f32, f64] }
    implement_test_for! { SinCos, sin_cos, sin_cos_test, 1.0, [sin_cos_f32, sin_cos_f64], [f32, f64] }
    implement_test_for! { Sinh, sinh, sinh_test, 1.0, [sinh_f32, sinh_f64], [f32, f64] }
    implement_test_for! { Sqrt, sqrt, sqrt_test, 2.0, [sqrt_f32, sqrt_f64], [f32, f64] }
    implement_test_for! { Tan, tan, tan_test, 1.0, [tan_f32, tan_f64], [f32, f64] }
    implement_test_for! { Tanh, tanh, tanh_test, 1.0, [tanh_f32, tanh_f64], [f32, f64] }
    implement_test_for! { ToDegrees, to_degrees, to_degrees_test, 3.1415, [to_degrees_f32, to_degrees_f64], [f32, f64] }
    implement_test_for! { ToRadians, to_radians, to_radians_test, 180.0, [to_radians_f32, to_radians_f64], [f32, f64] }
}
