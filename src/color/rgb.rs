use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, Mul};

use super::Color;
use super::YCbCr;
use super::RGBA;

use crate::traits::Number;

create_color_type! { RGB, [red green blue] }

impl From<YCbCr<u8>> for RGB<u8> {
    fn from(ycbcr: YCbCr<u8>) -> RGB<u8> {
        let y = ycbcr.y as f32;
        let cb = ycbcr.cb as f32;
        let cr = ycbcr.cr as f32;

        let red = (y + 1.402 * (cr - 128.0)).round().max(0.0).min(255.0) as u8;
        let green = (y - (0.114 * 1.772 * (cb - 128.0) + 0.299 * 1.402 * (cr - 128.0)) / 0.587)
            .round()
            .max(0.0)
            .min(255.0) as u8;
        let blue = (y + 1.772 * (cb - 128.0)).round().max(0.0).min(255.0) as u8;

        RGB::new(red, green, blue)
    }
}

impl<T> Index<usize> for RGB<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("Index out of range"),
        }
    }
}

impl<T> From<RGBA<T>> for RGB<T> {
    fn from(rgba: RGBA<T>) -> RGB<T> {
        RGB::new(rgba.red, rgba.green, rgba.blue)
    }
}

macro_rules! rgb_from_floating_to_integral {
    ($floating: ty, [$($integral: ty),+]) => {
        $(
        impl From<RGB<$floating>> for RGB<$integral> {
            fn from(rgb: RGB<$floating>) -> RGB<$integral> {
                RGB::new( (rgb.red * <$integral>::MAX as $floating) as $integral,
                          (rgb.green * <$integral>::MAX as $floating) as $integral,
                          (rgb.blue * <$integral>::MAX as $floating) as $integral
                )
            }
        })+
    }
}

rgb_from_floating_to_integral! { f32, [u8, u16, i8, i16] }
rgb_from_floating_to_integral! { f64, [u8, u16, i8, i16] }

// To test

impl<T: AddAssign> AddAssign for RGB<T> {
    fn add_assign(&mut self, rhs: RGB<T>) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl<T: Div<Output = T> + Copy + Clone> Div<T> for RGB<T> {
    type Output = RGB<T>;

    fn div(self, rhs: T) -> Self::Output {
        RGB::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

// End to test

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_rgb {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgb = RGB::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(rgb.red, 1 as $type);
                assert_eq!(rgb.green, 2 as $type);
                assert_eq!(rgb.blue, 3 as $type);
            }
        };
    }

    new_rgb! { u8, new_rgb_u8 }
    new_rgb! { u16, new_rgb_u16 }
    new_rgb! { u32, new_rgb_u32 }
    new_rgb! { u64, new_rgb_u64 }
    new_rgb! { u128, new_rgb_u128 }
    new_rgb! { i8, new_rgb_i8 }
    new_rgb! { i16, new_rgb_i16 }
    new_rgb! { i32, new_rgb_i32 }
    new_rgb! { i64, new_rgb_i64 }
    new_rgb! { i128, new_rgb_i128 }
    new_rgb! { f32, new_rgb_f32 }
    new_rgb! { f64, new_rgb_f64 }

