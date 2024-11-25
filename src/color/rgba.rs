use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::{Add, Index, Mul};

use super::{Color, RGB};

use crate::traits::Number;

create_color_type! { RGBA, [red green blue alpha] }

impl<T> Index<usize> for RGBA<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            3 => &self.alpha,
            _ => panic!("Index out of range"),
        }
    }
}

macro_rules! rgba_from_floating_to_integral {
    ($floating: ty, [$($integral: ty),+]) => {
        $(
        impl From<RGBA<$floating>> for RGBA<$integral> {
            fn from(rgba: RGBA<$floating>) -> RGBA<$integral> {
                RGBA::new( (rgba.red * <$integral>::MAX as $floating) as $integral,
                           (rgba.green * <$integral>::MAX as $floating) as $integral,
                           (rgba.blue * <$integral>::MAX as $floating) as $integral,
                           (rgba.alpha * <$integral>::MAX as $floating) as $integral
                )
            }
        })+
    }
}

rgba_from_floating_to_integral! { f32, [u8, u16, i8, i16] }
rgba_from_floating_to_integral! { f64, [u8, u16, i8, i16] }

macro_rules! rgba_floating_from_rgb {
    ($($type: ty),+) => {
        $(
            impl From<RGB<$type>> for RGBA<$type> {
                fn from(rgb: RGB<$type>) -> RGBA<$type> {
                    RGBA::new(rgb.red, rgb.green, rgb.blue, 1.0)
                }
            }
        )+
    }
}

rgba_floating_from_rgb! { f32, f64 }

macro_rules! rgba_integral_from_rgb {
    ($($type: ty),+) => {
        $(
            impl From<RGB<$type>> for RGBA<$type> {
                fn from(rgb: RGB<$type>) -> RGBA<$type> {
                    RGBA::new(rgb.red, rgb.green, rgb.blue, <$type>::MAX)
                }
            }
        )+
    }
}

