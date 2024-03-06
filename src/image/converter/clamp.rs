use crate::color::Color as ColorTrait;
use crate::image::Image;
use crate::math::Point;

pub struct Clamp<T: Image> {
    source: T,
    min: <T as Image>::ColorType,
    max: <T as Image>::ColorType,
}

impl<T: Image> Clamp<T> {
    pub fn new(source: T, min: <T as Image>::ColorType, max: <T as Image>::ColorType) -> Clamp<T> {
        Clamp { source, min, max }
    }
}

impl<T: Image> Image for Clamp<T>
where
    <T as Image>::ColorType: PartialEq,
    <<T as Image>::ColorType as ColorTrait>::ChannelType: PartialOrd,
{
    type ColorType = <T as Image>::ColorType;
    type PointType = <T as Image>::PointType;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.source.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        self.source.get(p).clamped(self.min, self.max)
    }
}
