use std::ops;

use super::Intersect;
use super::ParametricLine;

use crate::math::Mat3x3;
use crate::math::Point3;
use crate::math::Vector3;
use crate::traits;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle<T> {
   a: T,
   b: T,
   c: T 
}

impl<T> Triangle<T> {
    pub fn new(a: T, b: T, c: T) -> Triangle<T> {
        Triangle { a, b, c }
    }
}

impl<T> Intersect<Triangle<Point3<T>>> for ParametricLine<Point3<T>, Vector3<T>> where
        T: ops::Add< Output = T>,
        T: ops::Sub< Output = T>,
        T: traits::Zero,
        T: traits::One,
        T: ops::Mul,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: PartialEq<T>,
        <T as ops::Mul>::Output: ops::Mul<T>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Add< Output = <<T as ops::Mul>::Output as ops::Mul<T>>::Output >,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Sub< Output = <<T as ops::Mul>::Output as ops::Mul<T>>::Output>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: ops::Div<Output = T>,
        <<T as ops::Mul>::Output as ops::Mul<T>>::Output: Copy + Clone,
        T: Copy + Clone + PartialOrd
{
    type Output = Vec<<<<T as ops::Mul>::Output as ops::Mul<T>>::Output as ops::Div>::Output >; 

    fn intersect(self, triangle: Triangle<Point3<T>>) -> Self::Output 
        {
        let m = Mat3x3::from_vector3s( triangle.a - triangle.b, triangle.a - triangle.c, self.direction );
        let v = triangle.a - self.origin;

        let m_determinante = m.determinant();
        
        if m_determinante == T::zero() {
            return vec! { }
        }

        let m1 = m.change_column_1(v);

        let beta = m1.determinant() / m_determinante;

        if beta < T::zero() || beta > T::one() {
            return vec! { }
        }

        let m2 = m.change_column_2(v);

        let gamma = m2.determinant() / m_determinante;

        if gamma < T::zero() || gamma > T::one() {
            return vec! { }
        }

        if beta + gamma < T::zero() || beta + gamma > T::one() {
            return vec! { }
        }

        let m3 = m.change_column_3(v);

        let t = m3.determinant() / m_determinante;

        vec! { t }
    }
}

#[cfg(tests)]
pub mod tests {
    use super::*;

    macro_rules! new_triangle {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = Point3::new( 1 as $type, 2 as $type, 3 as $type ); 
                let b = Point3::new( 4 as $type, 5 as $type, 6 as $type ); 
                let c = Point3::new( 7 as $type, 8 as $type, 9 as $type ); 

                let triangle = Triangle::new(a, b, c);

                assert_eq!(triangle.a, a);
                assert_eq!(triangle.b, b);
                assert_eq!(triangle.c, c);
            }
        }
    }

    new_triangle! { u8, new_triangle_u8 }
    new_triangle! { u16, new_triangle_u16 }
    new_triangle! { u32, new_triangle_u32 }
    new_triangle! { u64, new_triangle_u64 }
    new_triangle! { u128, new_triangle_u128 }
    new_triangle! { i8, new_triangle_i8 }
    new_triangle! { i16, new_triangle_i16 }
    new_triangle! { i32, new_triangle_i32 }
    new_triangle! { i64, new_triangle_i64 }
    new_triangle! { i128, new_triangle_i128 }
    new_triangle! { f32, new_triangle_f32 }
    new_triangle! { f64, new_triangle_f64 }

    macro_rules! parametric_line_intersect_triangle {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let triangle = Triangle::new(
                    Point3::new(-1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, 1 as $type, -2 as $type),
                    Point3::new(1 as $type, -1 as $type, -2 as $type)
                );

                let line1 = ParametricLine::new(
                    Point3::new(0 as $type, 0 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );


                let line2 = ParametricLine::new(
                    Point3::new(-1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line3 = ParametricLine::new(
                    Point3::new(1 as $type, 1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line4 = ParametricLine::new(
                    Point3::new(1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                let line5 = ParametricLine::new(
                    Point3::new(-1 as $type, -1 as $type, 0 as $type),
                    Vector3::new(0 as $type, 0 as $type, -1 as $type)
                );

                assert_eq!(line1.intersect(triangle),vec![2 as $type]);
                assert_eq!(line2.intersect(triangle),vec![2 as $type]);
                assert_eq!(line3.intersect(triangle),vec![2 as $type]);
                assert_eq!(line4.intersect(triangle),vec![2 as $type]);
                assert_eq!(line5.intersect(triangle),Vec::new());
            }
        }
    }

    parametric_line_intersect_triangle! { f32, parametric_line_intersect_triangle_f32 }
    parametric_line_intersect_triangle! { f64, parametric_line_intersect_triangle_f64 }
}
