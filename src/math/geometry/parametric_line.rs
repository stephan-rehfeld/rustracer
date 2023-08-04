use std::ops;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ParametricLine<P,V> {
    pub(super) origin: P,
    pub(super) direction: V
}

impl<P,V> ParametricLine<P,V> {
    pub fn new( origin: P, direction: V ) -> ParametricLine<P,V> {
        ParametricLine { origin, direction }
    }

    pub fn at<T>(self, t: T) -> <P as ops::Add<<V as ops::Mul<T>>::Output>>::Output
    where
        V: ops::Mul<T>,
        P: ops::Add<<V as ops::Mul<T>>::Output>

    {
       self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::Vector3;
    use crate::math::Point3;

    macro_rules! new_parametric_line {
        ( $type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let origin = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let direction = Vector3::new( 4 as $type, 5 as $type, 6 as $type );

                let ray = ParametricLine::new(origin, direction);

                assert_eq!(ray.origin, origin);
                assert_eq!(ray.direction, direction);
            }
        }
    }

    new_parametric_line! { u8, new_parametric_line_3_u8 }
    new_parametric_line! { u16, new_parametric_line_3_u16 }
    new_parametric_line! { u32, new_parametric_line_3_u32 }
    new_parametric_line! { u64, new_parametric_line_3_u64 }
    new_parametric_line! { u128, new_parametric_line_3_u128 }
    new_parametric_line! { i8, new_parametric_line_3_i8 }
    new_parametric_line! { i16, new_parametric_line_3_i16 }
    new_parametric_line! { i32, new_parametric_line_3_i32 }
    new_parametric_line! { i64, new_parametric_line_3_i64 }
    new_parametric_line! { i128, new_parametric_line_3_i128 }
    new_parametric_line! { f32, new_parametric_line_3_f32 }
    new_parametric_line! { f64, new_parametric_line_3_f64 }

    macro_rules! parametric_line_3_at {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let origin = Point3::new( 1 as $type, 2 as $type, 3 as $type );
                let direction = Vector3::new( 4 as $type, 5 as $type, 6 as $type);

                let t = 10.0 as $type;

                let ray = ParametricLine::new(origin, direction);

                assert_eq!(ray.at(t), origin + direction * t);
            }    
        }
    }

    parametric_line_3_at! { u8, parametric_line_3_at_u8 }
    parametric_line_3_at! { u16, parametric_line_3_at_u16 }
    parametric_line_3_at! { u32, parametric_line_3_at_u32 }
    parametric_line_3_at! { u64, parametric_line_3_at_u64 }
    parametric_line_3_at! { u128, parametric_line_3_at_u128 }
    parametric_line_3_at! { i8, parametric_line_3_at_i8 }
    parametric_line_3_at! { i16, parametric_line_3_at_i16 }
    parametric_line_3_at! { i32, parametric_line_3_at_i32 }
    parametric_line_3_at! { i64, parametric_line_3_at_i64 }
    parametric_line_3_at! { i128, parametric_line_3_at_i128 }
    parametric_line_3_at! { f32, parametric_line_3_at_f32 }
    parametric_line_3_at! { f64, parametric_line_3_at_f64 }
}
