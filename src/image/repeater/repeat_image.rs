use crate::image::Image;
use crate::math::{Point, Point2};

pub struct RepeatImage<T: Image> {
    source: T,
}

impl<T: Image> RepeatImage<T> {
    pub fn new(source: T) -> RepeatImage<T> {
        RepeatImage { source }
    }
}

// Change this to Integer later
impl<T: Image<PointType = Point2<u32>>> Image for RepeatImage<T> {
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
