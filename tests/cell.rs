use std::cell::Cell;

use memcond::memcond_copy;

const CELL: Cell<u32> = Cell::new(123);

memcond_copy! {
    const fn is_page_address(address: u64) -> bool {
        address % 4096 == 0 && CELL.replace(321) == 123
    }

    pub struct PageAddress;
}

memcond_copy! {
    const fn is_finite(number: f64) -> bool {
        number.is_finite()
    }

    pub struct FiniteF64;
}

#[test]
fn cell_works() {
    let page = PageAddress::new(4096).unwrap();
    let _page2 = PageAddress::new(4096).unwrap();
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.get());
    assert_eq!(4096, page.address());
}
