use std::ops;

use crate::math::Vector3;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Mat3x3<T> {
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
}

impl<T> Mat3x3<T> {
    pub fn new(m11: T, m12: T, m13: T,
               m21: T, m22: T, m23: T,
               m31: T, m32: T, m33: T ) -> Mat3x3<T> {
        Mat3x3 { m11, m12, m13,
                 m21, m22, m23,
                 m31, m32, m33 }
    }

    pub fn from_vector3s(col1: Vector3<T>, col2: Vector3<T>, col3: Vector3<T>) -> Mat3x3<T> {
        Mat3x3::new( col1.x, col2.x, col3.x,
                     col1.y, col2.y, col3.y,
                     col1.z, col2.z, col3.z )

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

    pub fn determinant(self) -> <<T as ops::Mul>::Output as ops::Mul<T>>::Output where
        T: ops::Mul + Copy + Clone,
        <T as ops::Mul>::Output: ops::Mul<T>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Add<Output=<<T as ops::Mul>::Output as ops::Mul<T>>::Output>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Sub<Output=<<T as ops::Mul>::Output as ops::Mul<T>>::Output>,
    {
        self.m11 * self.m22 * self.m33 + self.m12 * self.m23 * self.m31 + self.m13 * self.m21 * self.m32 - self.m31 * self.m22 * self.m13 - self.m32 * self.m23 * self.m11 - self.m33 * self.m21 * self.m12    
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

    macro_rules! mat3x3_from_vector3s {
        ($type: ty, $name: ident) => { 
            #[test]
            fn $name() {
                let m = Mat3x3::from_vector3s( Vector3::new(1 as $type, 4 as $type, 7 as $type),
                                               Vector3::new(2 as $type, 5 as $type, 8 as $type),
                                               Vector3::new(3 as $type, 6 as $type, 9 as $type)
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

    macro_rules! mat3x3_determinant {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let m1 = Mat3x3::new(
                    1 as $type, 0 as $type, 0 as $type,
                    0 as $type, 1 as $type, 0 as $type,
                    0 as $type, 0 as $type, 1 as $type
                );

                let m2 = Mat3x3::new(
                    1 as $type, 0 as $type, 0 as $type,
                    0 as $type, 1 as $type, 0 as $type,
                    0 as $type, 0 as $type, 0 as $type
                );

                let m3 = Mat3x3::new(
                    1 as $type, 2 as $type, 3 as $type,
                    4 as $type, 5 as $type, 6 as $type,
                    7 as $type, 8 as $type, 9 as $type
                );

                let m4 = Mat3x3::new(
                    0 as $type, 0 as $type, 1 as $type,
                    0 as $type, 1 as $type, 0 as $type,
                    1 as $type, 0 as $type, 0 as $type
                );


                assert_eq!(m1.determinant(), 1 as $type);
                assert_eq!(m2.determinant(), 0 as $type);
                assert_eq!(m3.determinant(), 0 as $type);
                assert_eq!(m4.determinant(), -1 as $type);
            }
        }
    }
    
    mat3x3_determinant! { i16, mat3x3_determinant_i16 }
    mat3x3_determinant! { i32, mat3x3_determinant_i32 }
    mat3x3_determinant! { i64, mat3x3_determinant_i64 }
    mat3x3_determinant! { i128, mat3x3_determinant_i128 }
    mat3x3_determinant! { f32, mat3x3_determinant_f32 }
    mat3x3_determinant! { f64, mat3x3_determinant_f64 }
}
