use std::marker::PhantomData;

use crate::color::Color as ColorTrait;
use crate::image::Image;
use crate::math::Point;

pub struct Color<T: Image, C: ColorTrait> {
    source: T,
    _color: PhantomData<C>,
}

impl<T: Image, C: ColorTrait> Color<T, C> {
    pub fn new(source: T) -> Color<T, C> {
        Color {
            source,
            _color: PhantomData,
        }
    }
}

impl<T: Image, C: ColorTrait> Image for Color<T, C>
where
    C: From<<T as Image>::ColorType>,
{
    type ColorType = C;
    type PointType = <T as Image>::PointType;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.source.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        C::from(self.source.get(p))
    }
}
