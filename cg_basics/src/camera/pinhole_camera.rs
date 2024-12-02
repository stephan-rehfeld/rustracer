use std::ops::{Div, Mul};

use math::{Point3, Vector3};
use traits::{ConvenientNumber, FloatingPoint, Half, Number, SelfMulNumber, Sqrt};
use units::angle::Radians;

pub struct PinholeCamera<T>
where
    T: Div,
{
    pub e: Point3<T>,
    pub u: Vector3<<T as Div>::Output>,
    pub v: Vector3<<T as Div>::Output>,
    pub w: Vector3<<T as Div>::Output>,
    pub vertical_field_of_view: Radians<<T as Div>::Output>,
}

impl<T> PinholeCamera<T>
where
    T: SelfMulNumber<<T as Div>::Output>,
    <T as Div>::Output: FloatingPoint + ConvenientNumber,
    <T as Mul>::Output: Number<<T as Div>::Output> + ConvenientNumber + Sqrt<Output = T>,
{
    pub fn new(
        e: Point3<T>,
        g: Vector3<T>,
        t: Vector3<T>,
        vertical_field_of_view: Radians<<T as Div>::Output>,
    ) -> PinholeCamera<T> {
        let w = -g.normalized();
        let u = Vector3::cross(t, w).normalized();
        let v = Vector3::cross(w, u).normalized();

        let vertical_field_of_view = vertical_field_of_view.half();

        PinholeCamera {
            e,
            u,
            v,
            w,
            vertical_field_of_view,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use traits::ToRadians;
    use units::angle::Degrees;

    macro_rules! new_pinhole_camera {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let e = Point3::new(1 as $type, 2 as $type, 3 as $type);
                let g = Vector3::new(0 as $type, 0 as $type, -1 as $type);
                let t = Vector3::new(0 as $type, -1 as $type, 0 as $type);

                let fov = Degrees::<$type>::new(90.0).to_radians();

                let persp = PinholeCamera::new(e, g, t, fov);

                assert_eq!(persp.e, e);
                assert_eq!(persp.u, Vector3::new(-1 as $type, 0 as $type, 0 as $type));
                assert_eq!(persp.v, Vector3::new(0 as $type, -1 as $type, 0 as $type));
                assert_eq!(persp.w, Vector3::new(0 as $type, 0 as $type, 1 as $type));

                assert_eq!(persp.vertical_field_of_view, fov.half());
            }
        };
    }

    new_pinhole_camera! { f32, new_pinhole_camera_f32 }
    new_pinhole_camera! { f64, new_pinhole_camera_f64 }
}
