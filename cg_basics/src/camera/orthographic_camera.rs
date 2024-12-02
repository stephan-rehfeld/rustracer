use std::ops::{Div, Mul};

use math::{Point3, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Number, SelfMulNumber, Sqrt};

pub struct OrthographicCamera<T>
where
    T: Div,
{
    pub e: Point3<T>,
    pub u: Vector3<<T as Div>::Output>,
    pub v: Vector3<<T as Div>::Output>,
    pub w: Vector3<<T as Div>::Output>,
    pub scale: <T as Div>::Output,
}

impl<T> OrthographicCamera<T>
where
    T: Number<<T as Div>::Output> + SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        scale: <T as Div>::Output,
    ) -> OrthographicCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        OrthographicCamera { e, u, v, w, scale }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_orthographic_camera {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);

                let orth = OrthographicCamera::new(e, g, t, 15.0);

                assert_eq!(orth.e, e);
                assert_eq!(
                    orth.u,
                    Vector3::new(-1 as $type, 0 as $type, 0 as $type).normalized()
                );
                assert_eq!(
                    orth.v,
                    Vector3::new(0 as $type, -1 as $type, 0 as $type).normalized()
                );
                assert_eq!(
                    orth.w,
                    Vector3::new(0 as $type, 0 as $type, 1 as $type).normalized()
                );

                assert_eq!(orth.scale, 15.0);
            }
        };
    }

    new_orthographic_camera! { f32, new_orthographic_camera_f32 }
    new_orthographic_camera! { f64, new_orthographic_camera_f64 }
}
