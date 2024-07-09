#![no_std]

pub mod binary;

#[macro_export]
macro_rules! integer_enum {
    ($enum:ident, $repr:ty, { $( $variant:ident = $value:expr ),* $(,)? }) => {
        #[repr($repr)]
        #[derive(zerocopy_derive::AsBytes, Ord, PartialOrd, Eq, PartialEq)]
        enum $enum {
            $( $variant = $value ),*
        }

        impl core::convert::TryFrom<$repr> for $enum {
            type Error = $repr;

            fn try_from(value: $repr) -> Result<Self, Self::Error> {
                match value {
                    $( $value => Ok($enum::$variant), )*
                    _ => Err(value),
                }
            }
        }
    };
}
