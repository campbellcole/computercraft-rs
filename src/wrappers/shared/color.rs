use serde_json::Value;
use thiserror::Error;

macro_rules! define_colors {
    ($(
        $name:ident = $value:expr
    ),*) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u64)]
        pub enum Color {
            $($name = $value),*
        }

        impl Color {
            pub fn colors() -> [Color; 16] {
                [
                    $(Color::$name),*
                ]
            }
        }

        impl TryFrom<u64> for Color {
            type Error = TryFromColorError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Color::$name),)*
                    _ => Err(TryFromColorError::InvalidColorValue(value)),
                }
            }
        }
    };
}

#[derive(Debug, Error)]
pub enum TryFromColorError {
    #[error("invalid color value: {0}")]
    InvalidColorValue(u64),
}

define_colors! {
    White = 1,
    Orange = 2,
    Magenta = 4,
    LightBlue = 8,
    Yellow = 16,
    Lime = 32,
    Pink = 64,
    Gray = 128,
    LightGray = 256,
    Cyan = 512,
    Purple = 1024,
    Blue = 2048,
    Brown = 4096,
    Green = 8192,
    Red = 16384,
    Black = 32768
}

impl From<Color> for u64 {
    fn from(color: Color) -> Self {
        color as u64
    }
}

impl From<Color> for Value {
    fn from(color: Color) -> Self {
        Value::Number((color as u64).into())
    }
}
