use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{Normal2, Normal3, Orthonormal2, Orthonormal3, Point2, Point3};

use crate::traits::{Half, Sqrt, Zero};

pub trait Vector: Copy {
    const DIMS: u32;
    type ValueType: Mul;
    type PointType;
    type NormalType;

    fn dot(self, rhs: Self) -> <Self::ValueType as Mul>::Output;
}

macro_rules! create_vector_type {
    ($name: ident, [$($element: ident)+], $dims: literal, $pointType: ident, $normalType: ident) => {
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

            pub fn as_normal(self) -> $normalType<T> {
                $normalType::new( $( self.$element, )* )
            }

            pub fn as_point(self) -> $pointType<T> {
                $pointType::new( $( self.$element, )* )
            }
        }

        impl<T> $name<T> {
            pub fn dot<U>(self, v: $name<U>) -> <T as Mul<U>>::Output
                where
                            T: Mul<U>,
                            <T as Mul<U>>::Output: Add<Output=<T as Mul<U>>::Output> + Zero {
                $(self.$element * v.$element + )* Zero::zero()
            }
        }

        impl<T: Div + Mul + Copy> Vector for $name<T> where
            <T as Mul>::Output: Add<Output=<T as Mul>::Output> + Zero
        {
            const DIMS: u32 = $dims;

            type ValueType = T;
            type PointType = $pointType<T>;
            type NormalType = $normalType<T>;

            fn dot(self, v: Self) -> <T as Mul>::Output {
                $(self.$element * v.$element + )* Zero::zero()
            }
        }

        impl<T> $name<T>
        where
            T: Mul + Copy,
            <T as Mul>::Output: Add<Output=<T as Mul>::Output> + Sqrt<Output=T> + Zero
        {
            pub fn magnitude(self) -> T {
                self.dot(self).sqrt()
            }
        }

        impl<T> $name<T>
        where
            T: Div + Mul + Copy,
            <T as Mul>::Output: Add<Output=<T as Mul>::Output> + Sqrt<Output=T> + Zero
        {
            pub fn normalized(self) -> $name<<T as Div>::Output> {
                self / self.magnitude()
            }
        }


        impl<T> $name<T> where
            T: Div + Mul<<T as Div>::Output, Output=T> + Add<Output=T> + Sub<T, Output=T> + Zero + Copy,
            <T as Div>::Output: Mul<T, Output=T> + Copy + Clone,
        {
            pub fn reflect_on(self, n: $normalType<<T as Div>::Output>) -> Self {
                let dot_product = self.dot(n.as_vector());
                let dot_product_times_two = dot_product + dot_product;
                self - n * dot_product_times_two
            }
        }

        impl<T: Add<U> , U> Add<$name<U>> for $name<T> {
            type Output = $name<<T as Add<U>>::Output>;

            fn add(self, rhs: $name<U>) -> Self::Output {
                $name::new( $( self.$element + rhs.$element, )* )
            }
        }

        impl<T: Add<U> , U> Add<$pointType<U>> for $name<T> {
            type Output = $pointType<<T as Add<U>>::Output>;

            fn add(self, rhs: $pointType<U>) -> Self::Output {
                $pointType::new( $( self.$element + rhs.$element, )* )
            }
        }

        impl<T: AddAssign<U> , U> AddAssign<$name<U>> for $name<T> {
            fn add_assign(&mut self, rhs: $name<U>) {
                $( self.$element += rhs.$element; )*
            }
        }

        impl<T: Sub<U>, U> Sub<$name<U>> for $name<T> {
            type Output = $name<<T as Sub<U>>::Output>;

            fn sub(self, rhs: $name<U>) -> Self::Output {
                $name::new( $( self.$element - rhs.$element, )* )
            }
        }

        impl<T: SubAssign<U>, U> SubAssign<$name<U>> for $name<T> {
            fn sub_assign(&mut self, rhs: $name<U>) {
                $( self.$element -= rhs.$element; )*
            }
        }

        impl<T: Mul<U>, U: Copy + Clone> Mul<U> for $name<T> {
            type Output = $name<<T as Mul<U>>::Output>;

            fn mul(self, rhs: U) -> Self::Output {
                $name::new( $( self.$element * rhs, )* )
            }
        }

        impl<T: MulAssign<U>, U: Copy + Clone> MulAssign<U> for $name<T> {
            fn mul_assign(&mut self, rhs: U) {
                $( self.$element *= rhs; )*
            }
        }

        impl<T: Div<U>, U: Copy + Clone> Div<U> for $name<T> {
            type Output = $name<<T as Div<U>>::Output>;

            fn div(self, rhs: U) -> Self::Output {
                $name::new($(self.$element / rhs, )*)
            }
        }

        impl<T: DivAssign<U>, U: Copy + Clone> DivAssign<U> for $name<T> {
            fn div_assign(&mut self, rhs: U) {
                $( self.$element /= rhs; )*
            }
        }

        impl<T: Neg> Neg for $name<T> {
            type Output = $name<<T as Neg>::Output>;

            fn neg(self) -> Self::Output {
                $name::new( $(-self.$element, )*)
            }
        }

        impl<T: Half> Half for $name<T> {
            fn half(&self) -> $name<T> {
                $name::new( $(self.$element.half(), )*)
            }
        }
    }
}

