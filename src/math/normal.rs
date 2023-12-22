use std::ops::{Add, Neg, Mul};

use crate::math::{Vector2, Vector3};
use crate::traits::Zero;

pub trait Normal {
    type ValueType;
}

pub trait Orthonormal2<T> {
    fn x_axis() -> Normal2<T>;
    fn y_axis() -> Normal2<T>;
}

pub trait Orthonormal3<T> {
    fn x_axis() -> Normal3<T>;
    fn y_axis() -> Normal3<T>;
    fn z_axis() -> Normal3<T>;
}

macro_rules! create_normal_type {
    ($name: ident, [$($element: ident)+], $vectorType: ident ) => {
        #[derive(Debug,PartialEq,Clone,Copy)]
        pub struct $name<T> {
            $(
            pub(super) $element: T,
            )*
        }

        impl<T> $name<T> {
            pub fn new( $( $element: T, )*) -> $name<T> {
                $name { $( $element, )* }
            }

            pub fn as_vector(self) -> $vectorType<T> {
                $vectorType::new( $( self.$element, )* )
            }

            pub fn dot<U>(a: $name<T>, b: $name<U>) -> <T as Mul<U>>::Output where
                T: Mul<U> + Copy + Clone,
                <T as Mul<U>>::Output: Add<Output=<T as Mul<U>>::Output> + Zero,
            {
                $(a.$element * b.$element + )* Zero::zero()
            }
        }
        
        impl<T> Normal for $name<T> {
            type ValueType = T;
        }

        impl<T: Neg> Neg for $name<T> {
            type Output = $name<<T as Neg>::Output>;

            fn neg(self) -> Self::Output {
                $name::new( $(-self.$element, )*)
            }
        }

        impl<T, U> Mul<U> for $name<T> where
            T: Mul<U>,
            U: Copy + Clone,
        {
            type Output = $vectorType<<T as Mul<U>>::Output>;

            fn mul(self, rhs: U) -> Self::Output {
                $vectorType::new( $( self.$element * rhs, )* )
            }
        }
    }
}

create_normal_type! { Normal2, [x y], Vector2 }
create_normal_type! { Normal3, [x y z], Vector3 }

macro_rules! impl_orthonormal2_for {
    ($($type: ty)* ) => ($(
        impl Orthonormal2<$type> for Normal2<$type> {
            fn x_axis() -> Normal2<$type> {
                Normal2::new( 1 as $type, 0 as $type )
            }

            fn y_axis() -> Normal2<$type> {
                Normal2::new( 0 as $type, 1 as $type )
            }
        }
    )*)
}

impl_orthonormal2_for! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_orthonormal3_for {
    ($($type: ty)* ) => ($(
        impl Orthonormal3<$type> for Normal3<$type> {
            fn x_axis() -> Normal3<$type> {
                Normal3::new( 1 as $type, 0 as $type, 0 as $type )
            }

            fn y_axis() -> Normal3<$type> {
                Normal3::new( 0 as $type, 1 as $type, 0 as $type )
            }

            fn z_axis() -> Normal3<$type> {
                Normal3::new( 0 as $type, 0 as $type, 1 as $type )
            }
        }
    )*)
}

