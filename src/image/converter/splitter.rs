use crate::color::{Color as ColorTrait, Gray};
use crate::image::Image;
use crate::math::Point;

pub struct Splitter<'a, T: Image> {
    source: &'a T,
    channel: usize,
}

impl<T: Image> Splitter<'_, T> {
    pub fn new(source: &T, channel: usize) -> Splitter<T> {
        Splitter { source, channel }
    }
}

impl<T: Image> Image for Splitter<'_, T>
where
    <<T as Image>::ColorType as ColorTrait>::ChannelType: Copy + Default + PartialEq,
{
    type ColorType = Gray<<<T as Image>::ColorType as ColorTrait>::ChannelType>;
    type PointType = <T as Image>::PointType;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.source.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        Gray::new(self.source.get(p)[self.channel])
    }
}