create_vector_type! { Vector2, [x y], 2, Point2, Normal2 }
create_vector_type! { Vector3, [x y z], 3, Point3, Normal3 }

impl<T> Vector3<T> {
    pub fn cross<U>(a: Vector3<T>, b: Vector3<U>) -> Vector3<<T as Mul<U>>::Output>
    where
        T: Mul<U> + Copy + Clone,
        <T as Mul<U>>::Output: Sub<Output = <T as Mul<U>>::Output>,
        U: Copy,
    {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

macro_rules! impl_orthonormal2_for {
    ($($type: ty)* ) => ($(
        impl Orthonormal2 for Vector2<$type> {
            fn x_axis() -> Vector2<$type> {
                Vector2::new( 1 as $type, 0 as $type )
            }

            fn y_axis() -> Vector2<$type> {
                Vector2::new( 0 as $type, 1 as $type )
            }
        }
    )*)
}

impl_orthonormal2_for! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_orthonormal3_for {
    ($($type: ty)* ) => ($(
        impl Orthonormal3 for Vector3<$type> {
            fn x_axis() -> Vector3<$type> {
                Vector3::new( 1 as $type, 0 as $type, 0 as $type )
            }

            fn y_axis() -> Vector3<$type> {
                Vector3::new( 0 as $type, 1 as $type, 0 as $type )
            }

            fn z_axis() -> Vector3<$type> {
                Vector3::new( 0 as $type, 0 as $type, 1 as $type )
            }
        }
    )*)
}

impl_orthonormal3_for! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_mul_scalar_with_vector2 {
    ($($type: ty)+ ) => ($(
        impl<T> Mul<Vector2<T>> for $type where
            $type: Mul<T>,
        {
            type Output = Vector2<<$type as Mul<T>>::Output>;

            fn mul(self, rhs: Vector2<T>) -> Self::Output {
                Vector2::new( self * rhs.x, self * rhs.y )
            }
        }
    )*)
}

impl_mul_scalar_with_vector2! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_mul_scalar_with_vector3 {
    ($($type: ty)+ ) => ($(
        impl<T> Mul<Vector3<T>> for $type where
            $type: Mul<T>
        {
            type Output = Vector3<<$type as Mul<T>>::Output>;

            fn mul(self, rhs: Vector3<T>) -> Self::Output {
                Vector3::new( self * rhs.x, self * rhs.y, self * rhs.z )
            }
        }
    )*)
}

impl_mul_scalar_with_vector3! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let vec = Vector2::new(1 as $type, 2 as $type);

                assert_eq!(vec.x, 1 as $type);
                assert_eq!(vec.y, 2 as $type);
            }
        };
    }

    new_vector2! { u8, new_vector2_u8 }
    new_vector2! { u16, new_vector2_u16 }
    new_vector2! { u32, new_vector2_u32 }
    new_vector2! { u64, new_vector2_u64 }
    new_vector2! { u128, new_vector2_u128 }
    new_vector2! { i8, new_vector2_i8 }
    new_vector2! { i16, new_vector2_i16 }
    new_vector2! { i32, new_vector2_i32 }
    new_vector2! { i64, new_vector2_i64 }
    new_vector2! { i128, new_vector2_i128 }
    new_vector2! { f32, new_vector2_f32 }
    new_vector2! { f64, new_vector2_f64 }

    macro_rules! add_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector2::new(1 as $type, 2 as $type);
                let v2 = Vector2::new(4 as $type, 5 as $type);

                assert_eq!(v1 + v2, Vector2::new(5 as $type, 7 as $type));
            }
        };
    }

    add_vector2! { u8, add_vector2_u8 }
    add_vector2! { u16, add_vector2_u16 }
    add_vector2! { u32, add_vector2_u32 }
    add_vector2! { u64, add_vector2_u64 }
    add_vector2! { u128, add_vector2_u128 }
    add_vector2! { i8, add_vector2_i8 }
    add_vector2! { i16, add_vector2_i16 }
    add_vector2! { i32, add_vector2_i32 }
    add_vector2! { i64, add_vector2_i64 }
    add_vector2! { i128, add_vector2_i128 }
    add_vector2! { f32, add_vector2_f32 }
    add_vector2! { f64, add_vector2_f64 }

    macro_rules! vector2_add_point2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector2::new(1 as $type, 2 as $type);
                let p = Point2::new(4 as $type, 5 as $type);

                assert_eq!(v + p, Point2::new(5 as $type, 7 as $type));
            }
        };
    }

    vector2_add_point2! { u8, vector2_add_point2_u8 }
    vector2_add_point2! { u16, vector2_add_point2_u16 }
    vector2_add_point2! { u32, vector2_add_point2_u32 }
    vector2_add_point2! { u64, vector2_add_point2_u64 }
    vector2_add_point2! { u128, vector2_add_point2_u128 }
    vector2_add_point2! { i8, vector2_add_point2_i8 }
    vector2_add_point2! { i16, vector2_add_point2_i16 }
    vector2_add_point2! { i32, vector2_add_point2_i32 }
    vector2_add_point2! { i64, vector2_add_point2_i64 }
    vector2_add_point2! { i128, vector2_add_point2_i128 }
    vector2_add_point2! { f32, vector2_add_point2_f32 }
    vector2_add_point2! { f64, vector2_add_point2_f64 }

    macro_rules! add_assign_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v1 = Vector2::new(1 as $type, 2 as $type);
                let v2 = Vector2::new(4 as $type, 5 as $type);

                v1 += v2;

                assert_eq!(v1, Vector2::new(5 as $type, 7 as $type));
            }
        };
    }

    add_assign_vector2! { u8, add_assign_vector2_u8 }
    add_assign_vector2! { u16, add_assign_vector2_u16 }
    add_assign_vector2! { u32, add_assign_vector2_u32 }
    add_assign_vector2! { u64, add_assign_vector2_u64 }
    add_assign_vector2! { u128, add_assign_vector2_u128 }
    add_assign_vector2! { i8, add_assign_vector2_i8 }
    add_assign_vector2! { i16, add_assign_vector2_i16 }
    add_assign_vector2! { i32, add_assign_vector2_i32 }
    add_assign_vector2! { i64, add_assign_vector2_i64 }
    add_assign_vector2! { i128, add_assign_vector2_i128 }
    add_assign_vector2! { f32, add_assign_vector2_f32 }
    add_assign_vector2! { f64, add_assign_vector2_f64 }

    macro_rules! sub_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector2::new(4 as $type, 5 as $type);
                let v2 = Vector2::new(1 as $type, 2 as $type);

                assert_eq!(v1 - v2, Vector2::new(3 as $type, 3 as $type));
            }
        };
    }

    sub_vector2! { u8, sub_vector2_u8 }
    sub_vector2! { u16, sub_vector2_u16 }
    sub_vector2! { u32, sub_vector2_u32 }
    sub_vector2! { u64, sub_vector2_u64 }
    sub_vector2! { u128, sub_vector2_u128 }
    sub_vector2! { i8, sub_vector2_i8 }
    sub_vector2! { i16, sub_vector2_i16 }
    sub_vector2! { i32, sub_vector2_i32 }
    sub_vector2! { i64, sub_vector2_i64 }
    sub_vector2! { i128, sub_vector2_i128 }
    sub_vector2! { f32, sub_vector2_f32 }
    sub_vector2! { f64, sub_vector2_f64 }

    macro_rules! sub_assign_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v1 = Vector2::new(4 as $type, 5 as $type);
                let v2 = Vector2::new(1 as $type, 2 as $type);

                v1 -= v2;

                assert_eq!(v1, Vector2::new(3 as $type, 3 as $type));
            }
        };
    }

    sub_assign_vector2! { u8, sub_assign_vector2_u8 }
    sub_assign_vector2! { u16, sub_assign_vector2_u16 }
    sub_assign_vector2! { u32, sub_assign_vector2_u32 }
    sub_assign_vector2! { u64, sub_assign_vector2_u64 }
    sub_assign_vector2! { u128, sub_assign_vector2_u128 }
    sub_assign_vector2! { i8, sub_assign_vector2_i8 }
    sub_assign_vector2! { i16, sub_assign_vector2_i16 }
    sub_assign_vector2! { i32, sub_assign_vector2_i32 }
    sub_assign_vector2! { i64, sub_assign_vector2_i64 }
    sub_assign_vector2! { i128, sub_assign_vector2_i128 }
    sub_assign_vector2! { f32, sub_assign_vector2_f32 }
    sub_assign_vector2! { f64, sub_assign_vector2_f64 }

    macro_rules! mul_vector2_scalar {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector2::new(2 as $type, 3 as $type);
                let r = Vector2::new(4 as $type, 6 as $type);

                assert_eq!(v * 2 as $type, r);
            }
        };
    }

    mul_vector2_scalar! { u8, mul_vector2_scalar_u8 }
    mul_vector2_scalar! { u16, mul_vector2_scalar_u16 }
    mul_vector2_scalar! { u32, mul_vector2_scalar_u32 }
    mul_vector2_scalar! { u64, mul_vector2_scalar_u64 }
    mul_vector2_scalar! { u128, mul_vector2_scalar_u128 }
    mul_vector2_scalar! { i8, mul_vector2_scalar_i8 }
    mul_vector2_scalar! { i16, mul_vector2_scalar_i16 }
    mul_vector2_scalar! { i32, mul_vector2_scalar_i32 }
    mul_vector2_scalar! { i64, mul_vector2_scalar_i64 }
    mul_vector2_scalar! { i128, mul_vector2_scalar_i128 }
    mul_vector2_scalar! { f32, mul_vector2_scalar_f32 }
    mul_vector2_scalar! { f64, mul_vector2_scalar_f64 }

    macro_rules! mul_scalar_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector2::new(2 as $type, 3 as $type);
                let r = Vector2::new(4 as $type, 6 as $type);

                assert_eq!(2 as $type * v, r);
            }
        };
    }

    mul_scalar_vector2! { u8, mul_scalar_vector2_u8 }
    mul_scalar_vector2! { u16, mul_scalar_vector2_u16 }
    mul_scalar_vector2! { u32, mul_scalar_vector2_u32 }
    mul_scalar_vector2! { u64, mul_scalar_vector2_u64 }
    mul_scalar_vector2! { u128, mul_scalar_vector2_u128 }
    mul_scalar_vector2! { i8, mul_scalar_vector2_i8 }
    mul_scalar_vector2! { i16, mul_scalar_vector2_i16 }
    mul_scalar_vector2! { i32, mul_scalar_vector2_i32 }
    mul_scalar_vector2! { i64, mul_scalar_vector2_i64 }
    mul_scalar_vector2! { i128, mul_scalar_vector2_i128 }
    mul_scalar_vector2! { f32, mul_scalar_vector2_f32 }
    mul_scalar_vector2! { f64, mul_scalar_vector2_f64 }

    macro_rules! dot_product_vector2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec1 = Vector2::<$type>::x_axis();
                let y_vec1 = Vector2::<$type>::y_axis();

                assert_eq!(x_vec1.dot(y_vec1), 0 as $type);

                let x_vec2 = x_vec1 * 2 as $type;
                let y_vec2 = y_vec1 * 2 as $type;

                let x_vec3 = x_vec1 * 3 as $type;
                let y_vec3 = y_vec1 * 3 as $type;

                assert_eq!(x_vec2.dot(x_vec3), 6 as $type);
                assert_eq!(y_vec2.dot(y_vec3), 6 as $type);
            }
        };
    }

    dot_product_vector2! { u8, dot_product_vector2_u8 }
    dot_product_vector2! { u16, dot_product_vector2_u16 }
    dot_product_vector2! { u32, dot_product_vector2_u32 }
    dot_product_vector2! { u64, dot_product_vector2_u64 }
    dot_product_vector2! { u128, dot_product_vector2_u128 }
    dot_product_vector2! { i8, dot_product_vector2_i8 }
    dot_product_vector2! { i16, dot_product_vector2_i16 }
    dot_product_vector2! { i32, dot_product_vector2_i32 }
    dot_product_vector2! { i64, dot_product_vector2_i64 }
    dot_product_vector2! { i128, dot_product_vector2_i128 }
    dot_product_vector2! { f32, dot_product_vector2_f32 }
    dot_product_vector2! { f64, dot_product_vector2_f64 }

    // reflect on

    macro_rules! vector2_mul_assign {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v = Vector2::new(2 as $type, 4 as $type);
                let r = Vector2::new(4 as $type, 8 as $type);

                v *= 2.0 as $type;

                assert_eq!(v, r);
            }
        };
    }

    vector2_mul_assign! { u16, vector2_mul_assign_u16 }
    vector2_mul_assign! { u32, vector2_mul_assign_u32 }
    vector2_mul_assign! { u64, vector2_mul_assign_u64 }
    vector2_mul_assign! { u128, vector2_mul_assign_u128 }
    vector2_mul_assign! { i8, vector2_mul_assign_i8 }
    vector2_mul_assign! { i16, vector2_mul_assign_i16 }
    vector2_mul_assign! { i32, vector2_mul_assign_i32 }
    vector2_mul_assign! { i64, vector2_mul_assign_i64 }
    vector2_mul_assign! { i128, vector2_mul_assign_i128 }
    vector2_mul_assign! { f32, vector2_mul_assign_f32 }
    vector2_mul_assign! { f64, vector2_mul_assign_f64 }

    macro_rules! vector2_div {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector2::new(2 as $type, 4 as $type);
                let r = Vector2::new(1 as $type, 2 as $type);

                assert_eq!(v / 2 as $type, r);
            }
        };
    }

    vector2_div! { u8, vector2_div_u8 }
    vector2_div! { u16, vector2_div_u16 }
    vector2_div! { u32, vector2_div_u32 }
    vector2_div! { u64, vector2_div_u64 }
    vector2_div! { u128, vector2_div_u128 }
    vector2_div! { i8, vector2_div_i8 }
    vector2_div! { i16, vector2_div_i16 }
    vector2_div! { i32, vector2_div_i32 }
    vector2_div! { i64, vector2_div_i64 }
    vector2_div! { i128, vector2_div_i128 }
    vector2_div! { f32, vector2_div_f32 }
    vector2_div! { f64, vector2_div_f64 }

    macro_rules! vector2_div_assign {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v = Vector2::new(2 as $type, 4 as $type);
                let r = Vector2::new(1 as $type, 2 as $type);

                v /= 2.0 as $type;

                assert_eq!(v, r);
            }
        };
    }

    vector2_div_assign! { u16, vector2_div_assign_u16 }
    vector2_div_assign! { u32, vector2_div_assign_u32 }
    vector2_div_assign! { u64, vector2_div_assign_u64 }
    vector2_div_assign! { u128, vector2_div_assign_u128 }
    vector2_div_assign! { i8, vector2_div_assign_i8 }
    vector2_div_assign! { i16, vector2_div_assign_i16 }
    vector2_div_assign! { i32, vector2_div_assign_i32 }
    vector2_div_assign! { i64, vector2_div_assign_i64 }
    vector2_div_assign! { i128, vector2_div_assign_i128 }
    vector2_div_assign! { f32, vector2_div_assign_f32 }
    vector2_div_assign! { f64, vector2_div_assign_f64 }

    macro_rules! vector2_neg {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector2::new(1 as $type, 2 as $type);
                let v2 = -v1;

                assert_eq!(v2.x, -1 as $type);
                assert_eq!(v2.y, -2 as $type);
            }
        };
    }

    vector2_neg! { i8, vector2_neg_i8 }
    vector2_neg! { i16, vector2_neg_i16 }
    vector2_neg! { i32, vector2_neg_i32 }
    vector2_neg! { i64, vector2_neg_i64 }
    vector2_neg! { i128, vector2_neg_i128 }
    vector2_neg! { f32, vector2_neg_f32 }
    vector2_neg! { f64, vector2_neg_f64 }

    macro_rules! vector2_magnitude {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec = Vector2::new(2 as $type, 0 as $type);
                let y_vec = Vector2::new(0 as $type, 3 as $type);

                assert_eq!(x_vec.magnitude(), 2 as $type);
                assert_eq!(y_vec.magnitude(), 3 as $type);
            }
        };
    }

    vector2_magnitude! { f32, vector2_magnitude_f32 }
    vector2_magnitude! { f64, vector2_magnitude_f64 }

    macro_rules! vector2_normalized {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec = Vector2::new(2 as $type, 0 as $type);
                let y_vec = Vector2::new(0 as $type, 3 as $type);

                assert_eq!(x_vec.normalized(), Vector2::new(1 as $type, 0 as $type));
                assert_eq!(y_vec.normalized(), Vector2::new(0 as $type, 1 as $type));
            }
        };
    }

    vector2_normalized! { f32, vector2_normalized_f32 }
    vector2_normalized! { f64, vector2_normalized_f64 }

    macro_rules! new_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let vec = Vector3::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(vec.x, 1 as $type);
                assert_eq!(vec.y, 2 as $type);
                assert_eq!(vec.z, 3 as $type);
            }
        };
    }

    new_vector3! { u8, new_vector3_u8 }
    new_vector3! { u16, new_vector3_u16 }
    new_vector3! { u32, new_vector3_u32 }
    new_vector3! { u64, new_vector3_u64 }
    new_vector3! { u128, new_vector3_u128 }
    new_vector3! { i8, new_vector3_i8 }
    new_vector3! { i16, new_vector3_i16 }
    new_vector3! { i32, new_vector3_i32 }
    new_vector3! { i64, new_vector3_i64 }
    new_vector3! { i128, new_vector3_i128 }
    new_vector3! { f32, new_vector3_f32 }
    new_vector3! { f64, new_vector3_f64 }

    macro_rules! vector3_cross {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_axis = Vector3::<$type>::x_axis();
                let y_axis = Vector3::<$type>::y_axis();
                let z_axis = Vector3::<$type>::z_axis();

                assert_eq!(x_axis, Vector3::cross(y_axis, z_axis));
                assert_eq!(-x_axis, Vector3::cross(z_axis, y_axis));

                assert_eq!(y_axis, Vector3::cross(z_axis, x_axis));
                assert_eq!(-y_axis, Vector3::cross(x_axis, z_axis));

                assert_eq!(z_axis, Vector3::cross(x_axis, y_axis));
                assert_eq!(-z_axis, Vector3::cross(y_axis, x_axis));
            }
        };
    }

    vector3_cross! { i8, vector3_cross_i8 }
    vector3_cross! { i16, vector3_cross_i16 }
    vector3_cross! { i32, vector3_cross_i32 }
    vector3_cross! { i64, vector3_cross_i64 }
    vector3_cross! { i128, vector3_cross_i128 }
    vector3_cross! { f32, vector3_cross_f32 }
    vector3_cross! { f64, vector3_cross_f64 }

    macro_rules! add_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector3::new(1 as $type, 2 as $type, 3 as $type);
                let v2 = Vector3::new(4 as $type, 5 as $type, 6 as $type);

                assert_eq!(v1 + v2, Vector3::new(5 as $type, 7 as $type, 9 as $type));
            }
        };
    }

    add_vector3! { u8, add_vector3_u8 }
    add_vector3! { u16, add_vector3_u16 }
    add_vector3! { u32, add_vector3_u32 }
    add_vector3! { u64, add_vector3_u64 }
    add_vector3! { u128, add_vector3_u128 }
    add_vector3! { i8, add_vector3_i8 }
    add_vector3! { i16, add_vector3_i16 }
    add_vector3! { i32, add_vector3_i32 }
    add_vector3! { i64, add_vector3_i64 }
    add_vector3! { i128, add_vector3_i128 }
    add_vector3! { f32, add_vector3_f32 }
    add_vector3! { f64, add_vector3_f64 }

    macro_rules! vector3_add_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector3::new(1 as $type, 2 as $type, 3 as $type);
                let p = Point3::new(4 as $type, 5 as $type, 6 as $type);

                assert_eq!(v + p, Point3::new(5 as $type, 7 as $type, 9 as $type));
            }
        };
    }

    vector3_add_point3! { u8, vector3_add_point3_u8 }
    vector3_add_point3! { u16, vector3_add_point3_u16 }
    vector3_add_point3! { u32, vector3_add_point3_u32 }
    vector3_add_point3! { u64, vector3_add_point3_u64 }
    vector3_add_point3! { u128, vector3_add_point3_u128 }
    vector3_add_point3! { i8, vector3_add_point3_i8 }
    vector3_add_point3! { i16, vector3_add_point3_i16 }
    vector3_add_point3! { i32, vector3_add_point3_i32 }
    vector3_add_point3! { i64, vector3_add_point3_i64 }
    vector3_add_point3! { i128, vector3_add_point3_i128 }
    vector3_add_point3! { f32, vector3_add_point3_f32 }
    vector3_add_point3! { f64, vector3_add_point3_f64 }

    macro_rules! add_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v1 = Vector3::new(1 as $type, 2 as $type, 3 as $type);
                let v2 = Vector3::new(4 as $type, 5 as $type, 6 as $type);

                v1 += v2;

                assert_eq!(v1, Vector3::new(5 as $type, 7 as $type, 9 as $type));
            }
        };
    }

    add_assign_vector3! { u8, add_assign_vector3_u8 }
    add_assign_vector3! { u16, add_assign_vector3_u16 }
    add_assign_vector3! { u32, add_assign_vector3_u32 }
    add_assign_vector3! { u64, add_assign_vector3_u64 }
    add_assign_vector3! { u128, add_assign_vector3_u128 }
    add_assign_vector3! { i8, add_assign_vector3_i8 }
    add_assign_vector3! { i16, add_assign_vector3_i16 }
    add_assign_vector3! { i32, add_assign_vector3_i32 }
    add_assign_vector3! { i64, add_assign_vector3_i64 }
    add_assign_vector3! { i128, add_assign_vector3_i128 }
    add_assign_vector3! { f32, add_assign_vector3_f32 }
    add_assign_vector3! { f64, add_assign_vector3_f64 }

    macro_rules! sub_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector3::new(4 as $type, 5 as $type, 6 as $type);
                let v2 = Vector3::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(v1 - v2, Vector3::new(3 as $type, 3 as $type, 3 as $type));
            }
        };
    }

    sub_vector3! { u8, sub_vector3_u8 }
    sub_vector3! { u16, sub_vector3_u16 }
    sub_vector3! { u32, sub_vector3_u32 }
    sub_vector3! { u64, sub_vector3_u64 }
    sub_vector3! { u128, sub_vector3_u128 }
    sub_vector3! { i8, sub_vector3_i8 }
    sub_vector3! { i16, sub_vector3_i16 }
    sub_vector3! { i32, sub_vector3_i32 }
    sub_vector3! { i64, sub_vector3_i64 }
    sub_vector3! { i128, sub_vector3_i128 }
    sub_vector3! { f32, sub_vector3_f32 }
    sub_vector3! { f64, sub_vector3_f64 }

    macro_rules! sub_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v1 = Vector3::new(4 as $type, 5 as $type, 6 as $type);
                let v2 = Vector3::new(1 as $type, 2 as $type, 3 as $type);

                v1 -= v2;

                assert_eq!(v1, Vector3::new(3 as $type, 3 as $type, 3 as $type));
            }
        };
    }

    sub_assign_vector3! { u8, sub_assign_vector3_u8 }
    sub_assign_vector3! { u16, sub_assign_vector3_u16 }
    sub_assign_vector3! { u32, sub_assign_vector3_u32 }
    sub_assign_vector3! { u64, sub_assign_vector3_u64 }
    sub_assign_vector3! { u128, sub_assign_vector3_u128 }
    sub_assign_vector3! { i8, sub_assign_vector3_i8 }
    sub_assign_vector3! { i16, sub_assign_vector3_i16 }
    sub_assign_vector3! { i32, sub_assign_vector3_i32 }
    sub_assign_vector3! { i64, sub_assign_vector3_i64 }
    sub_assign_vector3! { i128, sub_assign_vector3_i128 }
    sub_assign_vector3! { f32, sub_assign_vector3_f32 }
    sub_assign_vector3! { f64, sub_assign_vector3_f64 }

    macro_rules! mul_vector3_scalar {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector3::new(2 as $type, 3 as $type, 4 as $type);
                let r = Vector3::new(4 as $type, 6 as $type, 8 as $type);

                assert_eq!(v * 2 as $type, r);
            }
        };
    }

    mul_vector3_scalar! { u8, mul_vector3_scalar_u8 }
    mul_vector3_scalar! { u16, mul_vector3_scalar_u16 }
    mul_vector3_scalar! { u32, mul_vector3_scalar_u32 }
    mul_vector3_scalar! { u64, mul_vector3_scalar_u64 }
    mul_vector3_scalar! { u128, mul_vector3_scalar_u128 }
    mul_vector3_scalar! { i8, mul_vector3_scalar_i8 }
    mul_vector3_scalar! { i16, mul_vector3_scalar_i16 }
    mul_vector3_scalar! { i32, mul_vector3_scalar_i32 }
    mul_vector3_scalar! { i64, mul_vector3_scalar_i64 }
    mul_vector3_scalar! { i128, mul_vector3_scalar_i128 }
    mul_vector3_scalar! { f32, mul_vector3_scalar_f32 }
    mul_vector3_scalar! { f64, mul_vector3_scalar_f64 }

    macro_rules! mul_scalar_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector3::new(2 as $type, 3 as $type, 4 as $type);
                let r = Vector3::new(4 as $type, 6 as $type, 8 as $type);

                assert_eq!(2 as $type * v, r);
            }
        };
    }

    mul_scalar_vector3! { u8, mul_scalar_vector3_u8 }
    mul_scalar_vector3! { u16, mul_scalar_vector3_u16 }
    mul_scalar_vector3! { u32, mul_scalar_vector3_u32 }
    mul_scalar_vector3! { u64, mul_scalar_vector3_u64 }
    mul_scalar_vector3! { u128, mul_scalar_vector3_u128 }
    mul_scalar_vector3! { i8, mul_scalar_vector3_i8 }
    mul_scalar_vector3! { i16, mul_scalar_vector3_i16 }
    mul_scalar_vector3! { i32, mul_scalar_vector3_i32 }
    mul_scalar_vector3! { i64, mul_scalar_vector3_i64 }
    mul_scalar_vector3! { i128, mul_scalar_vector3_i128 }
    mul_scalar_vector3! { f32, mul_scalar_vector3_f32 }
    mul_scalar_vector3! { f64, mul_scalar_vector3_f64 }

    macro_rules! dot_product_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec1 = Vector3::<$type>::x_axis();
                let y_vec1 = Vector3::<$type>::y_axis();
                let z_vec1 = Vector3::<$type>::z_axis();

                assert_eq!(x_vec1.dot(y_vec1), 0 as $type);
                assert_eq!(x_vec1.dot(z_vec1), 0 as $type);
                assert_eq!(y_vec1.dot(z_vec1), 0 as $type);

                let x_vec2 = x_vec1 * 2 as $type;
                let y_vec2 = y_vec1 * 2 as $type;
                let z_vec2 = x_vec1 * 2 as $type;

                let x_vec3 = x_vec1 * 3 as $type;
                let y_vec3 = y_vec1 * 3 as $type;
                let z_vec3 = x_vec1 * 3 as $type;

                assert_eq!(x_vec2.dot(x_vec3), 6 as $type);
                assert_eq!(y_vec2.dot(y_vec3), 6 as $type);
                assert_eq!(z_vec2.dot(z_vec3), 6 as $type);
            }
        };
    }

    dot_product_vector3! { u8, dot_product_vector3_u8 }
    dot_product_vector3! { u16, dot_product_vector3_u16 }
    dot_product_vector3! { u32, dot_product_vector3_u32 }
    dot_product_vector3! { u64, dot_product_vector3_u64 }
    dot_product_vector3! { u128, dot_product_vector3_u128 }
    dot_product_vector3! { i8, dot_product_vector3_i8 }
    dot_product_vector3! { i16, dot_product_vector3_i16 }
    dot_product_vector3! { i32, dot_product_vector3_i32 }
    dot_product_vector3! { i64, dot_product_vector3_i64 }
    dot_product_vector3! { i128, dot_product_vector3_i128 }
    dot_product_vector3! { f32, dot_product_vector3_f32 }
    dot_product_vector3! { f64, dot_product_vector3_f64 }

    // reflect on

    macro_rules! vector3_mul_assign {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v = Vector3::new(2 as $type, 4 as $type, 8 as $type);
                let r = Vector3::new(4 as $type, 8 as $type, 16 as $type);

                v *= 2.0 as $type;

                assert_eq!(v, r);
            }
        };
    }

    vector3_mul_assign! { u16, vector3_mul_assign_u16 }
    vector3_mul_assign! { u32, vector3_mul_assign_u32 }
    vector3_mul_assign! { u64, vector3_mul_assign_u64 }
    vector3_mul_assign! { u128, vector3_mul_assign_u128 }
    vector3_mul_assign! { i8, vector3_mul_assign_i8 }
    vector3_mul_assign! { i16, vector3_mul_assign_i16 }
    vector3_mul_assign! { i32, vector3_mul_assign_i32 }
    vector3_mul_assign! { i64, vector3_mul_assign_i64 }
    vector3_mul_assign! { i128, vector3_mul_assign_i128 }
    vector3_mul_assign! { f32, vector3_mul_assign_f32 }
    vector3_mul_assign! { f64, vector3_mul_assign_f64 }

    macro_rules! vector3_div {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = Vector3::new(2 as $type, 4 as $type, 8 as $type);
                let r = Vector3::new(1 as $type, 2 as $type, 4 as $type);

                assert_eq!(v / 2 as $type, r);
            }
        };
    }

    vector3_div! { u8, vector3_div_u8 }
    vector3_div! { u16, vector3_div_u16 }
    vector3_div! { u32, vector3_div_u32 }
    vector3_div! { u64, vector3_div_u64 }
    vector3_div! { u128, vector3_div_u128 }
    vector3_div! { i8, vector3_div_i8 }
    vector3_div! { i16, vector3_div_i16 }
    vector3_div! { i32, vector3_div_i32 }
    vector3_div! { i64, vector3_div_i64 }
    vector3_div! { i128, vector3_div_i128 }
    vector3_div! { f32, vector3_div_f32 }
    vector3_div! { f64, vector3_div_f64 }

    macro_rules! vector3_div_assign {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v = Vector3::new(2 as $type, 4 as $type, 8 as $type);
                let r = Vector3::new(1 as $type, 2 as $type, 4 as $type);

                v /= 2.0 as $type;

                assert_eq!(v, r);
            }
        };
    }

    vector3_div_assign! { u16, vector3_div_assign_u16 }
    vector3_div_assign! { u32, vector3_div_assign_u32 }
    vector3_div_assign! { u64, vector3_div_assign_u64 }
    vector3_div_assign! { u128, vector3_div_assign_u128 }
    vector3_div_assign! { i8, vector3_div_assign_i8 }
    vector3_div_assign! { i16, vector3_div_assign_i16 }
    vector3_div_assign! { i32, vector3_div_assign_i32 }
    vector3_div_assign! { i64, vector3_div_assign_i64 }
    vector3_div_assign! { i128, vector3_div_assign_i128 }
    vector3_div_assign! { f32, vector3_div_assign_f32 }
    vector3_div_assign! { f64, vector3_div_assign_f64 }

    macro_rules! vector3_neg {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v1 = Vector3::new(1 as $type, 2 as $type, -3 as $type);
                let v2 = -v1;

                assert_eq!(v2.x, -1 as $type);
                assert_eq!(v2.y, -2 as $type);
                assert_eq!(v2.z, 3 as $type);
            }
        };
    }

    vector3_neg! { i8, vector3_neg_i8 }
    vector3_neg! { i16, vector3_neg_i16 }
    vector3_neg! { i32, vector3_neg_i32 }
    vector3_neg! { i64, vector3_neg_i64 }
    vector3_neg! { i128, vector3_neg_i128 }
    vector3_neg! { f32, vector3_neg_f32 }
    vector3_neg! { f64, vector3_neg_f64 }

    macro_rules! vector3_magnitude {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec = Vector3::new(2 as $type, 0 as $type, 0 as $type);
                let y_vec = Vector3::new(0 as $type, 3 as $type, 0 as $type);
                let z_vec = Vector3::new(0 as $type, 0 as $type, 4 as $type);

                assert_eq!(x_vec.magnitude(), 2 as $type);
                assert_eq!(y_vec.magnitude(), 3 as $type);
                assert_eq!(z_vec.magnitude(), 4 as $type);
            }
        };
    }

    vector3_magnitude! { f32, vector3_magnitude_f32 }
    vector3_magnitude! { f64, vector3_magnitude_f64 }

    macro_rules! vector3_normalized {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec = Vector3::new(2 as $type, 0 as $type, 0 as $type);
                let y_vec = Vector3::new(0 as $type, 3 as $type, 0 as $type);
                let z_vec = Vector3::new(0 as $type, 0 as $type, 4 as $type);

                assert_eq!(
                    x_vec.normalized(),
                    Vector3::new(1 as $type, 0 as $type, 0 as $type)
                );
                assert_eq!(
                    y_vec.normalized(),
                    Vector3::new(0 as $type, 1 as $type, 0 as $type)
                );
                assert_eq!(
                    z_vec.normalized(),
                    Vector3::new(0 as $type, 0 as $type, 1 as $type)
                );
            }
        };
    }

    vector3_normalized! { f32, vector3_normalized_f32 }
    vector3_normalized! { f64, vector3_normalized_f64 }
}
