use super::ImplicitNSphere;

use crate::math::geometry::Rectangle2;
use crate::math::{Point2, Vector2};
use crate::traits::Number;

pub type Circle<T> = ImplicitNSphere<Point2<T>>;

impl<T> Circle<T>
where
    T: Number,
{
    pub fn bound(self) -> Rectangle2<T> {
        let point = Point2::new(self.center.x - self.radius, self.center.y - self.radius);
        let dimension = Vector2::new(
            self.center.x + self.radius - point.x,
            self.center.y + self.radius - point.y,
        );

        Rectangle2::new(point, dimension)
    }
}
