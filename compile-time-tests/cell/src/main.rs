use core::cell::Cell;

use memcond::memcond_ref;

memcond_ref! {
    const fn check_cell(cell: &Cell<u8>) -> bool {
        cell.replace(123) == 100
    }

    pub struct CheckCell;
}

fn main() {
    CheckCell::new(Cell::new(123)).unwrap();
}
