use std::collections::HashMap;

use crate::camera::RaytracingCamera;
use colors::Color;
use units::length::Length;

pub struct Scene<T: Length, C, L, G>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub bg_color: C,
    pub lights: Vec<L>,
    pub cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
    pub geometries: Vec<G>,
}

impl<T: Length, C, L, G> Scene<T, C, L, G>
where
    C: Color<ChannelType = <T as Length>::ValueType>,
{
    pub fn new(
        bg_color: C,
        lights: Vec<L>,
        cameras: HashMap<String, Box<dyn RaytracingCamera<T>>>,
        geometries: Vec<G>,
    ) -> Scene<T, C, L, G> {
        Scene {
            bg_color,
            lights,
            cameras,
            geometries,
        }
    }
}
