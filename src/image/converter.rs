pub mod clamp;
pub mod color;
pub mod coordinate;
pub mod splitter;

pub use clamp::Clamp;
pub use color::Color;
pub use coordinate::Coordinate;
pub use splitter::Splitter;

use super::Image;

use crate::color::Color as ColorTrait;
use crate::math::Point;

pub trait Converter: Image {
    fn clamp_color(
        self,
        min: <Self as Image>::ColorType,
        max: <Self as Image>::ColorType,
    ) -> Clamp<Self>
    where
        Self: Sized;
    fn convert_color<C: ColorTrait>(self) -> Color<Self, C>
    where
        Self: Sized;
    fn convert_coordinate<P: Point>(self) -> Coordinate<Self, P>
    where
        Self: Sized;
    fn split_channel<'a>(&'a self, channel: usize) -> Splitter<'a, Self>
    where
        Self: Sized;
}

impl<T> Converter for T
where
    T: Image,
{
    fn clamp_color(
        self,
        min: <Self as Image>::ColorType,
        max: <Self as Image>::ColorType,
    ) -> Clamp<Self>
    where
        Self: Sized,
    {
        Clamp::new(self, min, max)
    }

    fn convert_color<C: ColorTrait>(self) -> Color<Self, C>
    where
        Self: Sized,
    {
        Color::new(self)
    }

    fn convert_coordinate<P: Point>(self) -> Coordinate<Self, P>
    where
        Self: Sized,
    {
        Coordinate::new(self)
    }

    fn split_channel<'a>(&'a self, channel: usize) -> Splitter<'a, Self>
    where
        Self: Sized,
    {
        Splitter::new(&self, channel)
    }
}
