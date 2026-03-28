#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod freeze;
mod macros;

pub use self::freeze::*;

/// We re-export every external symbol that we use in the macros to make sure that it's not replaced
/// with something unsafe.
///
/// You shouldn't use any of these symbols directly.
#[doc(hidden)]
pub mod private {
    pub use core;
    pub use core::clone::Clone;
    pub use core::cmp::Eq;
    pub use core::cmp::Ord;
    pub use core::cmp::PartialEq;
    pub use core::cmp::PartialOrd;
    pub use core::fmt::Debug;
    pub use core::hash::Hash;
    pub use core::marker::Copy;
    pub use core::result::Result;
}
