use std::ops::Neg;

use super::Number;

create_and_implement_proxy_trait! { with name Abs and function abs for types [f32 f64 i8 i16 i32 i64 i128 isize] }
create_and_implement_proxy_trait! { with name Signum and function signum for types [f32 f64 i8 i16 i32 i64 i128 isize] }

pub trait SignedNumber<N = Self>: Number<N> + Abs + Neg<Output = Self> + Signum {}

implement_marker_trait! { SignedNumber, f32 f64 i8 i16 i32 i64 i128 isize }
