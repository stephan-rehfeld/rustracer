use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub, SubAssign};

use super::{Normal, Normal2, Normal3, Vector, Vector2, Vector3};

use crate::traits::Zero;

pub trait Point:
    Add<Self::VectorType, Output = Self>
    + Sub<Output = Self::VectorType>
    + Sub<Self::VectorType, Output = Self>
    + Sized
    + Copy
    + Clone
{
    type ValueType: Mul + Copy + Clone + PartialEq + Debug;
    type VectorType: Vector<ValueType = Self::ValueType>;
    type NormalType: Normal<ValueType = Self::ValueType>;
}

macro_rules! create_point_type {
    ($name: ident, [$($element: ident)+], $vectorType: ident, $normalType: ident ) => {
        #[derive(Debug,PartialEq,Clone,Copy)]
        pub struct $name<T> {
            $(
            pub $element: T,
            )*
        }

        impl<T> $name<T> {
            pub fn new( $( $element: T, )*) -> $name<T> {
                $name { $( $element, )* }
            }

            pub fn as_vector( self ) -> $vectorType<T> {
                $vectorType::new( $( self.$element, )* )
            }

        }

        impl<T: Add<Output=T> + Div + Mul + Sub<Output=T> + Copy + Debug + PartialEq> Point for $name<T>
        where
            <T as Mul>::Output: Add<Output=<T as Mul>::Output> + Zero
        {
            type ValueType = T;
            type VectorType = $vectorType<T>;
            type NormalType = $normalType<T>;
        }

        impl<T: Add<U>, U> Add<$vectorType<U>> for $name<T> {
            type Output = $name<<T as Add<U>>::Output>;

            fn add(self, rhs: $vectorType<U>) -> Self::Output {
                 $name::new($(self.$element + rhs.$element, )*)
            }
        }

        impl<T: AddAssign<U>, U> AddAssign<$vectorType<U>> for $name<T> {
            fn add_assign(&mut self, rhs: $vectorType<U>) {
                $(self.$element += rhs.$element;)*
            }
        }

        impl<T: Div<U>, U> Div<U> for $name<T>
        where
            U: Copy + Clone
        {
            type Output = $name<<T as Div<U>>::Output>;

            fn div(self, rhs: U) -> Self::Output {
                $name::new($(self.$element / rhs, )*)
            }
        }

        impl<T: DivAssign<U>, U> DivAssign<U> for $name<T>
        where
            U: Copy + Clone
        {
            fn div_assign(&mut self, rhs: U) {
                $(self.$element /= rhs;)*
            }
        }

        impl<T: Sub<U>, U> Sub<$name<U>> for $name<T> {
            type Output = $vectorType<<T as Sub<U>>::Output>;

            fn sub(self, rhs: $name<U>) -> Self::Output {
                $vectorType::new($(self.$element - rhs.$element, )*)
            }
        }

        impl<T: Sub<U>, U> Sub<$vectorType<U>> for $name<T> {
            type Output = $name<<T as Sub<U>>::Output>;

            fn sub(self, rhs: $vectorType<U>) -> Self::Output {
                $name::new($(self.$element - rhs.$element, )*)
            }
        }

        impl<T: SubAssign<U>, U> SubAssign<$vectorType<U>> for $name<T> {
            fn sub_assign(&mut self, rhs: $vectorType<U>) {
                $(self.$element -= rhs.$element;)*
            }
        }
    }
}

