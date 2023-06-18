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
}

impl<T: ops::Mul<Output = T> + Clone + Copy> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new( self.x * rhs, self.y * rhs, self.z * rhs )
    }
}

impl<T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy + Clone> ops::Mul for Vector3<T> {
    type Output = T;

    fn mul<'a, 'b>(self, rhs: Vector3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: ops::Div<Output = T> + Copy + Clone> ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
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

impl Vector3<f32> {
    pub fn magnitude(self) -> f32 {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }
}

impl Vector3<f64> {
    pub fn magnitude(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
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

    #[test]
    fn new_vector3_f32() {
        let vec = Vector3::new( 1.0f32, 2.0f32, 3.0f32 );

        assert_eq!(vec.x, 1.0f32);
        assert_eq!(vec.y, 2.0f32);
        assert_eq!(vec.z, 3.0f32);
    }

    #[test]
    fn new_vector3_f64() {
        let vec = Vector3::new( 1.0f64, 2.0f64, 3.0f64 );

        assert_eq!(vec.x, 1.0f64);
        assert_eq!(vec.y, 2.0f64);
        assert_eq!(vec.z, 3.0f64);
    }

    #[test]
    fn dot_product_vector3_f32() {
        let x_vec1 = Vector3::new( 1.0f32, 0.0f32, 0.0f32 );
        let y_vec1 = Vector3::new( 0.0f32, 1.0f32, 0.0f32 );
        let z_vec1 = Vector3::new( 0.0f32, 0.0f32, 1.0f32 );

        assert_eq!(x_vec1 * y_vec1, 0.0f32);
        assert_eq!(x_vec1 * z_vec1, 0.0f32);
        assert_eq!(y_vec1 * z_vec1, 0.0f32);

        let x_vec2 = x_vec1 * 2.0f32;
        let y_vec2 = y_vec1 * 2.0f32;
        let z_vec2 = x_vec1 * 2.0f32;

        let x_vec3 = x_vec1 * 3.0f32;
        let y_vec3 = y_vec1 * 3.0f32;
        let z_vec3 = x_vec1 * 3.0f32;

        assert_eq!(x_vec2 * x_vec3, 6.0f32); 
        assert_eq!(y_vec2 * y_vec3, 6.0f32); 
        assert_eq!(z_vec2 * z_vec3, 6.0f32); 
    }

    #[test]
    fn dot_product_vector3_f64() {
        let x_vec1 = Vector3::new( 1.0f64, 0.0f64, 0.0f64 );
        let y_vec1 = Vector3::new( 0.0f64, 1.0f64, 0.0f64 );
        let z_vec1 = Vector3::new( 0.0f64, 0.0f64, 1.0f64 );

        assert_eq!(x_vec1 * y_vec1, 0.0f64);
        assert_eq!(x_vec1 * z_vec1, 0.0f64);
        assert_eq!(y_vec1 * z_vec1, 0.0f64);

        let x_vec2 = x_vec1 * 2.0f64;
        let y_vec2 = y_vec1 * 2.0f64;
        let z_vec2 = x_vec1 * 2.0f64;

        let x_vec3 = x_vec1 * 3.0f64;
        let y_vec3 = y_vec1 * 3.0f64;
        let z_vec3 = x_vec1 * 3.0f64;

        assert_eq!(x_vec2 * x_vec3, 6.0f64);
        assert_eq!(y_vec2 * y_vec3, 6.0f64);
        assert_eq!(z_vec2 * z_vec3, 6.0f64);
    }

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
}
