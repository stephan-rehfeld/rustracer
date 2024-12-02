use std::ops::Div;

use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use sampling::SamplingPattern;

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
