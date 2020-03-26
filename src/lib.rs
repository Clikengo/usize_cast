//! Very simply library that allow compile time checked cast from and to `usize`/`isize`
//!
//! ```
//! use usize_cast::{IntoUsize, FromUsize, IntoIsize, FromIsize};
//!
//! assert_eq!(1u64.into_usize(), 1usize);
//! assert_eq!(2i64.into_isize(), 2isize);
//! assert_eq!(3i64.into_isize(), 3isize);
//! assert_eq!(1u32.into_usize(), 1usize);
//! assert_eq!(2i32.into_isize(), 2isize);
//! assert_eq!(3i32.into_isize(), 3isize);
//! assert_eq!(u64::max_value().into_usize(), usize::max_value());
//! assert_eq!(i64::max_value().into_isize(), isize::max_value());
//! assert_eq!(i64::min_value().into_isize(), isize::min_value());
//!
//! assert_eq!(1u64, u64::from_usize(1usize));
//! assert_eq!(2i64, i64::from_isize(2isize));
//! assert_eq!(3i64, i64::from_isize(3isize));
//! assert_eq!(u64::max_value(), u64::from_usize(usize::max_value()));
//! assert_eq!(i64::max_value(), i64::from_isize(isize::max_value()));
//! assert_eq!(i64::min_value(), i64::from_isize(isize::min_value()));
//! ```
#![no_std]

pub trait IntoUsize {
    fn into_usize(self) -> usize;
}

pub trait FromUsize {
    fn from_usize(u: usize) -> Self;
}

pub trait IntoIsize {
    fn into_isize(self) -> isize;
}

pub trait FromIsize {
    fn from_isize(u: isize) -> Self;
}

macro_rules! impl_x {
    ($width:expr, $ux:ident, $ix:ident $(, ($sux:ident, $six:ident) )*) => {
        #[cfg(target_pointer_width = $width)]
        mod $ux {
            use super::*;

            const _ASSERT_EQ_SIZE: fn() = || {
                let _ = core::mem::transmute::<$ux, usize>;
                let _ = core::mem::transmute::<$ix, isize>;
            };

            impl FromUsize for $ux {
                fn from_usize(u: usize) -> Self {
                    u as $ux
                }
            }

            impl FromIsize for $ix {
                fn from_isize(u: isize) -> Self {
                    u as $ix
                }
            }

            impl IntoUsize for $ux {
                fn into_usize(self) -> usize {
                    self as usize
                }
            }

            impl IntoIsize for $ix {
                fn into_isize(self) -> isize {
                    self as isize
                }
            }

            $(
                impl IntoUsize for $sux {
                    fn into_usize(self) -> usize {
                        self as usize
                    }
                }

                impl IntoIsize for $six {
                    fn into_isize(self) -> isize {
                        self as isize
                    }
                }
            )*

            #[cfg(test)]
            mod tests {
                use crate::*;

                #[test]
                fn it_works() {
                    assert_eq!((0 as $ux).into_usize(), 0usize);
                    assert_eq!((0 as $ix).into_isize(), 0isize);
                    assert_eq!((255 as $ux).into_usize(), 255usize);
                    assert_eq!((255 as $ix).into_isize(), 255isize);
                    assert_eq!((-255 as $ix).into_isize(), -255isize);
                    assert_eq!((256 as $ux).into_usize(), 256usize);
                    assert_eq!((256 as $ix).into_isize(), 256isize);
                    assert_eq!((-256 as $ix).into_isize(), -256isize);
                    assert_eq!($ux::max_value().into_usize(), usize::max_value());
                    assert_eq!($ix::max_value().into_isize(), isize::max_value());
                    assert_eq!($ix::min_value().into_isize(), isize::min_value());
                }
            }
        }
    };
}

impl_x!("16", u16, i16);
impl_x!("32", u32, i32, (u16, i16));
impl_x!("64", u64, i64, (u32, i32), (u16, i16));
