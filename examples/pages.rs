use core::hint::black_box;
use core::mem::MaybeUninit;

use memcond::memcond;

const MAX_PAGE: u32 = u32::MAX / 4096;

const NUM_BITS_PER_UNDERLYING_PAGE: u16 = 4096 * u8::BITS as u16;
const NUM_PAGES: u32 = MAX_PAGE + 1;
const NUM_UNDERLYING_PAGES: u8 = (NUM_PAGES / NUM_BITS_PER_UNDERLYING_PAGE as u32) as u8;
const NUM_BITS_PER_BITSET: u8 = 2 * u64::BITS as u8;
const NUM_BITSETS: u16 = NUM_BITS_PER_UNDERLYING_PAGE / NUM_BITS_PER_BITSET as u16;

memcond! {
    const fn is_page_number(i: u8, j: u8, k: u8) -> bool {
        i < NUM_UNDERLYING_PAGES && j < u8::MAX && k < NUM_BITS_PER_BITSET
    }

    #[derive(Clone, Copy)]
    pub struct PageNumber;
}

impl PageNumber {
    pub const fn from_page_number(page: u32) -> Result<Self, (u8, u8, u8)> {
        let i = (page / NUM_BITS_PER_UNDERLYING_PAGE as u32) as u8;
        let tmp = (page % NUM_BITS_PER_UNDERLYING_PAGE as u32) as u16;
        let j = (tmp / NUM_BITSETS) as u8;
        let k = (tmp % NUM_BITSETS) as u8;
        Self::new(i, j, k)
    }
}

fn main() {
    eprintln!("NUM_PAGES = {NUM_PAGES}");
    eprintln!("NUM_BITS_PER_UNDERLYING_PAGE = {NUM_BITS_PER_UNDERLYING_PAGE}");
    eprintln!("NUM_UNDERLYING_PAGES = {NUM_UNDERLYING_PAGES}");
    eprintln!("NUM_BITS_PER_BITSET = {NUM_BITS_PER_BITSET}");
    eprintln!("NUM_BITSETS = {NUM_BITSETS}");
    let mut mapped_pages: Box<
        MaybeUninit<[[[u64; 2]; NUM_BITSETS as usize]; NUM_UNDERLYING_PAGES as usize]>,
    > = Box::new_uninit();
    // SAFETY: Zeroes are valid memory contents for a bitset.
    let mut mapped_pages: Box<[[[u64; 2]; NUM_BITSETS as usize]; NUM_UNDERLYING_PAGES as usize]> = unsafe {
        mapped_pages.as_mut_ptr().write_bytes(0_u8, 1);
        mapped_pages.assume_init()
    };
    let page = black_box(PageNumber::from_page_number(123).unwrap());
    mapped_pages[page.i() as usize][page.j() as usize][(page.k() / 64) as usize] =
        1_u64 << (page.k() % u64::BITS as u8);
    eprintln!(
        "{:064b}",
        mapped_pages[page.i() as usize][page.j() as usize][(page.k() / 64) as usize]
    );
}
