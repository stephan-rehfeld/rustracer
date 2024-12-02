use crate::Image;

use colors::RGBA;
use math::Point2;

pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

impl<T: Image<PointType = Point2<usize>, ColorType = RGBA<u16>>> Encoder for T {
    fn encode(&self) -> Vec<u8> {
        let size = self.size();
        let num_pixel = size.x * size.y;

        let mut result = Vec::with_capacity(8 + 4 + 4 + num_pixel * 8);

        result.append(&mut "farbfeld".as_bytes().to_vec());

        result.append(&mut (size.x as u32).to_be_bytes().to_vec());
        result.append(&mut (size.y as u32).to_be_bytes().to_vec());

        for y in 0..size.y {
            for x in 0..size.x {
                let color = self.get(Point2::new(x, y));

                result.append(&mut color.red.to_be_bytes().to_vec());
                result.append(&mut color.green.to_be_bytes().to_vec());
                result.append(&mut color.blue.to_be_bytes().to_vec());
                result.append(&mut color.alpha.to_be_bytes().to_vec());
            }
        }

        result
    }
}
