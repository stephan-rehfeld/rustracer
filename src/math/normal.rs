use std::ops;

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
    ($name: ident, [$($element: ident)+] ) => {
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
        }

        impl<T> ops::Mul for $name<T> where
            T: ops::Mul,
            <T as ops::Mul>::Output: ops::Add<Output=<T as ops::Mul>::Output>,
            <T as ops::Mul>::Output: Default,
            T: Copy + Clone
        {
            type Output = <T as ops::Mul>::Output;

            fn mul(self, rhs: $name<T>) -> Self::Output {
                $(self.$element * rhs.$element + )* Default::default()
            }
        }
    }
}

create_normal_type! { Normal2, [x y] }
create_normal_type! { Normal3, [x y z] }

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

                assert_eq!(x_norm * y_norm, 0 as $type);
                assert_eq!(x_norm * x_norm, 1 as $type);
                assert_eq!(y_norm * y_norm, 1 as $type);
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

                assert_eq!(x_norm * y_norm, 0 as $type);
                assert_eq!(x_norm * z_norm, 0 as $type);
                assert_eq!(y_norm * z_norm, 0 as $type);

                assert_eq!(x_norm * x_norm, 1 as $type);
                assert_eq!(y_norm * y_norm, 1 as $type);
                assert_eq!(z_norm * z_norm, 1 as $type);
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
}
