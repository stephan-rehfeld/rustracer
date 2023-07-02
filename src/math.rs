pub mod geometry;

use std::ops;

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

impl<T: ops::Add<U>, U> ops::Add<Vector3<U>> for Vector3<T> {
    type Output = Vector3<<T as ops::Add<U>>::Output>;

    fn add(self, rhs: Vector3<U>) -> Self::Output {
        Vector3::new( self.x + rhs.x, self.y + rhs.y, self.z + rhs.z )
    }
}

impl<T: ops::Sub<U>, U> ops::Sub<Vector3<U>> for Vector3<T> {
    type Output = Vector3<<T as ops::Sub<U>>::Output>;

    fn sub(self, rhs: Vector3<U>) -> Self::Output {
        Vector3::new( self.x - rhs.x, self.y - rhs.y, self.z - rhs.z )
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

impl<T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy + Clone> ops::Mul for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: ops::Div<U> + Copy + Clone, U: Copy + Clone> ops::Div<U> for Vector3<T> {
    type Output = Vector3<<T as ops::Div<U>>::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: ops::DivAssign + Copy + Clone> ops::DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
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

impl Vector3<f32> {
    pub fn magnitude(self) -> f32 {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    pub fn normalized(self) -> Vector3<f32> {
        self / self.magnitude()
    }
}

impl Vector3<f64> {
    pub fn magnitude(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }
 
    pub fn normalized(self) -> Vector3<f64> {
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

impl<T: ops::Add<Output = T> + Clone + Copy> ops::Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
         Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: ops::Sub<Output = T> + Clone + Copy> ops::Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
                let xAxis = Vector3::new(1 as $type, 0 as $type, 0 as $type);
                let yAxis = Vector3::new(0 as $type, 1 as $type, 0 as $type);
                let zAxis = Vector3::new(0 as $type, 0 as $type, 1 as $type);

                assert_eq!(xAxis, Vector3::cross(yAxis, zAxis));
                assert_eq!(-xAxis, Vector3::cross(zAxis, yAxis));

                assert_eq!(yAxis, Vector3::cross(zAxis, xAxis));
                assert_eq!(-yAxis, Vector3::cross(xAxis, zAxis));

                assert_eq!(zAxis, Vector3::cross(xAxis, yAxis));
                assert_eq!(-zAxis, Vector3::cross(yAxis, xAxis));
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
    mul_scalar_vector3! { u64, mul__scalar_vector3_u64 }
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

    #[test]
    fn vector3_magnitude_f32() {
        let x_vec = Vector3::new( 2.0f64, 0.0f64, 0.0f64 );
        let y_vec = Vector3::new( 0.0f64, 3.0f64, 0.0f64 );
        let z_vec = Vector3::new( 0.0f64, 0.0f64, 4.0f64 );

        assert_eq!(x_vec.magnitude(), 2.0f64);
        assert_eq!(y_vec.magnitude(), 3.0f64);
        assert_eq!(z_vec.magnitude(), 4.0f64);
    }

    #[test]
    fn vector3_magnitude_f64() {
        let x_vec = Vector3::new( 2.0f64, 0.0f64, 0.0f64 );
        let y_vec = Vector3::new( 0.0f64, 3.0f64, 0.0f64 );
        let z_vec = Vector3::new( 0.0f64, 0.0f64, 4.0f64 );

        assert_eq!(x_vec.magnitude(), 2.0f64);
        assert_eq!(y_vec.magnitude(), 3.0f64);
        assert_eq!(z_vec.magnitude(), 4.0f64);
    }

    macro_rules! vector3_normalize {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let mut x_vec = Vector3::new( 2 as $type, 0 as $type, 0 as $type );
                let mut y_vec = Vector3::new( 0 as $type, 3 as $type, 0 as $type );
                let mut z_vec = Vector3::new( 0 as $type, 0 as $type, 4 as $type );

                &x_vec.normalize();
                &y_vec.normalize();
                &z_vec.normalize();

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

    #[test]
    fn new_point3_f32() {
        let p = Point3::new( 1.0f32, 2.0f32, 3.0f32 );

        assert_eq!(p.x, 1.0f32);
        assert_eq!(p.y, 2.0f32);
        assert_eq!(p.z, 3.0f32);
    }

    #[test]
    fn new_point3_f64() {
        let p = Point3::new( 1.0f64, 2.0f64, 3.0f64 );

        assert_eq!(p.x, 1.0f64);
        assert_eq!(p.y, 2.0f64);
        assert_eq!(p.z, 3.0f64);
    }

    #[test]
    fn point3_sub_point3_f32() {
        let p1 = Point3::new( 1.0f32, 2.0f32, 3.0f32 );

        let p2 = Point3::new( 1.0f32, 0.0f32, 0.0f32 );
        let p3 = Point3::new( 0.0f32, 2.0f32, 0.0f32 );
        let p4 = Point3::new( 0.0f32, 0.0f32, 3.0f32 );

        assert_eq!(p1 - p2, Vector3::new(0.0f32, 2.0f32, 3.0f32));
        assert_eq!(p1 - p3, Vector3::new(1.0f32, 0.0f32, 3.0f32));
        assert_eq!(p1 - p4, Vector3::new(1.0f32, 2.0f32, 0.0f32));
    }

    #[test]
    fn point3_sub_point3_f64() {
        let p1 = Point3::new( 1.0f64, 2.0f64, 3.0f64 );

        let p2 = Point3::new( 1.0f64, 0.0f64, 0.0f64 );
        let p3 = Point3::new( 0.0f64, 2.0f64, 0.0f64 );
        let p4 = Point3::new( 0.0f64, 0.0f64, 3.0f64 );

        assert_eq!(p1 - p2, Vector3::new(0.0f64, 2.0f64, 3.0f64));
        assert_eq!(p1 - p3, Vector3::new(1.0f64, 0.0f64, 3.0f64));
        assert_eq!(p1 - p4, Vector3::new(1.0f64, 2.0f64, 0.0f64));
    }

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
}
