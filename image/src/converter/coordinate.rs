use std::marker::PhantomData;

use crate::Image;

use math::{Point, Point2, Vector2};

pub struct Coordinate<T: Image, P: Point> {
    source: T,
    _point: PhantomData<P>,
}

impl<T: Image, P: Point> Coordinate<T, P> {
    pub fn new(source: T) -> Coordinate<T, P> {
        Coordinate {
            source,
            _point: PhantomData,
        }
    }
}

impl<T: Image<PointType = Point2<f64>>> Image for Coordinate<T, Point2<usize>> {
    type ColorType = <T as Image>::ColorType;
    type PointType = Point2<usize>;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        let size = self.source.size();
        Vector2::new(size.x as usize, size.y as usize)
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        let p1 = Point2::new(p.x as f64, p.y as f64);
        self.source.get(p1)
    }
}
