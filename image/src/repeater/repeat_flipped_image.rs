use crate::image::Image;

use crate::math::{Point, Point2};

pub struct RepeatFlippedImage<T: Image> {
    source: T,
}

impl<T: Image> RepeatFlippedImage<T> {
    pub fn new(source: T) -> RepeatFlippedImage<T> {
        RepeatImage { source }
    }
}

// Change this to Integer later
impl<T: Image<PointType = Point2<u32>>> Image for RepeatFlippedImage<T> {
    type ColorType = <T as Image>::ColorType;
    type PointType = <T as Image>::PointType;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.source.size()
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        let p1 = Point2::new(p.x % self.size().x, p.y % self.size().y);
        self.source.get(p1)
    }
}
