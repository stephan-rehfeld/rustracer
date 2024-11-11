use super::{Image, WritableImage};
use crate::color::Color;
use crate::math::geometry::{Circle, Rectangle2};
use crate::math::{Point, Point2};

pub struct ImageBuffer<C: Color> {
    pixel_data: Vec<C>,
    size: <Point2<usize> as Point>::VectorType,
}

impl<C: Color> ImageBuffer<C> {
    pub fn new(size: <Point2<usize> as Point>::VectorType, color: C) -> ImageBuffer<C> {
        ImageBuffer {
            pixel_data: vec![color; size.x * size.y],
            size,
        }
    }
}

impl<C: Color> Image for ImageBuffer<C> {
    type ColorType = C;
    type PointType = Point2<usize>;

    fn size(&self) -> <Self::PointType as Point>::VectorType {
        self.size
    }

    fn get(&self, p: Self::PointType) -> Self::ColorType {
        self.pixel_data[p.y * self.size.x + p.x]
    }
}

impl<C: Color> WritableImage for ImageBuffer<C> {
    fn get_mut(&mut self, p: Self::PointType) -> &mut Self::ColorType {
        self.pixel_data.get_mut(p.y * self.size.x + p.x).unwrap()
    }
}

impl<C: Color> ImageBuffer<C> {
    pub fn draw_rectangle(&mut self, rectangle: Rectangle2<usize>, color: C) {
        for x in rectangle.point.x..(rectangle.dimension.x + rectangle.point.x) {
            let pixel = self.get_mut(Point2::new(x, rectangle.point.y));
            *pixel = color;

            let pixel = self.get_mut(Point2::new(x, rectangle.point.y + rectangle.dimension.y));
            *pixel = color;
        }
        for y in rectangle.point.y..(rectangle.dimension.y + rectangle.point.y) {
            let pixel = self.get_mut(Point2::new(rectangle.point.x, y));
            *pixel = color;

            let pixel = self.get_mut(Point2::new(rectangle.point.x + rectangle.dimension.x, y));
            *pixel = color;
        }
    }

    pub fn draw_point(&mut self, point: Point2<usize>, color: C) {
        let pixel = self.get_mut(point);
        *pixel = color;
    }

    pub fn draw_circle(&mut self, circle: Circle<isize>, color: C) {
        let bound = circle.bound();

        for x in bound.point.x..(bound.dimension.x + bound.point.x + 1) {
            for y in bound.point.y..(bound.dimension.y + bound.point.y + 1) {
                if circle.test(Point2::new(x, y)) <= 0 {
                    let pixel = self.get_mut(Point2::new(x as usize, y as usize));
                    *pixel = color;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::RGB;
    use crate::math::Vector2;

    #[test]
    fn new_image_buffer() {
        let size = Vector2::new(640, 480);
        let img: ImageBuffer<RGB<u8>> = ImageBuffer::new(size, RGB::default());

        assert_eq!(img.size(), size);
    }
}
