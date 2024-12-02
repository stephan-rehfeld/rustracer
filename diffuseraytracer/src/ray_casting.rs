use std::collections::HashMap;

use crate::Renderable;
use cg_basics::camera::RaytracingCamera;
use cg_basics::light::Light;
use colors::Color;
use math::{Normal3, Point3, Vector3};
use units::length::Length;

pub struct NewScene<L> {
    lights: Vec<L>,
}

impl<L> NewScene<L> {
    pub fn new(lights: Vec<L>) -> NewScene<L> {
        NewScene { lights }
    }
}

pub struct Scene<T: Length, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub bg_color: C,
    pub lights: Vec<Box<dyn Light<T, C>>>,
    pub cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
    pub geometries: Vec<
        Box<
            dyn Renderable<
                T,
                ScalarType = <T as Length>::ValueType,
                ColorType = C,
                LengthType = T,
                VectorType = Vector3<T>,
                PointType = Point3<T>,
                NormalType = Normal3<<T as Length>::ValueType>,
            >,
        >,
    >,
}

impl<T: Length, C> Scene<T, C>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub fn new(
        bg_color: C,
        lights: Vec<Box<dyn Light<T, C>>>,
        cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
        geometries: Vec<
            Box<
                dyn Renderable<
                    T,
                    ScalarType = <T as Length>::ValueType,
                    ColorType = C,
                    LengthType = T,
                    VectorType = Vector3<T>,
                    PointType = Point3<T>,
                    NormalType = Normal3<<T as Length>::ValueType>,
                >,
            >,
        >,
    ) -> Scene<T, C> {
        Scene {
            bg_color,
            lights,
            cameras,
            geometries,
        }
    }
}