rgba_integral_from_rgb! { u8, u16, i8, i16 }

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_rgba {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgba = RGBA::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);

                assert_eq!(rgba.red, 1 as $type);
                assert_eq!(rgba.green, 2 as $type);
                assert_eq!(rgba.blue, 3 as $type);
                assert_eq!(rgba.alpha, 4 as $type);
            }
        };
    }

    new_rgba! { u8, new_rgba_u8 }
    new_rgba! { u16, new_rgba_u16 }
    new_rgba! { u32, new_rgba_u32 }
    new_rgba! { u64, new_rgba_u64 }
    new_rgba! { u128, new_rgba_u128 }
    new_rgba! { i8, new_rgba_i8 }
    new_rgba! { i16, new_rgba_i16 }
    new_rgba! { i32, new_rgba_i32 }
    new_rgba! { i64, new_rgba_i64 }
    new_rgba! { i128, new_rgba_i128 }
    new_rgba! { f32, new_rgba_f32 }
    new_rgba! { f64, new_rgba_f64 }

    macro_rules! default_rgba {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgba1 = RGBA::new(
                    <$type>::default(),
                    <$type>::default(),
                    <$type>::default(),
                    <$type>::default(),
                );
                let rgba2 = RGBA::<$type>::default();

                assert_eq!(rgba1, rgba2);
            }
        };
    }

    default_rgba! { u8, default_rgba_u8 }
    default_rgba! { u16, default_rgba_u16 }
    default_rgba! { u32, default_rgba_u32 }
    default_rgba! { u64, default_rgba_u64 }
    default_rgba! { u128, default_rgba_u128 }
    default_rgba! { i8, default_rgba_i8 }
    default_rgba! { i16, default_rgba_i16 }
    default_rgba! { i32, default_rgba_i32 }
    default_rgba! { i64, default_rgba_i64 }
    default_rgba! { i128, default_rgba_i128 }
    default_rgba! { f32, default_rgba_f32 }
    default_rgba! { f64, default_rgba_f64 }

    macro_rules! rgba_index {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgba = RGBA::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);

                assert_eq!(rgba[0], 1 as $type);
                assert_eq!(rgba[1], 2 as $type);
                assert_eq!(rgba[2], 3 as $type);
                assert_eq!(rgba[3], 4 as $type);
            }
        };
    }

    rgba_index! { u8, rgba_index_u8 }
    rgba_index! { u16, rgba_index_u16 }
    rgba_index! { u32, rgba_index_u32 }
    rgba_index! { u64, rgba_index_u64 }
    rgba_index! { u128, rgba_index_u128 }
    rgba_index! { i8, rgba_index_i8 }
    rgba_index! { i16, rgba_index_i16 }
    rgba_index! { i32, rgba_index_i32 }
    rgba_index! { i64, rgba_index_i64 }
    rgba_index! { i128, rgba_index_i128 }
    rgba_index! { f32, rgba_index_f32 }
    rgba_index! { f64, rgba_index_f64 }

    macro_rules! integral_from_floating {
        ($floating:ty, $integral:ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = RGBA::new(
                    0.0 as $floating,
                    0.0 as $floating,
                    0.0 as $floating,
                    0.0 as $floating,
                );
                let b = RGBA::new(
                    0.5 as $floating,
                    0.5 as $floating,
                    0.5 as $floating,
                    0.5 as $floating,
                );
                let c = RGBA::new(
                    1.0 as $floating,
                    1.0 as $floating,
                    1.0 as $floating,
                    1.0 as $floating,
                );
                let d = RGBA::new(
                    0.0 as $floating,
                    0.5 as $floating,
                    1.0 as $floating,
                    1.0 as $floating,
                );

                assert_eq!(
                    RGBA::<$integral>::from(a),
                    RGBA::<$integral>::new(0, 0, 0, 0)
                );
                assert_eq!(
                    RGBA::<$integral>::from(b),
                    RGBA::<$integral>::new(
                        <$integral>::MAX / 2,
                        <$integral>::MAX / 2,
                        <$integral>::MAX / 2,
                        <$integral>::MAX / 2
                    )
                );
                assert_eq!(
                    RGBA::<$integral>::from(c),
                    RGBA::<$integral>::new(
                        <$integral>::MAX,
                        <$integral>::MAX,
                        <$integral>::MAX,
                        <$integral>::MAX
                    )
                );
                assert_eq!(
                    RGBA::<$integral>::from(d),
                    RGBA::<$integral>::new(
                        0,
                        <$integral>::MAX / 2,
                        <$integral>::MAX,
                        <$integral>::MAX
                    )
                );
            }
        };
    }

    integral_from_floating! { f32, u8, rgba_u8_from_rgb_f32 }
    integral_from_floating! { f32, u16, rgba_u16_from_rgb_f32 }
    integral_from_floating! { f32, i8, rgba_i8_from_rgb_f32 }
    integral_from_floating! { f32, i16, rgba_i16_from_rgb_f32 }
    integral_from_floating! { f64, u8, rgba_u8_from_rgb_f64 }
    integral_from_floating! { f64, u16, rgba_u16_from_rgb_f64 }
    integral_from_floating! { f64, i8, rgba_i8_from_rgb_f64 }
    integral_from_floating! { f64, i16, rgba_i16_from_rgb_f64 }

    macro_rules! rgba_from_rgb_floating {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = RGB::new(0.1 as $type, 0.5 as $type, 0.99 as $type);

                assert_eq!(RGBA::from(a), RGBA::new(0.1, 0.5, 0.99, 1.0));
            }
        };
    }

    rgba_from_rgb_floating! { f32, rgba_from_rgb_f32 }
    rgba_from_rgb_floating! { f64, rgba_from_rgb_f64 }

    macro_rules! rgba_from_rgb_integral {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let a = RGB::new(23 as $type, 42 as $type, 100 as $type);

                assert_eq!(RGBA::from(a), RGBA::new(23, 42, 100, <$type>::MAX));
            }
        };
    }

    rgba_from_rgb_integral! { u8, rgba_from_rgb_u8 }
    rgba_from_rgb_integral! { u16, rgba_from_rgb_u16 }
    rgba_from_rgb_integral! { i8, rgba_from_rgb_i8 }
    rgba_from_rgb_integral! { i16, rgba_from_rgb_i16 }
}
