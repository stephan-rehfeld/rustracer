use std::ops::Div;

use crate::math::geometry::ParametricLine;
use crate::math::{Point2, Point3, Vector2, Vector3};
use crate::sampling::SamplingPattern;

pub trait RaytracingCamera<T>
where
    T: Div,
{
    fn ray_for(
        &self,
        size: Vector2<<T as Div>::Output>,
        p: Point2<<T as Div>::Output>,
        pattern: &SamplingPattern<Point2<<T as Div>::Output>>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>>;
}

mod fisheye_camera;
mod orthographic_camera;
mod perspective_camera;
mod pinhole_camera;
mod spherical_camera;

pub use fisheye_camera::FisheyeCamera;
pub use orthographic_camera::OrthographicCamera;
pub use perspective_camera::PerspectiveCamera;
pub use pinhole_camera::PinholeCamera;
pub use spherical_camera::SphericalCamera;
