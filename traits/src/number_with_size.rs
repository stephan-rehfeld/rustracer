use super::Number;

use std::mem;

pub trait FromBeBytes<const BYTES: usize> {
    fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
}

pub trait FromLeBytes<const BYTES: usize> {
    fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
}

pub trait FromNeBytes<const BYTES: usize> {
    fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;
}

macro_rules! implement_from_bytes_trait {
    ($traitName: ident, $function: ident, $($type: ty)*  ) => {
        $(
        impl $traitName<{mem::size_of::<$type>()}> for $type {
            fn $function(bytes: [u8; mem::size_of::<$type>()]) -> $type {
                <$type>::$function(bytes)
            }
        }
        )*
    }
}

implement_from_bytes_trait! { FromBeBytes, from_be_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
implement_from_bytes_trait! { FromLeBytes, from_le_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
implement_from_bytes_trait! { FromNeBytes, from_ne_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

pub trait ToBeBytes<const BYTES: usize> {
    fn to_be_bytes(self) -> [u8; BYTES];
}

pub trait ToLeBytes<const BYTES: usize> {
    fn to_le_bytes(self) -> [u8; BYTES];
}

pub trait ToNeBytes<const BYTES: usize> {
    fn to_ne_bytes(self) -> [u8; BYTES];
}

macro_rules! implement_to_bytes_trait {
    ($traitName: ident, $function: ident, $($type: ty)*  ) => {
        $(
        impl $traitName<{mem::size_of::<$type>()}> for $type {
            fn $function(self) -> [u8; mem::size_of::<$type>()] {
                self.$function()
            }
        }
        )*
    }
}

implement_to_bytes_trait! { ToBeBytes, to_be_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
implement_to_bytes_trait! { ToLeBytes, to_le_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
implement_to_bytes_trait! { ToNeBytes, to_ne_bytes, f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

pub trait NumberWithSize<const BYTES: usize>:
    Number
    + FromBeBytes<BYTES>
    + FromLeBytes<BYTES>
    + FromNeBytes<BYTES>
    + ToBeBytes<BYTES>
    + ToLeBytes<BYTES>
    + ToNeBytes<BYTES>
{
}

macro_rules! implement_number_with_size_trait {
    ($($type: ty)*  ) => {
        $(
        impl NumberWithSize<{mem::size_of::<$type>()}> for $type {}
        )*
    }
}

implement_number_with_size_trait! { f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
