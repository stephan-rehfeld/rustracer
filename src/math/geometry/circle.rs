use super::ImplicitNSphere;

use std::fmt::Debug;
use std::ops::{Add, Sub};

use crate::math::geometry::Rectangle2;
use crate::math::{Point2, Vector2};

pub type Circle<T> = ImplicitNSphere<Point2<T>>;

impl<T> Circle<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + Copy + Debug,
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
