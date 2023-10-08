
#[derive(Debug,PartialEq,Clone,Copy)]
pub struct RGB<T> {
    pub red: T,
    pub green: T,
    pub blue: T
}

impl<T> RGB<T> {
    pub fn new(red: T, green: T, blue: T) -> RGB<T> {
        RGB { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_rgb {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let rgb = RGB::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(rgb.red, 1 as $type);
                assert_eq!(rgb.green, 2 as $type);
                assert_eq!(rgb.blue, 3 as $type);
            }
        }
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
}
