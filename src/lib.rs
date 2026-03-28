#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod freeze;
mod macros;

pub use self::freeze::*;

//impl PageAddress {
//    pub fn change(&mut self) {
//        self.address = 123;
//    }
//}

//memcond! {
//    const fn is_non_empty(vec: &Vec<u8>) -> bool {
//        !vec.is_empty()
//    }
//
//    pub struct NonEmptyVec;
//}