    macro_rules! default_rgb {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgb1 = RGB::new(<$type>::default(), <$type>::default(), <$type>::default());
                let rgb2 = RGB::<$type>::default();

                assert_eq!(rgb1, rgb2);
            }
        };
    }

    default_rgb! { u8, default_rgb_u8 }
    default_rgb! { u16, default_rgb_u16 }
    default_rgb! { u32, default_rgb_u32 }
    default_rgb! { u64, default_rgb_u64 }
    default_rgb! { u128, default_rgb_u128 }
    default_rgb! { i8, default_rgb_i8 }
    default_rgb! { i16, default_rgb_i16 }
    default_rgb! { i32, default_rgb_i32 }
    default_rgb! { i64, default_rgb_i64 }
    default_rgb! { i128, default_rgb_i128 }
    default_rgb! { f32, default_rgb_f32 }
    default_rgb! { f64, default_rgb_f64 }

    macro_rules! clamped_rgb {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = RGB::new(0 as $type, 0 as $type, 0 as $type);
                let b = RGB::new(1 as $type, 1 as $type, 1 as $type);
                let c = RGB::new(2 as $type, 2 as $type, 2 as $type);
                let d = RGB::new(3 as $type, 3 as $type, 3 as $type);
                let e = RGB::new(4 as $type, 4 as $type, 4 as $type);
                let f = RGB::new(5 as $type, 5 as $type, 5 as $type);

                let min = RGB::new(0 as $type, 1 as $type, 2 as $type);
                let max = RGB::new(2 as $type, 3 as $type, 4 as $type);

                assert_eq!(a.clamped(min, max), min);
                assert_eq!(
                    b.clamped(min, max),
                    RGB::new(1 as $type, 1 as $type, 2 as $type)
                );
                assert_eq!(c.clamped(min, max), c);
                assert_eq!(
                    d.clamped(min, max),
                    RGB::new(2 as $type, 3 as $type, 3 as $type)
                );
                assert_eq!(e.clamped(min, max), max);
                assert_eq!(f.clamped(min, max), max);
            }
        };
    }

    clamped_rgb! { u8, clamped_rgb_u8 }
    clamped_rgb! { u16, clamped_rgb_u16 }
    clamped_rgb! { u32, clamped_rgb_u32 }
    clamped_rgb! { u64, clamped_rgb_u64 }
    clamped_rgb! { u128, clamped_rgb_u128 }
    clamped_rgb! { i8, clamped_rgb_i8 }
    clamped_rgb! { i16, clamped_rgb_i16 }
    clamped_rgb! { i32, clamped_rgb_i32 }
    clamped_rgb! { i64, clamped_rgb_i64 }
    clamped_rgb! { i128, clamped_rgb_i128 }
    clamped_rgb! { f32, clamped_rgb_f32 }
    clamped_rgb! { f64, clamped_rgb_f64 }

    macro_rules! rgb_from_ycbcr {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let ycbcr_black = YCbCr::new(0 as $type, 128 as $type, 128 as $type);
                let ycbcr_white = YCbCr::new(255 as $type, 128 as $type, 128 as $type);
                let ycbcr_gray = YCbCr::new(128 as $type, 128 as $type, 128 as $type);
                let ycbcr_red = YCbCr::new(76 as $type, 85 as $type, 255 as $type);
                let ycbcr_green = YCbCr::new(150 as $type, 44 as $type, 21 as $type);
                let ycbcr_blue = YCbCr::new(29 as $type, 255 as $type, 107 as $type);

                let rgb_black = RGB::new(0 as $type, 0 as $type, 0 as $type);
                let rgb_white = RGB::new(255 as $type, 255 as $type, 255 as $type);
                let rgb_gray = RGB::new(128 as $type, 128 as $type, 128 as $type);
                let rgb_red = RGB::new(254 as $type, 0 as $type, 0 as $type);
                let rgb_green = RGB::new(0 as $type, 255 as $type, 1 as $type);
                let rgb_blue = RGB::new(0 as $type, 0 as $type, 254 as $type);

                assert_eq!(rgb_black, RGB::from(ycbcr_black));
                assert_eq!(rgb_white, RGB::from(ycbcr_white));
                assert_eq!(rgb_gray, RGB::from(ycbcr_gray));
                assert_eq!(rgb_red, RGB::from(ycbcr_red));
                assert_eq!(rgb_green, RGB::from(ycbcr_green));
                assert_eq!(rgb_blue, RGB::from(ycbcr_blue));
            }
        };
    }

    rgb_from_ycbcr! { u8, rgb_from_ycbcr_u8 }

    macro_rules! rgb_index {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgb = RGB::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(rgb[0], 1 as $type);
                assert_eq!(rgb[1], 2 as $type);
                assert_eq!(rgb[2], 3 as $type);
            }
        };
    }

    rgb_index! { u8, rgb_index_u8 }
    rgb_index! { u16, rgb_index_u16 }
    rgb_index! { u32, rgb_index_u32 }
    rgb_index! { u64, rgb_index_u64 }
    rgb_index! { u128, rgb_index_u128 }
    rgb_index! { i8, rgb_index_i8 }
    rgb_index! { i16, rgb_index_i16 }
    rgb_index! { i32, rgb_index_i32 }
    rgb_index! { i64, rgb_index_i64 }
    rgb_index! { i128, rgb_index_i128 }
    rgb_index! { f32, rgb_index_f32 }
    rgb_index! { f64, rgb_index_f64 }

    macro_rules! rgb_from_rgba {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgba = RGBA::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
                let rgb = RGB::new(1 as $type, 2 as $type, 3 as $type);

                assert_eq!(rgb, RGB::from(rgba));
            }
        };
    }

    rgb_from_rgba! { u8, rgb_from_rgba_u8 }
    rgb_from_rgba! { u16, rgb_from_rgba_u16 }
    rgb_from_rgba! { u32, rgb_from_rgba_u32 }
    rgb_from_rgba! { u64, rgb_from_rgba_u64 }
    rgb_from_rgba! { u128, rgb_from_rgba_u128 }
    rgb_from_rgba! { i8, rgb_from_rgba_i8 }
    rgb_from_rgba! { i16, rgb_from_rgba_i16 }
    rgb_from_rgba! { i32, rgb_from_rgba_i32 }
    rgb_from_rgba! { i64, rgb_from_rgba_i64 }
    rgb_from_rgba! { i128, rgb_from_rgba_i128 }
    rgb_from_rgba! { f32, rgb_from_rgba_f32 }
    rgb_from_rgba! { f64, rgb_from_rgba_f64 }

    macro_rules! integral_from_floating {
        ($floating:ty, $integral:ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = RGB::new(0.0 as $floating, 0.0 as $floating, 0.0 as $floating);
                let b = RGB::new(0.5 as $floating, 0.5 as $floating, 0.5 as $floating);
                let c = RGB::new(1.0 as $floating, 1.0 as $floating, 1.0 as $floating);
                let d = RGB::new(0.0 as $floating, 0.5 as $floating, 1.0 as $floating);

                assert_eq!(RGB::<$integral>::from(a), RGB::<$integral>::new(0, 0, 0));
                assert_eq!(
                    RGB::<$integral>::from(b),
                    RGB::<$integral>::new(
                        <$integral>::MAX / 2,
                        <$integral>::MAX / 2,
                        <$integral>::MAX / 2
                    )
                );
                assert_eq!(
                    RGB::<$integral>::from(c),
                    RGB::<$integral>::new(<$integral>::MAX, <$integral>::MAX, <$integral>::MAX)
                );
                assert_eq!(
                    RGB::<$integral>::from(d),
                    RGB::<$integral>::new(0, <$integral>::MAX / 2, <$integral>::MAX)
                );
            }
        };
    }

    integral_from_floating! { f32, u8, rgb_u8_from_rgb_f32 }
    integral_from_floating! { f32, u16, rgb_u16_from_rgb_f32 }
    integral_from_floating! { f32, i8, rgb_i8_from_rgb_f32 }
    integral_from_floating! { f32, i16, rgb_i16_from_rgb_f32 }
    integral_from_floating! { f64, u8, rgb_u8_from_rgb_f64 }
    integral_from_floating! { f64, u16, rgb_u16_from_rgb_f64 }
    integral_from_floating! { f64, i8, rgb_i8_from_rgb_f64 }
    integral_from_floating! { f64, i16, rgb_i16_from_rgb_f64 }
}
