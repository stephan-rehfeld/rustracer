use std::ops::Neg;

use math::geometry::ParametricLine;
use math::{Point2, Point3, Vector2, Vector3};
use scene_graph::Camera;
use traits::{ConvenientNumber, Cos, FloatingPoint, Sin};
use units::angle::{Angle, Radians};
use units::length::Length;

pub trait RaycasterCamera<T: Length> {
    fn ray_for(
        &self,
        size: Vector2<T::ValueType>,
        p: Point2<T::ValueType>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>>;
}

impl<T> RaycasterCamera<T> for Camera<T>
where
    T: Length + Neg<Output = T>,
    T::ValueType: FloatingPoint + ConvenientNumber,
    Radians<T::ValueType>: Angle + Sin<Output = T::ValueType> + Cos<Output = T::ValueType>,
{
    fn ray_for(
        &self,
        size: Vector2<T::ValueType>,
        p: Point2<T::ValueType>,
    ) -> Option<ParametricLine<Point3<T>, Vector3<T>>> {
        match self {
            Camera::FisheyeCamera(cam) => cam.ray_for(size, p),
            Camera::OrthigraphicCamera(cam) => cam.ray_for(size, p),
            Camera::PerspectiveCamera(cam) => cam.ray_for(size, p),
            Camera::PinholeCamera(cam) => cam.ray_for(size, p),
            Camera::SphericalCamera(cam) => cam.ray_for(size, p),
        }
    }
}

mod fisheye_camera;
mod orthographic_camera;
mod perspective_camera;
mod pinhole_camera;
mod spherical_camera;
