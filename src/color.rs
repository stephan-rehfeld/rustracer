use std::iter::Sum;
use std::ops::{Add, Index, Mul};

use crate::traits::{MultiplyStable, Number, SelfMultiply};

pub trait Color:
    Add<Output = Self>
    + Mul<Self::ChannelType, Output = Self>
    + Mul<Output = Self>
    + Sum
    + Default
    + Clone
    + Copy
    + PartialEq
    + Index<usize, Output = Self::ChannelType>
{
    type ChannelType: Number + SelfMultiply + MultiplyStable;

    fn clamped(self, min: Self, max: Self) -> Self
    where
        <Self as Color>::ChannelType: PartialOrd;
}

#[macro_export]
macro_rules! create_color_type {
    ($name: ident, [$($channel: ident)+]) => {
        #[derive(Debug,PartialEq,Clone,Copy)]
        pub struct $name<T> {
        $(
            pub $channel: T,
        )+
        }

        impl<T> $name<T> {
            pub fn new($($channel: T,)+) -> $name<T> {
                $name { $($channel, )+ }
            }
        }

        impl<T: Default> Default for $name<T> {
            fn default() -> Self {
                $name { $($channel: Default::default(),)+ }
            }
        }

        impl<T: Number + SelfMultiply + MultiplyStable> Color for $name<T> {
            type ChannelType = T;

            fn clamped(self, min: $name<T>, max: $name<T>) -> $name<T> where T: PartialOrd {
                $(
                let $channel = if self.$channel < min.$channel {
                    min.$channel
                } else {
                    if self.$channel > max.$channel {
                        max.$channel
                    } else {
                        self.$channel
                    }

                };
                )+
                $name::new( $($channel,)+ )
            }
        }

        impl<T: Add<U> , U> Add<$name<U>> for $name<T> {
            type Output = $name<<T as Add<U>>::Output>;

            fn add(self, rhs: $name<U>) -> Self::Output {
                $name::new( $( self.$channel + rhs.$channel, )* )
            }
        }


        impl<T: Mul<Output=T> + Copy> Mul<T> for $name<T> {
            type Output = $name<T>;

            fn mul(self, rhs: T) -> Self::Output {
                $name::new( $( self.$channel * rhs, )* )
            }
        }

        impl<T: Mul<Output=T>> Mul for $name<T> {
            type Output = $name<T>;

            fn mul(self, rhs: Self) -> Self::Output {
                $name::new( $( self.$channel * rhs.$channel, )* )
            }
        }

        impl<T: Eq> Eq for $name<T> {
        }


        impl<T: Add<Output=T> + Default> Sum for $name<T> {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    Default::default(),
                    |a, b| a + b
                )
            }
        }

        impl<T: Hash> Hash for $name<T> {
            fn hash<H>(&self, state: &mut H) where H: Hasher {
                $(
                self.$channel.hash(state);
                )+
            }
        }
    }
}

pub mod gray;
pub mod rgb;
pub mod rgba;
pub mod ycbcr;

pub use gray::Gray;
pub use rgb::RGB;
pub use rgba::RGBA;
pub use ycbcr::YCbCr;