impl_orthonormal3_for! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_normal2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let norm = Normal2::new( 1 as $type, 2 as $type );

                assert_eq!(norm.x, 1 as $type);
                assert_eq!(norm.y, 2 as $type);
            }
        }
    }

    new_normal2! { u8, new_normal2_u8 }
    new_normal2! { u16, new_normal2_u16 }
    new_normal2! { u32, new_normal2_u32 }
    new_normal2! { u64, new_normal2_u64 }
    new_normal2! { u128, new_normal2_u128 }
    new_normal2! { i8, new_normal2_i8 }
    new_normal2! { i16, new_normal2_i16 }
    new_normal2! { i32, new_normal2_i32 }
    new_normal2! { i64, new_normal2_i64 }
    new_normal2! { i128, new_normal2_i128 }
    new_normal2! { f32, new_normal2_f32 }
    new_normal2! { f64, new_normal2_f64 }

    macro_rules! dot_product_normal2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_norm = Normal2::<$type>::x_axis();
                let y_norm = Normal2::<$type>::y_axis();

                assert_eq!(Normal2::dot(x_norm, y_norm), 0 as $type);
                assert_eq!(Normal2::dot(x_norm, x_norm), 1 as $type);
                assert_eq!(Normal2::dot(y_norm, y_norm), 1 as $type);
            }
        }
    }

    dot_product_normal2! { u8, dot_product_normal2_u8 }
    dot_product_normal2! { u16, dot_product_normal2_u16 }
    dot_product_normal2! { u32, dot_product_normal2_u32 }
    dot_product_normal2! { u64, dot_product_normal2_u64 }
    dot_product_normal2! { u128, dot_product_normal2_u128 }
    dot_product_normal2! { i8, dot_product_normal2_i8 }
    dot_product_normal2! { i16, dot_product_normal2_i16 }
    dot_product_normal2! { i32, dot_product_normal2_i32 }
    dot_product_normal2! { i64, dot_product_normal2_i64 }
    dot_product_normal2! { i128, dot_product_normal2_i128 }
    dot_product_normal2! { f32, dot_product_normal2_f32 }
    dot_product_normal2! { f64, dot_product_normal2_f64 }

    macro_rules! normal2_neg {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n1 = Normal2::new( 1 as $type, 2 as $type);
                let n2 = -n1;

                assert_eq!(n2.x, -1 as $type);
                assert_eq!(n2.y, -2 as $type);
            }
        }
    }

    normal2_neg! { i8, normal2_neg_i8 }
    normal2_neg! { i16, normal2_neg_i16 }
    normal2_neg! { i32, normal2_neg_i32 }
    normal2_neg! { i64, normal2_neg_i64 }
    normal2_neg! { i128, normal2_neg_i128 }
    normal2_neg! { f32, normal2_neg_f32 }
    normal2_neg! { f64, normal2_neg_f64 }

    macro_rules! normal2_mul {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n = Normal2::new(1 as $type, 2 as $type);
                let v = n * (2 as $type);

                assert_eq!(v, Vector2::new(2 as $type, 4 as $type));
            }
        }
    }

    normal2_mul! { i8, normal2_mul_i8 }
    normal2_mul! { i16, normal2_mul_i16 }
    normal2_mul! { i32, normal2_mul_i32 }
    normal2_mul! { i64, normal2_mul_i64 }
    normal2_mul! { i128, normal2_mul_i128 }
    normal2_mul! { f32, normal2_mul_f32 }
    normal2_mul! { f64, normal2_mul_f64 }

    macro_rules! new_normal3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let norm = Normal3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(norm.x, 1 as $type);
                assert_eq!(norm.y, 2 as $type);
                assert_eq!(norm.z, 3 as $type);
            }
        }
    }

    new_normal3! { u8, new_normal3_u8 }
    new_normal3! { u16, new_normal3_u16 }
    new_normal3! { u32, new_normal3_u32 }
    new_normal3! { u64, new_normal3_u64 }
    new_normal3! { u128, new_normal3_u128 }
    new_normal3! { i8, new_normal3_i8 }
    new_normal3! { i16, new_normal3_i16 }
    new_normal3! { i32, new_normal3_i32 }
    new_normal3! { i64, new_normal3_i64 }
    new_normal3! { i128, new_normal3_i128 }
    new_normal3! { f32, new_normal3_f32 }
    new_normal3! { f64, new_normal3_f64 }

    macro_rules! dot_product_normal3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_norm = Normal3::<$type>::x_axis();
                let y_norm = Normal3::<$type>::y_axis();
                let z_norm = Normal3::<$type>::z_axis();

                assert_eq!(Normal3::dot(x_norm, y_norm), 0 as $type);
                assert_eq!(Normal3::dot(x_norm, z_norm), 0 as $type);
                assert_eq!(Normal3::dot(y_norm, z_norm), 0 as $type);

                assert_eq!(Normal3::dot(x_norm, x_norm), 1 as $type);
                assert_eq!(Normal3::dot(y_norm, y_norm), 1 as $type);
                assert_eq!(Normal3::dot(z_norm, z_norm), 1 as $type);
            }
        }
    }

    dot_product_normal3! { u8, dot_product_normal3_u8 }
    dot_product_normal3! { u16, dot_product_normal3_u16 }
    dot_product_normal3! { u32, dot_product_normal3_u32 }
    dot_product_normal3! { u64, dot_product_normal3_u64 }
    dot_product_normal3! { u128, dot_product_normal3_u128 }
    dot_product_normal3! { i8, dot_product_normal3_i8 }
    dot_product_normal3! { i16, dot_product_normal3_i16 }
    dot_product_normal3! { i32, dot_product_normal3_i32 }
    dot_product_normal3! { i64, dot_product_normal3_i64 }
    dot_product_normal3! { i128, dot_product_normal3_i128 }
    dot_product_normal3! { f32, dot_product_normal3_f32 }
    dot_product_normal3! { f64, dot_product_normal3_f64 }

    macro_rules! normal3_neg {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n1 = Normal3::new( 1 as $type, 2 as $type, -3 as $type);
                let n2 = -n1;

                assert_eq!(n2.x, -1 as $type);
                assert_eq!(n2.y, -2 as $type);
                assert_eq!(n2.z, 3 as $type);
            }
        }
    }

    normal3_neg! { i8, normal3_neg_i8 }
    normal3_neg! { i16, normal3_neg_i16 }
    normal3_neg! { i32, normal3_neg_i32 }
    normal3_neg! { i64, normal3_neg_i64 }
    normal3_neg! { i128, normal3_neg_i128 }
    normal3_neg! { f32, normal3_neg_f32 }
    normal3_neg! { f64, normal3_neg_f64 }

    macro_rules! normal3_mul {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n = Normal3::new(1 as $type, 2 as $type, 3 as $type);
                let v = n * (2 as $type);

                assert_eq!(v, Vector3::new(2 as $type, 4 as $type, 6 as $type));
            }
        }
    }

    normal3_mul! { i8, normal3_mul_i8 }
    normal3_mul! { i16, normal3_mul_i16 }
    normal3_mul! { i32, normal3_mul_i32 }
    normal3_mul! { i64, normal3_mul_i64 }
    normal3_mul! { i128, normal3_mul_i128 }
    normal3_mul! { f32, normal3_mul_f32 }
    normal3_mul! { f64, normal3_mul_f64 }
}
