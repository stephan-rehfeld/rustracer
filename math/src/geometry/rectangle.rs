use crate::{Point2, Vector2};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle2<T> {
    pub point: Point2<T>,
    pub dimension: Vector2<T>,
}

impl<T> Rectangle2<T> {
    pub fn new(point: Point2<T>, dimension: Vector2<T>) -> Rectangle2<T> {
        Rectangle2 { point, dimension }
    }
}
