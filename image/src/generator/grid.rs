use std::ops::Sub;

use crate::Image;
use colors::Color;
use math::{Point2, Vector2};
use traits::{Half, One};

pub struct Grid<C: Color> {
    border: C,
    face: C,
    width: C::ChannelType,
}

impl<C: Color> Grid<C> {
    pub fn generate(border: C, face: C, width: C::ChannelType) -> Grid<C> {
        Grid {
            border,
            face,
            width,
        }
    }
}

impl<C: Color> Image for Grid<C>
where
    C::ChannelType: Sub<Output = C::ChannelType> + Half + PartialOrd,
{
    type ColorType = C;
    type PointType = Point2<C::ChannelType>;

    fn size(&self) -> Vector2<C::ChannelType> {
        Vector2::new(One::one(), One::one())
    }

    fn get(&self, p: Self::PointType) -> C {
        if p.x < self.width.half()
            || p.x > C::ChannelType::one() - self.width.half()
            || p.y < self.width.half()
            || p.y > C::ChannelType::one() - self.width.half()
        {
            self.border
        } else {
            self.face
        }
    }
}
