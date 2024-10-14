use crate::color::Color;
use crate::math::{Point, Point2, Vector};

use std::ops::Deref;

pub mod analyzer;
pub mod converter;
pub mod farbfeld;
pub mod generator;
pub mod repeater;

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

pub struct ImageBuffer<C: Color> {
    pixel_data: Vec<C>,
    size: <Point2<usize> as Point>::VectorType,
}

impl<C: Color> ImageBuffer<C> {
    pub fn new(size: <Point2<usize> as Point>::VectorType, color: C) -> ImageBuffer<C> {
        ImageBuffer {
            pixel_data: vec![color; size.x * size.y],
            size,
        }
    }
}

impl<C: Color> Image for ImageBuffer<C> {
    type ColorType = C;
    type PointType = Point2<usize>;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.size
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        self.pixel_data[p.y * self.size.x + p.x]
    }
}

impl<C: Color> WritableImage for ImageBuffer<C> {
    fn get_mut(&mut self, p: Self::PointType) -> &mut Self::ColorType {
        self.pixel_data.get_mut(p.y * self.size.x + p.x).unwrap()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::RGB;
    use crate::math::Vector2;

    #[test]
    fn new_image_buffer() {
        let size = Vector2::new(640, 480);
        let img: ImageBuffer<RGB<u8>> = ImageBuffer::new(size, RGB::default());

        assert_eq!(img.size(), size);
    }
}
