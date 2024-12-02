use crate::Image;

use colors::Color;
use math::{Point2, Vector2};
use traits::{Half, One};

pub struct Checkerboard<C: Color> {
    a: C,
    b: C,
}

impl<C: Color> Checkerboard<C> {
    pub fn generate(a: C, b: C) -> Checkerboard<C> {
        Checkerboard { a, b }
    }
}

impl<C: Color> Image for Checkerboard<C>
where
    C::ChannelType: Half,
{
    type ColorType = C;
    type PointType = Point2<C::ChannelType>;

    fn size(&self) -> Vector2<C::ChannelType> {
        Vector2::new(One::one(), One::one())
    }

    fn get(&self, p: Self::PointType) -> C {
        if p.x < C::ChannelType::one().half() {
            if p.y < C::ChannelType::one().half() {
                return self.a;
            } else {
                return self.b;
            }
        } else {
            if p.y < C::ChannelType::one().half() {
                return self.b;
            } else {
                return self.a;
            }
        }
    }
}
