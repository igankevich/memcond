use std::cell::Cell;

use memcond::memcond;

#[allow(clippy::declare_interior_mutable_const)]
const CELL: Cell<u32> = Cell::new(123);

memcond! {
    const fn is_page_address(address: u64) -> bool {
        address % 4096 == 0 && CELL.replace(321) == 123
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PageAddress;
}

memcond! {
    const fn is_finite(number: f64) -> bool {
        number.is_finite()
    }

    #[derive(Clone, Copy)]
    pub struct FiniteF64;
}

memcond! {
    const fn number_in_range(number: u32, min: u32, max: u32) -> bool {
        min <= number && number <= max
    }

    #[derive(Clone, Copy)]
    pub struct NumberInRange;
}

#[allow(clippy::borrow_interior_mutable_const)]
#[test]
fn cell_works() {
    let page = PageAddress::new(4096).unwrap();
    let _page2 = PageAddress::new(4096).unwrap();
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.get());
    assert_eq!(4096, page.address());
}

#[test]
fn number_in_range_works() {
    assert!(NumberInRange::new(10, 0, 100).is_ok());
    assert!(NumberInRange::new(0, 0, 100).is_ok());
    assert!(NumberInRange::new(100, 0, 100).is_ok());
    assert!(NumberInRange::new(101, 0, 100).is_err());
}
