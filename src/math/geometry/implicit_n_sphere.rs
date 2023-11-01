use std::ops;

use super::Intersect;
use super::ParametricLine;

use crate::math::Point;
use crate::math::Vector;
use crate::math::vector::DotProduct;
use crate::traits::Sqrt; 
use crate::traits::Zero;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImplicitNSphere<P> where P: Point, <P as Point>::ValueType: Copy + Clone + PartialEq + std::fmt::Debug {
    center: P,
    radius: <P as Point>::ValueType,
}

impl<P> ImplicitNSphere<P> where P: Point, <P as Point>::ValueType: Copy + Clone + PartialEq + std::fmt::Debug {
    pub fn new(center: P, radius: <P as Point>::ValueType) -> ImplicitNSphere<P> {
        ImplicitNSphere { center, radius }
    }

    pub fn test(self, point: P) -> <<P as Point>::ValueType as ops::Mul>::Output  where 
        P: ops::Sub,
        <P as ops::Sub>::Output: DotProduct<<P as ops::Sub>::Output, ValueType=<P as Point>::ValueType > + Copy + Clone,
        <P as Point>::ValueType: ops::Mul,
        <<P as Point>::ValueType as ops::Mul>::Output: ops::Sub<Output=<<P as Point>::ValueType as ops::Mul>::Output>,
    {
        let d = point - self.center;
        d.dot(d) - self.radius * self.radius
    }
}

impl<P, V> Intersect<ImplicitNSphere<P>> for ParametricLine<P, V>
where
    V: DotProduct<V> + ops::Add<Output=V> + Copy + Clone,
    <V as Vector>::ValueType: ops::Mul + Copy + Clone,
    <<V as Vector>::ValueType as ops::Mul>::Output: ops::Sub< <<P as Point>::ValueType as ops::Mul>::Output  ,Output=<<V as Vector>::ValueType as ops::Mul>::Output> +  ops::Sub<Output=<<V as Vector>::ValueType as ops::Mul>::Output> + ops::Div + ops::Mul + ops::Neg<Output=<<V as Vector>::ValueType as ops::Mul>::Output> + ops::Add<Output=<<V as Vector>::ValueType as ops::Mul>::Output> + Copy + Clone,
    <<<V as Vector>::ValueType as ops::Mul>::Output as ops::Mul>::Output: ops::Add<Output=<<<V as Vector>::ValueType as ops::Mul>::Output as ops::Mul>::Output> + ops::Sub<Output=<<<V as Vector>::ValueType as ops::Mul>::Output as ops::Mul>::Output> + PartialOrd + PartialEq + Zero + Sqrt<Output=<<V as Vector>::ValueType as ops::Mul>::Output>,
    P: Point,
    <P as Point>::ValueType: ops::Mul + Copy + Clone + PartialEq + std::fmt::Debug,
    P: ops::Sub<Output=V> + Copy + Clone,
{
    type Output = Vec<<<<V as Vector>::ValueType as ops::Mul>::Output as ops::Div>::Output>;

    fn intersect(self, sphere: ImplicitNSphere<P>) -> Self::Output {
        let a = self.direction.dot(self.direction);
        let b = self.direction.dot((self.origin - sphere.center) + (self.origin - sphere.center));
        let c = (self.origin - sphere.center).dot(self.origin - sphere.center) - sphere.radius * sphere.radius;

        let helper = b * b - (a * c + a * c + a * c + a * c);
        
        if helper < Zero::zero() {
            Vec::new()
        } else if helper == Zero::zero() {
            vec![ (-b / (a + a) ) ]
        } else {
            let helper = helper.sqrt();
            vec![ ((-b - helper) / (a + a) ), ((-b + helper) / (a + a) ) ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::Vector3;
    use crate::math::Point3;

    macro_rules! new_implicit_3_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let radius = 4 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_eq!(sphere.center, center);
                assert_eq!(sphere.radius, radius);
            }
        }
    }

    new_implicit_3_sphere! { u8, new_implicit_3_sphere_u8 }
    new_implicit_3_sphere! { u16, new_implicit_3_sphere_u16 }
    new_implicit_3_sphere! { u32, new_implicit_3_sphere_u32 }
    new_implicit_3_sphere! { u64, new_implicit_3_sphere_u64 }
    new_implicit_3_sphere! { u128, new_implicit_3_sphere_u128 }
    new_implicit_3_sphere! { i8, new_implicit_3_sphere_i8 }
    new_implicit_3_sphere! { i16, new_implicit_3_sphere_i16 }
    new_implicit_3_sphere! { i32, new_implicit_3_sphere_i32 }
    new_implicit_3_sphere! { i64, new_implicit_3_sphere_i64 }
    new_implicit_3_sphere! { i128, new_implicit_3_sphere_i128 }
    new_implicit_3_sphere! { f32, new_implicit_3_sphere_f32 }
    new_implicit_3_sphere! { f64, new_implicit_3_sphere_f64 }

    macro_rules! implicit_3_sphere_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let center = Point3::new( 2 as $type, 2 as $type, 2 as $type );
                let radius = 2 as $type;

                let sphere = ImplicitNSphere::new(center, radius);

                assert_ne!(sphere.test(Point3::new( 2 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 0 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 4 as $type, 2 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 0 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 4 as $type, 2 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 2 as $type, 0 as $type )), 0 as $type);
                assert_eq!(sphere.test(Point3::new( 2 as $type, 2 as $type, 4 as $type )), 0 as $type);
            }
        }
    }

    implicit_3_sphere_test! { i8, implicit_3_sphere_test_i8 }
    implicit_3_sphere_test! { i16, implicit_3_sphere_test_i16 }
    implicit_3_sphere_test! { i32, implicit_3_sphere_test_i32 }
    implicit_3_sphere_test! { i64, implicit_3_sphere_test_i64 }
    implicit_3_sphere_test! { i128, implicit_3_sphere_test_i128 }
    implicit_3_sphere_test! { f32, implicit_3_sphere_test_f32 }
    implicit_3_sphere_test! { f64, implicit_3_sphere_test_f64 }

    macro_rules! parametric_line_intersect_implicit_3_sphere {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ray1 = ParametricLine::new(
                    Point3::new(4 as $type, 4 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray2 = ParametricLine::new(
                    Point3::new(1 as $type, 3 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );
                
                let ray3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 4 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let sphere = ImplicitNSphere::new(
                    Point3::new(1 as $type, 1 as $type, 1 as $type),
                    2 as $type
                );

                assert_eq!(ray1.intersect(sphere), Vec::new());
                assert_eq!(ray2.intersect(sphere), vec![3 as $type]);
                assert_eq!(ray3.intersect(sphere), vec![1 as $type, 5 as $type]);
            }
        }
    }

    parametric_line_intersect_implicit_3_sphere! { f32, parametric_line_intersect_implicit_3_sphere_f32 }
    parametric_line_intersect_implicit_3_sphere! { f64, parametric_line_intersect_implicit_3_sphere_f64 }
}
