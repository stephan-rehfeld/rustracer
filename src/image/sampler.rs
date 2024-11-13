use std::ops::{Add, AddAssign, Div};

use crate::color::Color;
use crate::image::Image;
use crate::math::{Point, Point2};
use crate::random::WichmannHillPRNG;
use crate::sampling::SamplingPatternSet;
use crate::traits::One;

pub trait Sampler: Image {
    fn sample<T>(self, patterns: SamplingPatternSet<Point2<T>>) -> SamplerStruct<T, Self>
    where
        Self: Image<PointType = Point2<T>> + Sized;
}

impl<I> Sampler for I
where
    I: Image,
{
    fn sample<T>(self, patterns: SamplingPatternSet<Point2<T>>) -> SamplerStruct<T, Self>
    where
        Self: Image<PointType = Point2<T>> + Sized,
    {
        SamplerStruct::new(self, patterns)
    }
}

pub struct SamplerStruct<T, I: Image<PointType = Point2<T>>> {
    source: I,
    patterns: SamplingPatternSet<Point2<T>>,
}

impl<T, I: Image<PointType = Point2<T>>> SamplerStruct<T, I> {
    pub fn new(source: I, patterns: SamplingPatternSet<Point2<T>>) -> SamplerStruct<T, I> {
        SamplerStruct { source, patterns }
    }
}

impl<T, I: Image<PointType = Point2<T>>> Image for SamplerStruct<T, I>
where
    T: AddAssign + Add<Output = T>,
    Point2<T>: Copy,
    <I as Image>::ColorType: AddAssign
        + Div<<<I as Image>::ColorType as Color>::ChannelType, Output = <I as Image>::ColorType>,
    <<I as Image>::ColorType as Color>::ChannelType: One,
{
    type ColorType = <I as Image>::ColorType;
    type PointType = <I as Image>::PointType;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.source.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        let mut rnd = WichmannHillPRNG::new_random();
        let pattern = self.patterns.draw_pattern(&mut rnd);

        let mut counter = <<I as Image>::ColorType as Color>::ChannelType::one();

        let mut color = self.source.get(p + pattern[0].as_vector());

        for i in 1..pattern.len() {
            color += self.source.get(p + pattern[i].as_vector());
            counter += <<I as Image>::ColorType as Color>::ChannelType::one();
        }

        color / counter
    }
}
