#![allow(clippy::ptr_arg)]
extern crate alloc;

use memcond::memcond_ref;

memcond_ref! {
    const fn is_non_empty(inner: &alloc::vec::Vec<u8>) -> bool {
        !inner.is_empty()
    }

    #[derive(Clone)]
    pub struct NonEmptyVec;
}

memcond_ref! {
    const fn len_in_range(vec: &alloc::vec::Vec<u8>, min: &usize, max: &usize) -> bool {
        let len = vec.len();
        *min <= len && len <= *max
    }

    #[derive(Clone)]
    pub struct VecInRange;
}

#[test]
fn vec_works() {
    let vec1 = NonEmptyVec::new(vec![1_u8]).unwrap();
    let vec2 = NonEmptyVec::new(vec![1_u8, 2]).unwrap();
    assert_eq!(1, vec1.inner().len());
    assert_eq!(2, vec2.len());
}

#[test]
fn vec_in_range_works() {
    assert!(VecInRange::new(vec![1_u8], 1, 10).is_ok());
    assert!(VecInRange::new(vec![], 1, 10).is_err());
}
