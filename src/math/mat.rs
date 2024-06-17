use std::ops::{Add, Mul, Sub};

use crate::math::{Normal3, Point3, Vector3};
use crate::traits::{One, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat3x3<T> {
    m11: T,
    m12: T,
    m13: T,
    m21: T,
    m22: T,
    m23: T,
    m31: T,
    m32: T,
    m33: T,
}

impl<T> Mat3x3<T> {
    pub fn new(
        m11: T,
        m12: T,
        m13: T,
        m21: T,
        m22: T,
        m23: T,
        m31: T,
        m32: T,
        m33: T,
    ) -> Mat3x3<T> {
        Mat3x3 {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        }
    }

    pub fn from_vector3s(col1: Vector3<T>, col2: Vector3<T>, col3: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new(
            col1.x, col2.x, col3.x, col1.y, col2.y, col3.y, col1.z, col2.z, col3.z,
        )
    }

    pub fn change_column_1(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new(
            v.x, self.m12, self.m13, v.y, self.m22, self.m23, v.z, self.m32, self.m33,
        )
    }

    pub fn change_column_2(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new(
            self.m11, v.x, self.m13, self.m21, v.y, self.m23, self.m31, v.z, self.m33,
        )
    }

    pub fn change_column_3(self, v: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new(
            self.m11, self.m12, v.x, self.m21, self.m22, v.y, self.m31, self.m32, v.z,
        )
    }

    pub fn determinant(self) -> <<T as Mul>::Output as Mul<T>>::Output
    where
        T: Mul + Copy + Clone,
        <T as Mul>::Output: Mul<T>,
        <<T as Mul>::Output as Mul<T>>::Output:
            Add<Output = <<T as Mul>::Output as Mul<T>>::Output>,
        <<T as Mul>::Output as Mul<T>>::Output:
            Sub<Output = <<T as Mul>::Output as Mul<T>>::Output>,
    {
        self.m11 * self.m22 * self.m33
            + self.m12 * self.m23 * self.m31
            + self.m13 * self.m21 * self.m32
            - self.m31 * self.m22 * self.m13
            - self.m32 * self.m23 * self.m11
            - self.m33 * self.m21 * self.m12
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4x4<T> {
    m11: T,
    m12: T,
    m13: T,
    m14: T,
    m21: T,
    m22: T,
    m23: T,
    m24: T,
    m31: T,
    m32: T,
    m33: T,
    m34: T,
    m41: T,
    m42: T,
    m43: T,
    m44: T,
}

impl<T> Mat4x4<T> {
    pub fn new(
        m11: T,
        m12: T,
        m13: T,
        m14: T,
        m21: T,
        m22: T,
        m23: T,
        m24: T,
        m31: T,
        m32: T,
        m33: T,
        m34: T,
        m41: T,
        m42: T,
        m43: T,
        m44: T,
    ) -> Mat4x4<T> {
        Mat4x4 {
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
            m41,
            m42,
            m43,
            m44,
        }
    }
}

impl<T: One + Zero> Mat4x4<T> {
    pub fn ident() -> Mat4x4<T> {
        Mat4x4::new(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        )
    }
}

impl<T> Mat4x4<T> {
    pub fn transposed(self) -> Mat4x4<T> {
        Mat4x4::new(
            self.m11, self.m21, self.m31, self.m41, self.m12, self.m22, self.m32, self.m42,
            self.m13, self.m23, self.m33, self.m43, self.m14, self.m24, self.m34, self.m44,
        )
    }
}

impl<T, U> Mul<Vector3<U>> for Mat4x4<T>
where
    U: Copy,
    T: Mul<U> + Copy,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    type Output = Vector3<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector3<U>) -> Self::Output {
        Vector3::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m21 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m31 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}

impl<T, U> Mul<Normal3<U>> for Mat4x4<T>
where
    U: Copy,
    T: Mul<U> + Copy,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    type Output = Normal3<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Normal3<U>) -> Self::Output {
        Normal3::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m21 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m31 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}

impl<T, U> Mul<Point3<U>> for Mat4x4<T>
where
    T: Mul<U> + Copy,
    U: One + Copy,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    type Output = Point3<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Point3<U>) -> Self::Output {
        Point3::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z + self.m14 * One::one(),
            self.m21 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z + self.m24 * One::one(),
            self.m31 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z + self.m34 * One::one(),
        )
    }
}

