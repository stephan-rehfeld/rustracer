use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::{Add, Index, Mul};

use super::Color;

use crate::traits::{MultiplyStable, Number, SelfMultiply};

create_color_type! { Gray, [value] }

impl<T> Index<usize> for Gray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.value,
            _ => todo!("Index out of range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_gray {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let gray = Gray::new(1 as $type);

                assert_eq!(gray.value, 1 as $type);
            }
        };
    }

    new_gray! { u8, new_gray_u8 }
    new_gray! { u16, new_gray_u16 }
    new_gray! { u32, new_gray_u32 }
    new_gray! { u64, new_gray_u64 }
    new_gray! { u128, new_gray_u128 }
    new_gray! { i8, new_gray_i8 }
    new_gray! { i16, new_gray_i16 }
    new_gray! { i32, new_gray_i32 }
    new_gray! { i64, new_gray_i64 }
    new_gray! { i128, new_gray_i128 }
    new_gray! { f32, new_gray_f32 }
    new_gray! { f64, new_gray_f64 }

    macro_rules! default_gray {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let gray1 = Gray::new(<$type>::default());
                let gray2 = Gray::<$type>::default();

                assert_eq!(gray1, gray2);
            }
        };
    }

    default_gray! { u8, default_gray_u8 }
    default_gray! { u16, default_gray_u16 }
    default_gray! { u32, default_gray_u32 }
    default_gray! { u64, default_gray_u64 }
    default_gray! { u128, default_gray_u128 }
    default_gray! { i8, default_gray_i8 }
    default_gray! { i16, default_gray_i16 }
    default_gray! { i32, default_gray_i32 }
    default_gray! { i64, default_gray_i64 }
    default_gray! { i128, default_gray_i128 }
    default_gray! { f32, default_gray_f32 }
    default_gray! { f64, default_gray_f64 }

    macro_rules! gray_index {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let gray = Gray::new(1 as $type);

                assert_eq!(gray[0], 1 as $type);
            }
        };
    }

    gray_index! { u8, gray_index_u8 }
    gray_index! { u16, gray_index_u16 }
    gray_index! { u32, gray_index_u32 }
    gray_index! { u64, gray_index_u64 }
    gray_index! { u128, gray_index_u128 }
    gray_index! { i8, gray_index_i8 }
    gray_index! { i16, gray_index_i16 }
    gray_index! { i32, gray_index_i32 }
    gray_index! { i64, gray_index_i64 }
    gray_index! { i128, gray_index_i128 }
    gray_index! { f32, gray_index_f32 }
    gray_index! { f64, gray_index_f64 }
}
