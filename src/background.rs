use core::fmt;
use rgb::RGB8;

/// Adds a background color
pub trait Background: Sized {
    /// Adds the given background color
    fn bg(self, rgb: RGB8) -> WithBackground<Self>;
}

/// Something with a background color
pub struct WithBackground<T> {
    t: T,
    rgb: RGB8
}

impl<T> Background for T {
    fn bg(self, rgb: RGB8) -> WithBackground<Self> {
        WithBackground {
            t: self,
            rgb
        }
    }
}

macro_rules! impl_me {
    ($bound:path, $format_arg:expr) => {
        impl<T> $bound for WithBackground<T>
        where T: $bound {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!("\x1B[48;2;{};{};{}m", $format_arg, "\x1B[0m"), self.rgb.r, self.rgb.g, self.rgb.b, self.t)
            }
        }
    };
}

impl_me!(fmt::Binary, "{:b}");
impl_me!(fmt::Debug, "{:?}");
impl_me!(fmt::Display, "{}");
impl_me!(fmt::LowerExp, "{:e}");
impl_me!(fmt::LowerHex, "{:x}");
impl_me!(fmt::Octal, "{:o}");
impl_me!(fmt::Pointer, "{:p}");
impl_me!(fmt::UpperExp, "{:E}");
impl_me!(fmt::UpperHex, "{:X}");