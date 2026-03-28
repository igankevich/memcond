extern crate alloc;

use memcond::memcond;

memcond! {
    const fn is_non_empty(inner: &alloc::vec::Vec<u8>) -> bool {
        !inner.is_empty()
    }

    #[derive(Clone)]
    pub struct NonEmptyVec;
}

#[test]
fn vec_works() {
    let vec1 = NonEmptyVec::new(vec![1_u8]).unwrap();
    let vec2 = NonEmptyVec::new(vec![1_u8, 2]).unwrap();
    assert_eq!(1, vec1.inner().len());
    assert_eq!(2, vec2.len());
}