impl<T> Mul for Mat4x4<T>
where
    T: Mul + Copy,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>,
{
    type Output = Mat4x4<<T as Mul>::Output>;

    fn mul(self, rhs: Mat4x4<T>) -> Self::Output {
        let m11 = self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41;
        let m12 = self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42;
        let m13 = self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43;
        let m14 = self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44;

        let m21 = self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41;
        let m22 = self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42;
        let m23 = self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43;
        let m24 = self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44;

        let m31 = self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41;
        let m32 = self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42;
        let m33 = self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43;
        let m34 = self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44;

        let m41 = self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41;
        let m42 = self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42;
        let m43 = self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43;
        let m44 = self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44;

        Mat4x4::new(
            m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_mat3x3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type, 4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type,
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
        };
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

    // Transposed

    macro_rules! mat3x3_from_vector3s {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::from_vector3s(
                    Vector3::new(1 as $type, 4 as $type, 7 as $type),
                    Vector3::new(2 as $type, 5 as $type, 8 as $type),
                    Vector3::new(3 as $type, 6 as $type, 9 as $type),
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
        };
    }

    mat3x3_from_vector3s! { u8, mat3x3_from_vector3s_u8 }
    mat3x3_from_vector3s! { u16, mat3x3_from_vector3s_u16 }
    mat3x3_from_vector3s! { u32, mat3x3_from_vector3s_u32 }
    mat3x3_from_vector3s! { u64, mat3x3_from_vector3s_u64 }
    mat3x3_from_vector3s! { u128, mat3x3_from_vector3s_u128 }
    mat3x3_from_vector3s! { i8, mat3x3_from_vector3s_i8 }
    mat3x3_from_vector3s! { i16, mat3x3_from_vector3s_i16 }
    mat3x3_from_vector3s! { i32, mat3x3_from_vector3s_i32 }
    mat3x3_from_vector3s! { i64, mat3x3_from_vector3s_i64 }
    mat3x3_from_vector3s! { i128, mat3x3_from_vector3s_i128 }
    mat3x3_from_vector3s! { f32, mat3x3_from_vector3s_f32 }
    mat3x3_from_vector3s! { f64, mat3x3_from_vector3s_f64 }

    macro_rules! mat3x3_change_column_1 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type, 4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type,
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
        };
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
                    1 as $type, 2 as $type, 3 as $type, 4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type,
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
        };
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
                    1 as $type, 2 as $type, 3 as $type, 4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type,
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
        };
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

    macro_rules! mat3x3_determinant {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m1 = Mat3x3::new(
                    1 as $type, 0 as $type, 0 as $type, 0 as $type, 1 as $type, 0 as $type,
                    0 as $type, 0 as $type, 1 as $type,
                );

                let m2 = Mat3x3::new(
                    1 as $type, 0 as $type, 0 as $type, 0 as $type, 1 as $type, 0 as $type,
                    0 as $type, 0 as $type, 0 as $type,
                );

                let m3 = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type, 4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type,
                );

                let m4 = Mat3x3::new(
                    0 as $type, 0 as $type, 1 as $type, 0 as $type, 1 as $type, 0 as $type,
                    1 as $type, 0 as $type, 0 as $type,
                );

                assert_eq!(m1.determinant(), 1 as $type);
                assert_eq!(m2.determinant(), 0 as $type);
                assert_eq!(m3.determinant(), 0 as $type);
                assert_eq!(m4.determinant(), -1 as $type);
            }
        };
    }

    mat3x3_determinant! { i16, mat3x3_determinant_i16 }
    mat3x3_determinant! { i32, mat3x3_determinant_i32 }
    mat3x3_determinant! { i64, mat3x3_determinant_i64 }
    mat3x3_determinant! { i128, mat3x3_determinant_i128 }
    mat3x3_determinant! { f32, mat3x3_determinant_f32 }
    mat3x3_determinant! { f64, mat3x3_determinant_f64 }

    macro_rules! new_mat4x4 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat4x4::new(
                    1 as $type,
                    2 as $type,
                    3 as $type,
                    4 as $type,
                    5 as $type,
                    6 as $type,
                    7 as $type,
                    8 as $type,
                    9 as $type,
                    10 as $type,
                    11 as $type,
                    12 as $type,
                    13 as $type,
                    14 as $type,
                    15 as $type,
                    16 as $type,
                );

                assert_eq!(m.m11, 1 as $type);
                assert_eq!(m.m12, 2 as $type);
                assert_eq!(m.m13, 3 as $type);
                assert_eq!(m.m14, 4 as $type);
                assert_eq!(m.m21, 5 as $type);
                assert_eq!(m.m22, 6 as $type);
                assert_eq!(m.m23, 7 as $type);
                assert_eq!(m.m24, 8 as $type);
                assert_eq!(m.m31, 9 as $type);
                assert_eq!(m.m32, 10 as $type);
                assert_eq!(m.m33, 11 as $type);
                assert_eq!(m.m34, 12 as $type);
                assert_eq!(m.m41, 13 as $type);
                assert_eq!(m.m42, 14 as $type);
                assert_eq!(m.m43, 15 as $type);
                assert_eq!(m.m44, 16 as $type);
            }
        };
    }

    new_mat4x4! { u8, new_mat4x4_u8 }
    new_mat4x4! { u16, new_mat4x4_u16 }
    new_mat4x4! { u32, new_mat4x4_u32 }
    new_mat4x4! { u64, new_mat4x4_u64 }
    new_mat4x4! { u128, new_mat4x4_u128 }
    new_mat4x4! { i8, new_mat4x4_i8 }
    new_mat4x4! { i16, new_mat4x4_i16 }
    new_mat4x4! { i32, new_mat4x4_i32 }
    new_mat4x4! { i64, new_mat4x4_i64 }
    new_mat4x4! { i128, new_mat4x4_i128 }
    new_mat4x4! { f32, new_mat4x4_f32 }
    new_mat4x4! { f64, new_mat4x4_f64 }

    macro_rules! mat4x4_mul_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat4x4::new(
                    1 as $type,
                    2 as $type,
                    3 as $type,
                    4 as $type,
                    5 as $type,
                    6 as $type,
                    7 as $type,
                    8 as $type,
                    9 as $type,
                    10 as $type,
                    11 as $type,
                    12 as $type,
                    13 as $type,
                    14 as $type,
                    15 as $type,
                    16 as $type,
                );

                let v = Vector3::new(1 as $type, 2 as $type, 3 as $type);

                let expected = Vector3::new(14 as $type, 38 as $type, 62 as $type);

                assert_eq!(m * v, expected);
            }
        };
    }

    mat4x4_mul_vector3! { u8, mat4x4_mul_vector3_u8 }
    mat4x4_mul_vector3! { u16, mat4x4_mul_vector3_u16 }
    mat4x4_mul_vector3! { u32, mat4x4_mul_vector3_u32 }
    mat4x4_mul_vector3! { u64, mat4x4_mul_vector3_u64 }
    mat4x4_mul_vector3! { u128, mat4x4_mul_vector3_u128 }
    mat4x4_mul_vector3! { i8, mat4x4_mul_vector3_i8 }
    mat4x4_mul_vector3! { i16, mat4x4_mul_vector3_i16 }
    mat4x4_mul_vector3! { i32, mat4x4_mul_vector3_i32 }
    mat4x4_mul_vector3! { i64, mat4x4_mul_vector3_i64 }
    mat4x4_mul_vector3! { i128, mat4x4_mul_vector3_i128 }
    mat4x4_mul_vector3! { f32, mat4x4_mul_vector3_f32 }
    mat4x4_mul_vector3! { f64, mat4x4_mul_vector3_f64 }

    macro_rules! mat4x4_mul_normal3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat4x4::new(
                    1 as $type,
                    2 as $type,
                    3 as $type,
                    4 as $type,
                    5 as $type,
                    6 as $type,
                    7 as $type,
                    8 as $type,
                    9 as $type,
                    10 as $type,
                    11 as $type,
                    12 as $type,
                    13 as $type,
                    14 as $type,
                    15 as $type,
                    16 as $type,
                );

                let v = Normal3::new(1 as $type, 2 as $type, 3 as $type);

                let expected = Normal3::new(14 as $type, 38 as $type, 62 as $type);

                assert_eq!(m * v, expected);
            }
        };
    }

    mat4x4_mul_normal3! { u8, mat4x4_mul_normal3_u8 }
    mat4x4_mul_normal3! { u16, mat4x4_mul_normal3_u16 }
    mat4x4_mul_normal3! { u32, mat4x4_mul_normal3_u32 }
    mat4x4_mul_normal3! { u64, mat4x4_mul_normal3_u64 }
    mat4x4_mul_normal3! { u128, mat4x4_mul_normal3_u128 }
    mat4x4_mul_normal3! { i8, mat4x4_mul_normal3_i8 }
    mat4x4_mul_normal3! { i16, mat4x4_mul_normal3_i16 }
    mat4x4_mul_normal3! { i32, mat4x4_mul_normal3_i32 }
    mat4x4_mul_normal3! { i64, mat4x4_mul_normal3_i64 }
    mat4x4_mul_normal3! { i128, mat4x4_mul_normal3_i128 }
    mat4x4_mul_normal3! { f32, mat4x4_mul_normal3_f32 }
    mat4x4_mul_normal3! { f64, mat4x4_mul_normal3_f64 }

    macro_rules! mat4x4_mul_point3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m = Mat4x4::new(
                    1 as $type,
                    2 as $type,
                    3 as $type,
                    4 as $type,
                    5 as $type,
                    6 as $type,
                    7 as $type,
                    8 as $type,
                    9 as $type,
                    10 as $type,
                    11 as $type,
                    12 as $type,
                    13 as $type,
                    14 as $type,
                    15 as $type,
                    16 as $type,
                );

                let v = Point3::new(1 as $type, 2 as $type, 3 as $type);

                let expected = Point3::new(18 as $type, 46 as $type, 74 as $type);

                assert_eq!(m * v, expected);
            }
        };
    }

    mat4x4_mul_point3! { u8, mat4x4_mul_point3_u8 }
    mat4x4_mul_point3! { u16, mat4x4_mul_point3_u16 }
    mat4x4_mul_point3! { u32, mat4x4_mul_point3_u32 }
    mat4x4_mul_point3! { u64, mat4x4_mul_point3_u64 }
    mat4x4_mul_point3! { u128, mat4x4_mul_point3_u128 }
    mat4x4_mul_point3! { i8, mat4x4_mul_point3_i8 }
    mat4x4_mul_point3! { i16, mat4x4_mul_point3_i16 }
    mat4x4_mul_point3! { i32, mat4x4_mul_point3_i32 }
    mat4x4_mul_point3! { i64, mat4x4_mul_point3_i64 }
    mat4x4_mul_point3! { i128, mat4x4_mul_point3_i128 }
    mat4x4_mul_point3! { f32, mat4x4_mul_point3_f32 }
    mat4x4_mul_point3! { f64, mat4x4_mul_point3_f64 }

    // mat4x4 mul mat4x4
}
