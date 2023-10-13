#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Normal3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Normal3<T> {
        Normal3 { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_normal3 {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let n = Normal3::new( 1 as $type, 2 as $type, 3 as $type );

                assert_eq!(n.x, 1 as $type);
                assert_eq!(n.y, 2 as $type);
                assert_eq!(n.z, 3 as $type);
            }
        }
    }

    new_normal3! { u8, new_normal3_u8 }
    new_normal3! { u16, new_normal3_u16 }
    new_normal3! { u32, new_normal3_u32 }
    new_normal3! { u64, new_normal3_u64 }
    new_normal3! { u128, new_normal3_u128 }
    new_normal3! { i8, new_normal3_i8 }
    new_normal3! { i16, new_normal3_i16 }
    new_normal3! { i32, new_normal3_i32 }
    new_normal3! { i64, new_normal3_i64 }
    new_normal3! { i128, new_normal3_i128 }
    new_normal3! { f32, new_normal3_f32 }
    new_normal3! { f64, new_normal3_f64 }
}
