use std::{
    fmt::Debug, hash::Hash, ops::{Add, AddAssign, SubAssign}
};
extern crate generic_array;

macro_rules! idx_impl {
    ($t:ident, $t1:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $t {
            val: $t1,
        }

        impl Add<$t> for $t {
            type Output = $t;
            #[inline(always)]
            fn add(self, rhs: $t) -> Self::Output {
                $t {
                    val: self.val + rhs.val,
                }
            }
        }

        impl AddAssign<$t> for $t {
            #[inline(always)]
            fn add_assign(&mut self, rhs: $t) {
                self.val += rhs.val;
            }
        }

        impl SubAssign<$t> for $t {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: $t) {
                self.val -= rhs.val;
            }
        }

        impl From<usize> for $t {
            #[inline(always)]
            fn from(val: usize) -> Self {
                $t { val: val as $t1 }
            }
        }

        impl From<$t1> for $t {
            #[inline(always)]
            fn from(val: $t1) -> Self {
                $t { val }
            }
        }

        impl From<$t> for usize {
            #[inline(always)]
            fn from(val: $t) -> Self {
                val.val as usize
            }
        }

        impl From<$t> for $t1 {
            #[inline(always)]
            fn from(val: $t) -> Self {
                val.val as $t1
            }
        }

        impl Idx<$t> for $t {}
    };
}

pub trait Idx<TNum>:
    Into<usize> + Add<TNum, Output = TNum> + From<usize> + AddAssign<TNum>+SubAssign<TNum>+ Clone+Copy+Debug+PartialEq+Eq+PartialOrd+Ord+Hash+Default
{

}
#[cfg(any(target_pointer_width="8",target_pointer_width="16",target_pointer_width="32",target_pointer_width="64"))]
idx_impl!(U8, u8);
#[cfg(any(target_pointer_width="16",target_pointer_width="32",target_pointer_width="64"))]
idx_impl!(U16, u16);
#[cfg(any(target_pointer_width="32",target_pointer_width="64",target_pointer_width="128"))]
idx_impl!(U32, u32);
#[cfg(any(target_pointer_width="64",target_pointer_width="128"))]
idx_impl!(U64, u64);
#[cfg(target_pointer_width="128")]
idx_impl!(U128, u128);