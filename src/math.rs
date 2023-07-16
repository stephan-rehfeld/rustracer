pub mod geometry;

use std::ops;
use crate::traits;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Vector3<T> {
    pub fn new( x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    pub fn cross(a: Vector3<T>, b: Vector3<T>) -> Vector3<<<T as ops::Mul>::Output as ops::Sub>::Output>
    where
        T: ops::Mul,
        <T as ops::Mul>::Output: ops::Sub,
        T: Copy + Clone,
    {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        )
    }
}

impl<T: ops::Add<U> , U> ops::Add<Vector3<U>> for Vector3<T> {
    type Output = Vector3<<T as ops::Add<U>>::Output>;

    fn add(self, rhs: Vector3<U>) -> Self::Output {
        Vector3::new( self.x + rhs.x, self.y + rhs.y, self.z + rhs.z )
    }
}

impl<T: ops::AddAssign<U> , U> ops::AddAssign<Vector3<U>> for Vector3<T> {
    fn add_assign(&mut self, rhs: Vector3<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: ops::Sub<U>, U> ops::Sub<Vector3<U>> for Vector3<T> {
    type Output = Vector3<<T as ops::Sub<U>>::Output>;

    fn sub(self, rhs: Vector3<U>) -> Self::Output {
        Vector3::new( self.x - rhs.x, self.y - rhs.y, self.z - rhs.z )
    }
}

impl<T: ops::SubAssign<U>, U> ops::SubAssign<Vector3<U>> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Vector3<U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: ops::Mul + Clone + Copy> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<<T as ops::Mul>::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new( self.x * rhs, self.y * rhs, self.z * rhs )
    }
}

macro_rules! mul_scalar_with_vector3 {
    ($($type: ty)* ) => ($(
        impl ops::Mul<Vector3<$type>> for $type  {
            type Output = Vector3<<$type as ops::Mul>::Output>;

            fn mul(self, rhs: Vector3<$type>) -> Self::Output {
                Vector3::new( self * rhs.x, self * rhs.y, self * rhs.z )
            }
        }
    )*)
}

mul_scalar_with_vector3! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }

impl<T> ops::Mul for Vector3<T> 
    where
        T: ops::Mul,
        <T as ops::Mul>::Output: ops::Add,
        <<T as ops::Mul>::Output as ops::Add>::Output: ops::Add,
        <<T as ops::Mul>::Output as ops::Add>::Output: ops::Add<<T as ops::Mul>::Output>,
        T: Copy + Clone
{
    type Output = <<<T as ops::Mul>::Output as ops::Add>::Output as ops::Add<<T as ops::Mul>::Output>>::Output;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: ops::MulAssign<U>, U: Copy + Clone> ops::MulAssign<U> for Vector3<T> {
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: ops::Div<U>, U: Copy + Clone> ops::Div<U> for Vector3<T> {
    type Output = Vector3<<T as ops::Div<U>>::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: ops::DivAssign<U>, U: Copy + Clone> ops::DivAssign<U> for Vector3<T> {
    fn div_assign(&mut self, rhs: U) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: ops::Neg> ops::Neg for Vector3<T> {
    type Output = Vector3<<T as ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Vector3::new( -self.x, -self.y, -self.z)
    }
}

impl<T> Vector3<T> where 
    T: traits::Sqrt<Output = T>,
    T: ops::Add<Output = T>,
    T: ops::Mul<Output = T>,
    T: ops::Div,
    T: ops::DivAssign,
    T: Copy + Clone 
{
    pub fn magnitude(self) -> <T as traits::Sqrt>::Output {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    pub fn normalized(self) -> Vector3<<T as ops::Div>::Output> {
        self / self.magnitude()
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Point3<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Point3<T> {
    pub fn new( x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }
}

impl<T: ops::Add<U> + Clone + Copy, U> ops::Add<Vector3<U>> for Point3<T> {
    type Output = Point3<<T as ops::Add<U>>::Output>;

    fn add(self, rhs: Vector3<U>) -> Self::Output {
         Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: ops::Sub<Output = T> + Clone + Copy> ops::Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Mat3x3<T> {
    m11: T,
    m12: T,
    m13: T,
    m21: T,
    m22: T,
    m23: T,
    m31: T,
    m32: T,
    m33: T
}

impl<T> Mat3x3<T> {
    pub fn new(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T, m31: T, m32: T, m33: T) -> Mat3x3<T> {
        Mat3x3 { m11, m12, m13, m21, m22, m23, m31, m32, m33 }
    }

    pub fn change_column_1(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new( v.x, self.m12, self.m13,
                     v.y, self.m22, self.m23,
                     v.z, self.m32, self.m33 )
    }

    pub fn change_column_2(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new( self.m11, v.x, self.m13,
                     self.m21, v.y, self.m23,
                     self.m31, v.z, self.m33 )
    }

    pub fn change_column_3(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new( self.m11, self.m12, v.x,
                     self.m21, self.m22, v.y,
                     self.m31, self.m32, v.z )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! new_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let vec = Vector3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(vec.x, 1 as $type);
                assert_eq!(vec.y, 2 as $type);
                assert_eq!(vec.z, 3 as $type);
            }
        }
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
                let x_axis = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let y_axis = Vector3::new(0 as $type, 1 as $type, 0 as $type);
                let z_axis = Vector3::new(0 as $type, 0 as $type, 1 as $type);

                assert_eq!(x_axis, Vector3::cross(y_axis, z_axis));
                assert_eq!(-x_axis, Vector3::cross(z_axis, y_axis));

                assert_eq!(y_axis, Vector3::cross(z_axis, x_axis));
                assert_eq!(-y_axis, Vector3::cross(x_axis, z_axis));

                assert_eq!(z_axis, Vector3::cross(x_axis, y_axis));
                assert_eq!(-z_axis, Vector3::cross(y_axis, x_axis));
            }
        }
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
                let v1 = Vector3::new( 1 as $type, 2 as $type, 3 as $type );
                let v2 = Vector3::new( 4 as $type, 5 as $type, 6 as $type );

                assert_eq!(v1 + v2, Vector3::new(5 as $type, 7 as $type, 9 as $type));
            }
        }
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

    macro_rules! add_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v1 = Vector3::new( 1 as $type, 2 as $type, 3 as $type );
                let v2 = Vector3::new( 4 as $type, 5 as $type, 6 as $type );

                v1 += v2;

                assert_eq!(v1, Vector3::new(5 as $type, 7 as $type, 9 as $type));
            }
        }
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
                let v1 = Vector3::new( 4 as $type, 5 as $type, 6 as $type );
                let v2 = Vector3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(v1 - v2, Vector3::new(3 as $type, 3 as $type, 3 as $type));
            }
        }
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
                let mut v1 = Vector3::new( 4 as $type, 5 as $type, 6 as $type );
                let v2 = Vector3::new( 1 as $type, 2 as $type, 3 as $type );

                v1 -= v2;

                assert_eq!(v1, Vector3::new(3 as $type, 3 as $type, 3 as $type));
            }
        }
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
                let v = Vector3::new( 2 as $type, 3 as $type, 4 as $type );
                let r = Vector3::new( 4 as $type, 6 as $type, 8 as $type );

                assert_eq!( v * 2 as $type, r);
            }
        }
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
                let v = Vector3::new( 2 as $type, 3 as $type, 4 as $type );
                let r = Vector3::new( 4 as $type, 6 as $type, 8 as $type );

                assert_eq!( 2 as $type * v, r);
            }
        }
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
                let x_vec1 = Vector3::new( 1 as $type, 0 as $type, 0 as $type );
                let y_vec1 = Vector3::new( 0 as $type, 1 as $type, 0 as $type );
                let z_vec1 = Vector3::new( 0 as $type, 0 as $type, 1 as $type );

                assert_eq!(x_vec1 * y_vec1, 0 as $type);
                assert_eq!(x_vec1 * z_vec1, 0 as $type);
                assert_eq!(y_vec1 * z_vec1, 0 as $type);

                let x_vec2 = x_vec1 * 2 as $type;
                let y_vec2 = y_vec1 * 2 as $type;
                let z_vec2 = x_vec1 * 2 as $type;

                let x_vec3 = x_vec1 * 3 as $type;
                let y_vec3 = y_vec1 * 3 as $type;
                let z_vec3 = x_vec1 * 3 as $type;

                assert_eq!(x_vec2 * x_vec3, 6 as $type); 
                assert_eq!(y_vec2 * y_vec3, 6 as $type); 
                assert_eq!(z_vec2 * z_vec3, 6 as $type); 
            }
        }
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
    
    macro_rules! vector3_mul_assign {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut v = Vector3::new( 2 as $type, 4 as $type, 8 as $type );
                let r = Vector3::new( 4 as $type, 8 as $type, 16 as $type );

                v *= 2.0 as $type;

                assert_eq!(v, r);

            }
        }
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
                let v = Vector3::new( 2 as $type, 4 as $type, 8 as $type );
                let r = Vector3::new( 1 as $type, 2 as $type, 4 as $type );

                assert_eq!(v / 2 as $type, r);

            }
        }
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
                let mut v = Vector3::new( 2 as $type, 4 as $type, 8 as $type );
                let r = Vector3::new( 1 as $type, 2 as $type, 4 as $type );

                v /= 2.0 as $type;

                assert_eq!(v, r);

            }
        }
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
                let v1 = Vector3::new( 1 as $type, 2 as $type, -3 as $type);
                let v2 = -v1;

                assert_eq!(v2.x, -1 as $type);
                assert_eq!(v2.y, -2 as $type);
                assert_eq!(v2.z, 3 as $type);
            }
        }
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
                let x_vec = Vector3::new( 2 as $type, 0 as $type, 0 as $type );
                let y_vec = Vector3::new( 0 as $type, 3 as $type, 0 as $type );
                let z_vec = Vector3::new( 0 as $type, 0 as $type, 4 as $type );

                assert_eq!(x_vec.magnitude(), 2 as $type);
                assert_eq!(y_vec.magnitude(), 3 as $type);
                assert_eq!(z_vec.magnitude(), 4 as $type);
            }
        }
    }

    vector3_magnitude! { f32, vector_magnitude_f32 }
    vector3_magnitude! { f64, vector_magnitude_f64 }

    macro_rules! vector3_normalize {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut x_vec = Vector3::new( 2 as $type, 0 as $type, 0 as $type );
                let mut y_vec = Vector3::new( 0 as $type, 3 as $type, 0 as $type );
                let mut z_vec = Vector3::new( 0 as $type, 0 as $type, 4 as $type );

                x_vec.normalize();
                y_vec.normalize();
                z_vec.normalize();

                assert_eq!(x_vec.magnitude(), 1 as $type);
                assert_eq!(y_vec.magnitude(), 1 as $type);
                assert_eq!(z_vec.magnitude(), 1 as $type);
            }
        }
    }

    vector3_normalize! { f32, vector3_normalize_f32 }
    vector3_normalize! { f64, vector3_normalize_f64 }

    macro_rules! vector3_normalized {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let x_vec = Vector3::new( 2 as $type, 0 as $type, 0 as $type );
                let y_vec = Vector3::new( 0 as $type, 3 as $type, 0 as $type );
                let z_vec = Vector3::new( 0 as $type, 0 as $type, 4 as $type );

                assert_eq!(x_vec.normalized().magnitude(), 1 as $type);
                assert_eq!(y_vec.normalized().magnitude(), 1 as $type);
                assert_eq!(z_vec.normalized().magnitude(), 1 as $type);
            }
        }
    }

    vector3_normalized! { f32, vector3_normalized_f32 }
    vector3_normalized! { f64, vector3_normalized_f64 }

    macro_rules! new_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p = Point3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(p.x, 1 as $type);
                assert_eq!(p.y, 2 as $type);
                assert_eq!(p.z, 3 as $type);
            }
        }
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
                let p1 = Point3::new( 1 as $type, 2 as $type, 3 as $type );

                let v1 = Vector3::new( 1 as $type, 0 as $type, 0 as $type );
                let v2 = Vector3::new( 0 as $type, 2 as $type, 0 as $type );
                let v3 = Vector3::new( 0 as $type, 0 as $type, 3 as $type );

                assert_eq!(p1 + v1, Point3::new(2 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 + v2, Point3::new(1 as $type, 4 as $type, 3 as $type));
                assert_eq!(p1 + v3, Point3::new(1 as $type, 2 as $type, 6 as $type));
            }
       }
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

    macro_rules! point3_sub_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point3::new( 1 as $type, 2 as $type, 3 as $type );

                let p2 = Point3::new( 1 as $type, 0 as $type, 0 as $type );
                let p3 = Point3::new( 0 as $type, 2 as $type, 0 as $type );
                let p4 = Point3::new( 0 as $type, 0 as $type, 3 as $type );

                assert_eq!(p1 - p2, Vector3::new(0 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 - p3, Vector3::new(1 as $type, 0 as $type, 3 as $type));
                assert_eq!(p1 - p4, Vector3::new(1 as $type, 2 as $type, 0 as $type));
            }
       }
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

    macro_rules! new_mat3x3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type,
                    4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type
                );

                assert_eq!(m.m11, 1 as $type);
                assert_eq!(m.m12, 2 as $type);
                assert_eq!(m.m13, 3 as $type);
                assert_eq!(m.m21, 4 as $type);
                assert_eq!(m.m22, 5 as $type);
                assert_eq!(m.m23, 6 as $type);
                assert_eq!(m.m31, 7 as $type);
                assert_eq!(m.m32, 8 as $type);
                assert_eq!(m.m33, 9 as $type);
            }
        }
    }

    new_mat3x3! { u8, new_mat3x3_u8 }
    new_mat3x3! { u16, new_mat3x3_u16 }
    new_mat3x3! { u32, new_mat3x3_u32 }
    new_mat3x3! { u64, new_mat3x3_u64 }
    new_mat3x3! { u128, new_mat3x3_u128 }
    new_mat3x3! { i8, new_mat3x3_i8 }
    new_mat3x3! { i16, new_mat3x3_i16 }
    new_mat3x3! { i32, new_mat3x3_i32 }
    new_mat3x3! { i64, new_mat3x3_i64 }
    new_mat3x3! { i128, new_mat3x3_i128 }
    new_mat3x3! { f32, new_mat3x3_f32 }
    new_mat3x3! { f64, new_mat3x3_f64 }
    
    macro_rules! mat3x3_change_column_1 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type,
                    4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type
                );

                let v = Vector3::new(10 as $type, 11 as $type, 12 as $type);

                let m = m.change_column_1(v);

                assert_eq!(m.m11, v.x);
                assert_eq!(m.m12, 2 as $type);
                assert_eq!(m.m13, 3 as $type);
                assert_eq!(m.m21, v.y);
                assert_eq!(m.m22, 5 as $type);
                assert_eq!(m.m23, 6 as $type);
                assert_eq!(m.m31, v.z);
                assert_eq!(m.m32, 8 as $type);
                assert_eq!(m.m33, 9 as $type);
            }
        }
    }

    mat3x3_change_column_1! { u8, mat3x3_change_column_1_u8 }
    mat3x3_change_column_1! { u16, mat3x3_change_column_1_u16 }
    mat3x3_change_column_1! { u32, mat3x3_change_column_1_u32 }
    mat3x3_change_column_1! { u64, mat3x3_change_column_1_u64 }
    mat3x3_change_column_1! { u128, mat3x3_change_column_1_u128 }
    mat3x3_change_column_1! { i8, mat3x3_change_column_1_i8 }
    mat3x3_change_column_1! { i16, mat3x3_change_column_1_i16 }
    mat3x3_change_column_1! { i32, mat3x3_change_column_1_i32 }
    mat3x3_change_column_1! { i64, mat3x3_change_column_1_i64 }
    mat3x3_change_column_1! { i128, mat3x3_change_column_1_i128 }
    mat3x3_change_column_1! { f32, mat3x3_change_column_1_f32 }
    mat3x3_change_column_1! { f64, mat3x3_change_column_1_f64 }
 
    macro_rules! mat3x3_change_column_2 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type,
                    4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type
                );

                let v = Vector3::new(10 as $type, 11 as $type, 12 as $type);

                let m = m.change_column_2(v);

                assert_eq!(m.m11, 1 as $type);
                assert_eq!(m.m12, v.x);
                assert_eq!(m.m13, 3 as $type);
                assert_eq!(m.m21, 4 as $type);
                assert_eq!(m.m22, v.y);
                assert_eq!(m.m23, 6 as $type);
                assert_eq!(m.m31, 7 as $type);
                assert_eq!(m.m32, v.z);
                assert_eq!(m.m33, 9 as $type);
            }
        }
    }

    mat3x3_change_column_2! { u8, mat3x3_change_column_2_u8 }
    mat3x3_change_column_2! { u16, mat3x3_change_column_2_u16 }
    mat3x3_change_column_2! { u32, mat3x3_change_column_2_u32 }
    mat3x3_change_column_2! { u64, mat3x3_change_column_2_u64 }
    mat3x3_change_column_2! { u128, mat3x3_change_column_2_u128 }
    mat3x3_change_column_2! { i8, mat3x3_change_column_2_i8 }
    mat3x3_change_column_2! { i16, mat3x3_change_column_2_i16 }
    mat3x3_change_column_2! { i32, mat3x3_change_column_2_i32 }
    mat3x3_change_column_2! { i64, mat3x3_change_column_2_i64 }
    mat3x3_change_column_2! { i128, mat3x3_change_column_2_i128 }
    mat3x3_change_column_2! { f32, mat3x3_change_column_2_f32 }
    mat3x3_change_column_2! { f64, mat3x3_change_column_2_f64 }
 
    macro_rules! mat3x3_change_column_3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type,
                    4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type
                );

                let v = Vector3::new(10 as $type, 11 as $type, 12 as $type);

                let m = m.change_column_3(v);

                assert_eq!(m.m11, 1 as $type);
                assert_eq!(m.m12, 2 as $type);
                assert_eq!(m.m13, v.x);
                assert_eq!(m.m21, 4 as $type);
                assert_eq!(m.m22, 5 as $type);
                assert_eq!(m.m23, v.y);
                assert_eq!(m.m31, 7 as $type);
                assert_eq!(m.m32, 8 as $type);
                assert_eq!(m.m33, v.z);
            }
        }
    }

    mat3x3_change_column_3! { u8, mat3x3_change_column_3_u8 }
    mat3x3_change_column_3! { u16, mat3x3_change_column_3_u16 }
    mat3x3_change_column_3! { u32, mat3x3_change_column_3_u32 }
    mat3x3_change_column_3! { u64, mat3x3_change_column_3_u64 }
    mat3x3_change_column_3! { u128, mat3x3_change_column_3_u128 }
    mat3x3_change_column_3! { i8, mat3x3_change_column_3_i8 }
    mat3x3_change_column_3! { i16, mat3x3_change_column_3_i16 }
    mat3x3_change_column_3! { i32, mat3x3_change_column_3_i32 }
    mat3x3_change_column_3! { i64, mat3x3_change_column_3_i64 }
    mat3x3_change_column_3! { i128, mat3x3_change_column_3_i128 }
    mat3x3_change_column_3! { f32, mat3x3_change_column_3_f32 }
    mat3x3_change_column_3! { f64, mat3x3_change_column_3_f64 }
}
