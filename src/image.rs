use crate::color::Color;
use crate::math::{Point, Vector};

use std::ops::Deref;

pub mod analyzer;
pub mod converter;
pub mod farbfeld;
pub mod generator;
pub mod image_buffer;
pub mod repeater;
pub mod sampler;

pub use image_buffer::ImageBuffer;

pub trait Image {
    type ColorType: Color;
    type PointType: Point;

    fn size(&self) -> <<Self as Image>::PointType as Point>::VectorType;
    fn get(&self, p: Self::PointType) -> Self::ColorType;
}

impl<C: Color, P: Point> Image for Box<dyn Image<ColorType = C, PointType = P>> {
    type ColorType = C;
    type PointType = P;

    fn size(&self) -> <<Self as Image>::PointType as Point>::VectorType {
        self.deref().size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        self.deref().get(p)
    }
}

pub trait WritableImage: Image {
    fn get_mut(&mut self, p: Self::PointType) -> &mut Self::ColorType;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SingleColorImage<C: Color, S: Vector> {
    color: C,
    size: S,
}

impl<C: Color, S: Vector> SingleColorImage<C, S> {
    pub fn new(color: C, size: S) -> SingleColorImage<C, S> {
        SingleColorImage { color, size }
    }
}

impl<C: Color, S: Vector + Copy + Clone> Image for SingleColorImage<C, S>
where
    <S as Vector>::PointType: Point<VectorType = S>,
{
    type ColorType = C;
    type PointType = <S as Vector>::PointType;

    fn size(&self) -> S {
        self.size
    }

    fn get(&self, _p: <S as Vector>::PointType) -> C {
        self.color
    }
}
