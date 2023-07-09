
pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_sqrt_for {
    ($($type: ty)* ) => ($(
        impl Sqrt for $type  {
            type Output = $type;

            fn sqrt(self) -> <$type as Sqrt>::Output {
                self.sqrt()
            }
        }
    )*)

}

impl_sqrt_for! { f32 f64 }

#[cfg(test)]
mod tests {

    use super::*;

    fn sqrt_test<T: Sqrt>(v: T) -> <T as Sqrt>::Output {
        v.sqrt()
    }

    macro_rules! sqrt {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let v = 4 as $type;

                assert_eq!(sqrt_test(v), 2 as $type);
            }
        }
    }

    sqrt! { f32, sqrt_f32 }
    sqrt! { f64, sqrt_f64 } 
}
