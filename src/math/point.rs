use std::ops;
use crate::math::vector::Vector3;

pub trait Point {
    type ValueType;
    type VectorType;
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Point3<T> {
    pub(super) x: T,
    pub(super) y: T,
    pub(super) z: T
}

impl<T> Point for Point3<T> {
    type ValueType = T;
    type VectorType = Vector3<T>;
}

impl<T> Point3<T> {
    pub fn new( x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }
}

impl<T: ops::Add<U>, U> ops::Add<Vector3<U>> for Point3<T> {
    type Output = Point3<<T as ops::Add<U>>::Output>;

    fn add(self, rhs: Vector3<U>) -> Self::Output {
         Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: ops::AddAssign<U>, U> ops::AddAssign<Vector3<U>> for Point3<T> {
    fn add_assign(&mut self, rhs: Vector3<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: ops::Sub<U>, U> ops::Sub<Point3<U>> for Point3<T> {
    type Output = Vector3<<T as ops::Sub<U>>::Output>;

    fn sub(self, rhs: Point3<U>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: ops::Sub<U>, U> ops::Sub<Vector3<U>> for Point3<T> {
    type Output = Point3<<T as ops::Sub<U>>::Output>;

    fn sub(self, rhs: Vector3<U>) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: ops::SubAssign<U>, U> ops::SubAssign<Vector3<U>> for Point3<T> {
    fn sub_assign(&mut self, rhs: Vector3<U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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

    macro_rules! point3_add_assign_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut p = Point3::new( 1 as $type, 2 as $type, 3 as $type );

                p += Vector3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(p, Point3::new(2 as $type, 4 as $type, 6 as $type));
            }
        }
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
    
    macro_rules! point3_sub_vector3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let p1 = Point3::new( 1 as $type, 2 as $type, 3 as $type );

                let v1 = Vector3::new( 1 as $type, 0 as $type, 0 as $type );
                let v2 = Vector3::new( 0 as $type, 2 as $type, 0 as $type );
                let v3 = Vector3::new( 0 as $type, 0 as $type, 3 as $type );

                assert_eq!(p1 - v1, Point3::new(0 as $type, 2 as $type, 3 as $type));
                assert_eq!(p1 - v2, Point3::new(1 as $type, 0 as $type, 3 as $type));
                assert_eq!(p1 - v3, Point3::new(1 as $type, 2 as $type, 0 as $type));
            }
        }
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
                let mut p = Point3::new( 2 as $type, 4 as $type, 6 as $type );

                p -= Vector3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(p, Point3::new(1 as $type, 2 as $type, 3 as $type));
            }
        }
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
