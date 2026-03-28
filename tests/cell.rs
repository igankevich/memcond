use std::cell::Cell;

use memcond::memcond;

const CELL: Cell<u32> = Cell::new(123);

memcond! {
    pub struct PageAddress {
        address: u64,
    }

    impl PageAddress {
        const fn is_page_address(&self) -> bool {
            self.address % 4096 == 0 && CELL.replace(321) == 123
        }
    }
}

#[test]
fn cell_works() {
    let _page = PageAddress::new(4096).unwrap();
    let _page2 = PageAddress::new(4096).unwrap();
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.replace(321));
    assert_eq!(123, crate::CELL.get());
}