create_point_type! { Point2, [ x y ], Vector2, Normal2 }
create_point_type! { Point3, [ x y z ], Vector3, Normal3 }

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_point2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p = Point2::new(1 as $type, 2 as $type);

                assert_eq!(p.x, 1 as $type);
                assert_eq!(p.y, 2 as $type);
            }
        };
    }

    new_point2! { u8, new_point2_u8 }
    new_point2! { u16, new_point2_u16 }
    new_point2! { u32, new_point2_u32 }
    new_point2! { u64, new_point2_u64 }
    new_point2! { u128, new_point2_u128 }
    new_point2! { i8, new_point2_i8 }
    new_point2! { i16, new_point2_i16 }
    new_point2! { i32, new_point2_i32 }
    new_point2! { i64, new_point2_i64 }
    new_point2! { i128, new_point2_i128 }
    new_point2! { f32, new_point2_f32 }
    new_point2! { f64, new_point2_f64 }

    macro_rules! point2_add_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point2::new(1 as $type, 2 as $type);

                let v1 = Vector2::new(1 as $type, 0 as $type);
                let v2 = Vector2::new(0 as $type, 2 as $type);

                assert_eq!(p1 + v1, Point2::new(2 as $type, 2 as $type));
                assert_eq!(p1 + v2, Point2::new(1 as $type, 4 as $type));
            }
        };
    }

    point2_add_vector2! { u8, point2_add_vector2_u8 }
    point2_add_vector2! { u16, point2_add_vector2_u16 }
    point2_add_vector2! { u32, point2_add_vector2_u32 }
    point2_add_vector2! { u64, point2_add_vector2_u64 }
    point2_add_vector2! { u128, point2_add_vector2_u128 }
    point2_add_vector2! { i8, point2_add_vector2_i8 }
    point2_add_vector2! { i16, point2_add_vector2_i16 }
    point2_add_vector2! { i32, point2_add_vector2_i32 }
    point2_add_vector2! { i64, point2_add_vector2_i64 }
    point2_add_vector2! { i128, point2_add_vector2_i128 }
    point2_add_vector2! { f32, point2_add_vector2_f32 }
    point2_add_vector2! { f64, point2_add_vector2_f64 }

    macro_rules! point2_add_assign_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut p = Point2::new(1 as $type, 2 as $type);

                p += Vector2::new(1 as $type, 2 as $type);

                assert_eq!(p, Point2::new(2 as $type, 4 as $type));
            }
        };
    }

    point2_add_assign_vector2! { u8, point2_add_assign_vector2_u8 }
    point2_add_assign_vector2! { u16, point2_add_assign_vector2_u16 }
    point2_add_assign_vector2! { u32, point2_add_assign_vector2_u32 }
    point2_add_assign_vector2! { u64, point2_add_assign_vector2_u64 }
    point2_add_assign_vector2! { u128, point2_add_assign_vector2_u128 }
    point2_add_assign_vector2! { i8, point2_add_assign_vector2_i8 }
    point2_add_assign_vector2! { i16, point2_add_assign_vector2_i16 }
    point2_add_assign_vector2! { i32, point2_add_assign_vector2_i32 }
    point2_add_assign_vector2! { i64, point2_add_assign_vector2_i64 }
    point2_add_assign_vector2! { i128, point2_add_assign_vector2_i128 }
    point2_add_assign_vector2! { f32, point2_add_assign_vector2_f32 }
    point2_add_assign_vector2! { f64, point2_add_assign_vector2_f64 }

    macro_rules! point2_sub_point2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point2::new(1 as $type, 2 as $type);

                let p2 = Point2::new(1 as $type, 0 as $type);
                let p3 = Point2::new(0 as $type, 2 as $type);

                assert_eq!(p1 - p2, Vector2::new(0 as $type, 2 as $type));
                assert_eq!(p1 - p3, Vector2::new(1 as $type, 0 as $type));
            }
        };
    }

    point2_sub_point2! { u8, point2_sub_point2_u8 }
    point2_sub_point2! { u16, point2_sub_point2_u16 }
    point2_sub_point2! { u32, point2_sub_point2_u32 }
    point2_sub_point2! { u64, point2_sub_point2_u64 }
    point2_sub_point2! { u128, point2_sub_point2_u128 }
    point2_sub_point2! { i8, point2_sub_point2_i8 }
    point2_sub_point2! { i16, point2_sub_point2_i16 }
    point2_sub_point2! { i32, point2_sub_point2_i32 }
    point2_sub_point2! { i64, point2_sub_point2_i64 }
    point2_sub_point2! { i128, point2_sub_point2_i128 }
    point2_sub_point2! { f32, point2_sub_point2_f32 }
    point2_sub_point2! { f64, point2_sub_point2_f64 }

    macro_rules! point2_sub_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point2::new(1 as $type, 2 as $type);

                let v1 = Vector2::new(1 as $type, 0 as $type);
                let v2 = Vector2::new(0 as $type, 2 as $type);

                assert_eq!(p1 - v1, Point2::new(0 as $type, 2 as $type));
                assert_eq!(p1 - v2, Point2::new(1 as $type, 0 as $type));
            }
        };
    }

    point2_sub_vector2! { u8, point2_sub_vector2_u8 }
    point2_sub_vector2! { u16, point2_sub_vector2_u16 }
    point2_sub_vector2! { u32, point2_sub_vector2_u32 }
    point2_sub_vector2! { u64, point2_sub_vector2_u64 }
    point2_sub_vector2! { u128, point2_sub_vector2_u128 }
    point2_sub_vector2! { i8, point2_sub_vector2_i8 }
    point2_sub_vector2! { i16, point2_sub_vector2_i16 }
    point2_sub_vector2! { i32, point2_sub_vector2_i32 }
    point2_sub_vector2! { i64, point2_sub_vector2_i64 }
    point2_sub_vector2! { i128, point2_sub_vector2_i128 }
    point2_sub_vector2! { f32, point2_sub_vector2_f32 }
    point2_sub_vector2! { f64, point2_sub_vector2_f64 }

    macro_rules! point2_sub_assign_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut p = Point2::new(2 as $type, 4 as $type);

                p -= Vector2::new(1 as $type, 2 as $type);

                assert_eq!(p, Point2::new(1 as $type, 2 as $type));
            }
        };
    }

    point2_sub_assign_vector2! { u8, point2_sub_assign_vector2_u8 }
    point2_sub_assign_vector2! { u16, point2_sub_assign_vector2_u16 }
    point2_sub_assign_vector2! { u32, point2_sub_assign_vector2_u32 }
    point2_sub_assign_vector2! { u64, point2_sub_assign_vector2_u64 }
    point2_sub_assign_vector2! { u128, point2_sub_assign_vector2_u128 }
    point2_sub_assign_vector2! { i8, point2_sub_assign_vector2_i8 }
    point2_sub_assign_vector2! { i16, point2_sub_assign_vector2_i16 }
    point2_sub_assign_vector2! { i32, point2_sub_assign_vector2_i32 }
    point2_sub_assign_vector2! { i64, point2_sub_assign_vector2_i64 }
    point2_sub_assign_vector2! { i128, point2_sub_assign_vector2_i128 }
    point2_sub_assign_vector2! { f32, point2_sub_assign_vector2_f32 }
    point2_sub_assign_vector2! { f64, point2_sub_assign_vector2_f64 }

    macro_rules! new_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p = Point3::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(p.x, 1 as $type);
                assert_eq!(p.y, 2 as $type);
                assert_eq!(p.z, 3 as $type);
            }
        };
    }

    new_point3! { u8, new_point3_u8 }
    new_point3! { u16, new_point3_u16 }
    new_point3! { u32, new_point3_u32 }
    new_point3! { u64, new_point3_u64 }
    new_point3! { u128, new_point3_u128 }
    new_point3! { i8, new_point3_i8 }
    new_point3! { i16, new_point3_i16 }
    new_point3! { i32, new_point3_i32 }
    new_point3! { i64, new_point3_i64 }
    new_point3! { i128, new_point3_i128 }
    new_point3! { f32, new_point3_f32 }
    new_point3! { f64, new_point3_f64 }

    macro_rules! point3_add_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point3::new(1 as $type, 2 as $type, 3 as $type);

                let v1 = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let v2 = Vector3::new(0 as $type, 2 as $type, 0 as $type);
                let v3 = Vector3::new(0 as $type, 0 as $type, 3 as $type);

                assert_eq!(p1 + v1, Point3::new(2 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 + v2, Point3::new(1 as $type, 4 as $type, 3 as $type));
                assert_eq!(p1 + v3, Point3::new(1 as $type, 2 as $type, 6 as $type));
            }
        };
    }

    point3_add_vector3! { u8, point3_add_vector3_u8 }
    point3_add_vector3! { u16, point3_add_vector3_u16 }
    point3_add_vector3! { u32, point3_add_vector3_u32 }
    point3_add_vector3! { u64, point3_add_vector3_u64 }
    point3_add_vector3! { u128, point3_add_vector3_u128 }
    point3_add_vector3! { i8, point3_add_vector3_i8 }
    point3_add_vector3! { i16, point3_add_vector3_i16 }
    point3_add_vector3! { i32, point3_add_vector3_i32 }
    point3_add_vector3! { i64, point3_add_vector3_i64 }
    point3_add_vector3! { i128, point3_add_vector3_i128 }
    point3_add_vector3! { f32, point3_add_vector3_f32 }
    point3_add_vector3! { f64, point3_add_vector3_f64 }

    macro_rules! point3_add_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut p = Point3::new(1 as $type, 2 as $type, 3 as $type);

                p += Vector3::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(p, Point3::new(2 as $type, 4 as $type, 6 as $type));
            }
        };
    }

    point3_add_assign_vector3! { u8, point3_add_assign_vector3_u8 }
    point3_add_assign_vector3! { u16, point3_add_assign_vector3_u16 }
    point3_add_assign_vector3! { u32, point3_add_assign_vector3_u32 }
    point3_add_assign_vector3! { u64, point3_add_assign_vector3_u64 }
    point3_add_assign_vector3! { u128, point3_add_assign_vector3_u128 }
    point3_add_assign_vector3! { i8, point3_add_assign_vector3_i8 }
    point3_add_assign_vector3! { i16, point3_add_assign_vector3_i16 }
    point3_add_assign_vector3! { i32, point3_add_assign_vector3_i32 }
    point3_add_assign_vector3! { i64, point3_add_assign_vector3_i64 }
    point3_add_assign_vector3! { i128, point3_add_assign_vector3_i128 }
    point3_add_assign_vector3! { f32, point3_add_assign_vector3_f32 }
    point3_add_assign_vector3! { f64, point3_add_assign_vector3_f64 }

    macro_rules! point3_sub_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point3::new(1 as $type, 2 as $type, 3 as $type);

                let p2 = Point3::new(1 as $type, 0 as $type, 0 as $type);
                let p3 = Point3::new(0 as $type, 2 as $type, 0 as $type);
                let p4 = Point3::new(0 as $type, 0 as $type, 3 as $type);

                assert_eq!(p1 - p2, Vector3::new(0 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 - p3, Vector3::new(1 as $type, 0 as $type, 3 as $type));
                assert_eq!(p1 - p4, Vector3::new(1 as $type, 2 as $type, 0 as $type));
            }
        };
    }

    point3_sub_point3! { u8, point3_sub_point3_u8 }
    point3_sub_point3! { u16, point3_sub_point3_u16 }
    point3_sub_point3! { u32, point3_sub_point3_u32 }
    point3_sub_point3! { u64, point3_sub_point3_u64 }
    point3_sub_point3! { u128, point3_sub_point3_u128 }
    point3_sub_point3! { i8, point3_sub_point3_i8 }
    point3_sub_point3! { i16, point3_sub_point3_i16 }
    point3_sub_point3! { i32, point3_sub_point3_i32 }
    point3_sub_point3! { i64, point3_sub_point3_i64 }
    point3_sub_point3! { i128, point3_sub_point3_i128 }
    point3_sub_point3! { f32, point3_sub_point3_f32 }
    point3_sub_point3! { f64, point3_sub_point3_f64 }

    macro_rules! point3_sub_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point3::new(1 as $type, 2 as $type, 3 as $type);

                let v1 = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let v2 = Vector3::new(0 as $type, 2 as $type, 0 as $type);
                let v3 = Vector3::new(0 as $type, 0 as $type, 3 as $type);

                assert_eq!(p1 - v1, Point3::new(0 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 - v2, Point3::new(1 as $type, 0 as $type, 3 as $type));
                assert_eq!(p1 - v3, Point3::new(1 as $type, 2 as $type, 0 as $type));
            }
        };
    }

    point3_sub_vector3! { u8, point3_sub_vector3_u8 }
    point3_sub_vector3! { u16, point3_sub_vector3_u16 }
    point3_sub_vector3! { u32, point3_sub_vector3_u32 }
    point3_sub_vector3! { u64, point3_sub_vector3_u64 }
    point3_sub_vector3! { u128, point3_sub_vector3_u128 }
    point3_sub_vector3! { i8, point3_sub_vector3_i8 }
    point3_sub_vector3! { i16, point3_sub_vector3_i16 }
    point3_sub_vector3! { i32, point3_sub_vector3_i32 }
    point3_sub_vector3! { i64, point3_sub_vector3_i64 }
    point3_sub_vector3! { i128, point3_sub_vector3_i128 }
    point3_sub_vector3! { f32, point3_sub_vector3_f32 }
    point3_sub_vector3! { f64, point3_sub_vector3_f64 }

    macro_rules! point3_sub_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut p = Point3::new(2 as $type, 4 as $type, 6 as $type);

                p -= Vector3::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(p, Point3::new(1 as $type, 2 as $type, 3 as $type));
            }
        };
    }

    point3_sub_assign_vector3! { u8, point3_sub_assign_vector3_u8 }
    point3_sub_assign_vector3! { u16, point3_sub_assign_vector3_u16 }
    point3_sub_assign_vector3! { u32, point3_sub_assign_vector3_u32 }
    point3_sub_assign_vector3! { u64, point3_sub_assign_vector3_u64 }
    point3_sub_assign_vector3! { u128, point3_sub_assign_vector3_u128 }
    point3_sub_assign_vector3! { i8, point3_sub_assign_vector3_i8 }
    point3_sub_assign_vector3! { i16, point3_sub_assign_vector3_i16 }
    point3_sub_assign_vector3! { i32, point3_sub_assign_vector3_i32 }
    point3_sub_assign_vector3! { i64, point3_sub_assign_vector3_i64 }
    point3_sub_assign_vector3! { i128, point3_sub_assign_vector3_i128 }
    point3_sub_assign_vector3! { f32, point3_sub_assign_vector3_f32 }
    point3_sub_assign_vector3! { f64, point3_sub_assign_vector3_f64 }
}
