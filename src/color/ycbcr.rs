use std::hash::{Hash, Hasher};
use std::ops::Index;

use super::Color;
use super::RGB;

create_color_type! { YCbCr, [y cb cr] }

impl<T> Index<usize> for YCbCr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.y,
            1 => &self.cb,
            2 => &self.cr,
            _ => panic!("Index out of range"),
        }
    }
}

impl From<RGB<u8>> for YCbCr<u8> {
    fn from(rgb: RGB<u8>) -> YCbCr<u8> {
        let r = rgb.red as f32;
        let g = rgb.green as f32;
        let b = rgb.blue as f32;

        let y = (0.299 * r + 0.587 * g + 0.114 * b).round().max(0.0).min(255.0) as u8;
        let cb = ((-0.299 * r - 0.587 * g + 0.886 * b) / 1.772 + 128.0).round().max(0.0).min(255.0) as u8;
        let cr = ((0.701 * r - 0.587 * g - 0.114 * b) / 1.402 + 128.0).round().max(0.0).min(255.0) as u8;

        YCbCr::new(y, cb, cr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_ycbcr {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ycbcr = YCbCr::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(ycbcr.y, 1 as $type);
                assert_eq!(ycbcr.cb, 2 as $type);
                assert_eq!(ycbcr.cr, 3 as $type);
            }
        }
    }

    new_ycbcr! { u8, new_ycbcr_u8 }
    new_ycbcr! { u16, new_ycbcr_u16 }
    new_ycbcr! { u32, new_ycbcr_u32 }
    new_ycbcr! { u64, new_ycbcr_u64 }
    new_ycbcr! { u128, new_ycbcr_u128 }
    new_ycbcr! { i8, new_ycbcr_i8 }
    new_ycbcr! { i16, new_ycbcr_i16 }
    new_ycbcr! { i32, new_ycbcr_i32 }
    new_ycbcr! { i64, new_ycbcr_i64 }
    new_ycbcr! { i128, new_ycbcr_i128 }
    new_ycbcr! { f32, new_ycbcr_f32 }
    new_ycbcr! { f64, new_ycbcr_f64 }
 
    macro_rules! default_ycbcr {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ycbcr1 = YCbCr::new( <$type>::default(), <$type>::default(), <$type>::default() );
                let ycbcr2 = YCbCr::<$type>::default();

                assert_eq!(ycbcr1, ycbcr2);
            }
        }
    }

    default_ycbcr! { u8, default_ycbcr_u8 }
    default_ycbcr! { u16, default_ycbcr_u16 }
    default_ycbcr! { u32, default_ycbcr_u32 }
    default_ycbcr! { u64, default_ycbcr_u64 }
    default_ycbcr! { u128, default_ycbcr_u128 }
    default_ycbcr! { i8, default_ycbcr_i8 }
    default_ycbcr! { i16, default_ycbcr_i16 }
    default_ycbcr! { i32, default_ycbcr_i32 }
    default_ycbcr! { i64, default_ycbcr_i64 }
    default_ycbcr! { i128, default_ycbcr_i128 }
    default_ycbcr! { f32, default_ycbcr_f32 }
    default_ycbcr! { f64, default_ycbcr_f64 }

    macro_rules! ycbcr_from_rgb {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgb_black = RGB::new( 0 as $type, 0 as $type, 0 as $type );
                let rgb_white = RGB::new( 255 as $type, 255 as $type, 255 as $type );
                let rgb_gray = RGB::new( 128 as $type, 128 as $type, 128 as $type );
                let rgb_red = RGB::new( 255 as $type, 0 as $type, 0 as $type );
                let rgb_green = RGB::new( 0 as $type, 255 as $type, 0 as $type );
                let rgb_blue = RGB::new( 0 as $type, 0 as $type, 255 as $type );

                let ycbcr_black = YCbCr::new( 0 as $type, 128 as $type, 128 as $type);
                let ycbcr_white = YCbCr::new( 255 as $type, 128 as $type, 128 as $type);
                let ycbcr_gray = YCbCr::new( 128 as $type, 128 as $type, 128 as $type);
                let ycbcr_red = YCbCr::new( 76 as $type, 85 as $type, 255 as $type);
                let ycbcr_green = YCbCr::new( 150 as $type, 44 as $type, 21 as $type);
                let ycbcr_blue = YCbCr::new( 29 as $type, 255 as $type, 107 as $type);


                assert_eq!(ycbcr_black, YCbCr::from(rgb_black));
                assert_eq!(ycbcr_white, YCbCr::from(rgb_white));
                assert_eq!(ycbcr_gray, YCbCr::from(rgb_gray));
                assert_eq!(ycbcr_red, YCbCr::from(rgb_red));
                assert_eq!(ycbcr_green, YCbCr::from(rgb_green));
                assert_eq!(ycbcr_blue, YCbCr::from(rgb_blue));
            }
        }
    }

    ycbcr_from_rgb! { u8, ycbcr_from_rgb_u8 }

    macro_rules! ycbcr_index {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ycbcr = YCbCr::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(ycbcr[0], 1 as $type);
                assert_eq!(ycbcr[1], 2 as $type);
                assert_eq!(ycbcr[2], 3 as $type);
            }
        }
    }

    ycbcr_index! { u8, ycbcr_index_u8 }
    ycbcr_index! { u16, ycbcr_index_u16 }
    ycbcr_index! { u32, ycbcr_index_u32 }
    ycbcr_index! { u64, ycbcr_index_u64 }
    ycbcr_index! { u128, ycbcr_index_u128 }
    ycbcr_index! { i8, ycbcr_index_i8 }
    ycbcr_index! { i16, ycbcr_index_i16 }
    ycbcr_index! { i32, ycbcr_index_i32 }
    ycbcr_index! { i64, ycbcr_index_i64 }
    ycbcr_index! { i128, ycbcr_index_i128 }
    ycbcr_index! { f32, ycbcr_index_f32 }
    ycbcr_index! { f64, ycbcr_index_f64 }
}
