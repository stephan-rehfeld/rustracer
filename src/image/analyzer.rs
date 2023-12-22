use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

use crate::color::Color;
use crate::math::Point2;
use crate::image::Image;

pub struct Histogram<C: Color> {
    data: HashMap<C, usize>,
    num_pixel: usize,
}

impl<C: Color> Histogram<C> where C: Eq + Hash {

    pub fn from_image<T>(img: &T) -> Histogram<C> where
        T: Image<ColorType=C, PointType=Point2<usize>>,
    {
        let mut data: HashMap<C, usize> = HashMap::new();

        let size = img.size();

        for x in 0..size.x {
            for y in 0..size.y {
               let color = img.get(Point2::new(x, y));
                match data.get_mut(&color) {
                    Some(counter) => {
                        *counter += 1;
                    }
                    None => {
                        data.insert(color, 1);
                    }
                }
            }
        }

        Histogram { data, num_pixel: size.x * size.y }
    }
}

impl<C: Color> Histogram<C> where C: Eq + Hash {
    pub fn entropy(&self) -> f32 {
        let len = self.num_pixel as f32;
        
        let mut e = 0.0;

        for c in self.data.keys() {
            let n = self[*c] as f32;

            let p = n / len;

            if p != 0.0 {
                e += p * p.log2();
            }
        }

        -e
    }
}

impl<C: Color> Index<C> for Histogram<C> where C: Eq + Hash {
    type Output = usize;

    fn index(&self, index: C) -> &Self::Output {
        match self.data.get(&index) {
            Some(n) => n,
            None => &0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::Gray;
    use crate::image::{ImageBuffer, WritableImage};
    use crate::math::Vector2;

    #[test]
    fn histogram_from_image() {
        let size = Vector2::new(16,16);

        let black: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(0)); 
        let mut all_tones: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(0)); 

        for x in 0..size.x {
            for y in 0..size.y {
                let color = all_tones.get_mut(Point2::new(x, y));
                let x = x as u8;
                let y = y as u8;
                *color = Gray::new(y * 16 + x);
            }
        }

        let white: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(255)); 

        let histogram_black = Histogram::from_image(&black);
        let histogram_all_tones = Histogram::from_image(&all_tones);
        let histogram_white = Histogram::from_image(&white);

        assert_eq!(histogram_black[Gray::new(0)], 256);
        for v in 1..=255 {
            assert_eq!(histogram_black[Gray::new(v)], 0);
        }

        for v in 0..=255 {
            assert_eq!(histogram_all_tones[Gray::new(v)], 1);
        }

        for v in 0..=254 {
            assert_eq!(histogram_white[Gray::new(v)], 0);
        }
        assert_eq!(histogram_white[Gray::new(255)], 256);
    }

    #[test]
    fn histogram_entropy() {
        let size = Vector2::new(16,16);

        let black: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(0)); 
        let mut all_tones: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(0)); 

        for x in 0..size.x {
            for y in 0..size.y {
                let color = all_tones.get_mut(Point2::new(x, y));
                let x = x as u8;
                let y = y as u8;
                *color = Gray::new(y * 16 + x);
            }
        }

        let white: ImageBuffer<Gray<u8>> = ImageBuffer::new(Vector2::new(16, 16), Gray::new(255)); 

        let histogram_black = Histogram::from_image(&black);
        let histogram_all_tones = Histogram::from_image(&all_tones);
        let histogram_white = Histogram::from_image(&white);

        assert_eq!(histogram_black.entropy(), 0.0);
        assert_eq!(histogram_all_tones.entropy(), 8.0);
        assert_eq!(histogram_white.entropy(), 0.0);
    }
}
