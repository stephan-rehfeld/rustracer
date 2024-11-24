macro_rules! implement_marker_trait {
    ($traitName: ident, $($type: ty)*  ) => {
        $(
        impl $traitName for $type  {
        }
        )*
    }
}

macro_rules! create_and_implement_proxy_trait {
    (with name $traitName: ident and function $functionName: ident for types [ $($type: ty)*  ] ) => {
        pub trait $traitName {
            fn $functionName(self) -> Self;
        }

        $(
            impl $traitName for $type  {
                fn $functionName(self) -> $type {
                    self.$functionName()
                }
            }
        )*

    };

    (with name $traitName: ident and function $functionName: ident and one parameter for types [ $($type: ty)*  ] ) => {
        pub trait $traitName {
            fn $functionName(self, rhs: Self) -> Self;
        }

        $(
            impl $traitName for $type  {
                fn $functionName(self, rhs: $type) -> $type {
                    self.$functionName(rhs)
                }
            }
        )*
    };

    (with name $traitName: ident and function $functionName: ident and output $output: ident  for types [ $($type: ty)*  ] ) => {
        pub trait $traitName {
            fn $functionName(self) -> $output;
        }

        $(
            impl $traitName for $type  {
                fn $functionName(self) -> $output {
                    self.$functionName()
                }
            }
        )*
    };

    (with name $traitName: ident and function $functionName: ident and different output  for types [ $($type: ty)*  ] ) => {
        pub trait $traitName {
            type Output;

            fn $functionName(self) -> Self::Output;
        }

        $(
            impl $traitName for $type  {
                type Output = $type;

                fn $functionName(self) -> <$type as $traitName>::Output {
                    self.$functionName()
                }
            }
        )*
    };

    (with name $traitName: ident and function $functionName: ident and different output and one parameter for types [ $($type: ty)*  ] ) => {
        pub trait $traitName {
            type Output;

            fn $functionName(self, rhs: Self) -> Self::Output;
        }

        $(
            impl $traitName for $type  {
                type Output = $type;

                fn $functionName(self, rhs: $type) -> <$type as $traitName>::Output {
                    self.$functionName(rhs)
                }
            }
        )*
    };

}

pub mod convenience_number;
pub mod floating_point;
pub mod number;
pub mod number_with_size;
pub mod signed_number;

use std::num::NonZeroI32;

pub use convenience_number::*;
pub use floating_point::*;
pub use number::*;
pub use number_with_size::*;
pub use signed_number::*;

pub trait Integer: Number {
    type NonZeroType;
}

impl Integer for i32 {
    type NonZeroType = NonZeroI32;
}

pub trait SignedInteger: Integer + SignedNumber {}

impl SignedInteger for i32 {}

pub trait UnsignedInteger: Integer {}
