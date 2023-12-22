use std::ops::Index;

pub trait Color : Default + Clone + Copy + PartialEq + Index<usize, Output=Self::ChannelType> {
    type ChannelType;

    fn clamped(self, min: Self, max: Self) -> Self where <Self as Color>::ChannelType: PartialOrd;
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

        impl<T: Copy + Clone + Default + PartialEq> Color for $name<T> {
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

        impl<T: Eq> Eq for $name<T> {
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

pub mod rgb;
pub mod rgba;
pub mod ycbcr;
pub mod gray;

pub use rgb::RGB;
pub use rgba::RGBA;
pub use ycbcr::YCbCr;
pub use gray::Gray;
