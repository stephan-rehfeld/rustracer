use crate::color::Color;
use crate::image::Image;
use crate::math::{Point2, Vector2};
use crate::traits::{Half, One};

pub struct ChessBoard<C: Color> {
    a: C,
    b: C,
}

impl<C: Color> ChessBoard<C> {
    pub fn generate(a: C, b: C) -> ChessBoard<C> {
        ChessBoard { a, b }
    }
}

impl<C: Color> Image for ChessBoard<C>
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
