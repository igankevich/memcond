#![allow(clippy::ptr_arg)]

use core::hint::black_box;

use memcond::memcond_ref;

memcond_ref! {
    const fn check_yuv_len(data: &Vec<i16>, y_len: &u32, uv_len: &u32) -> bool {
        data.len() == *y_len as usize + 2 * *uv_len as usize
    }

    pub struct Yuv420pFrameInner;
}

struct Yuv420pFrame {
    inner: Yuv420pFrameInner,
}

impl Yuv420pFrame {
    pub fn new(width: u16, height: u16) -> Self {
        let y_len = u32::from(width) * u32::from(height);
        let uv_width = width.div_ceil(2);
        let uv_height = height.div_ceil(2);
        let uv_len = u32::from(uv_width) * u32::from(uv_height);
        Self {
            inner: Yuv420pFrameInner::new(
                vec![0_i16; y_len as usize + 2 * uv_len as usize],
                y_len,
                uv_len,
            )
            .expect("We pass correct values"),
        }
    }

    pub fn as_slices(&mut self) -> (&[i16], &[i16], &[i16]) {
        let (y, uv) = self.inner.data().split_at(*self.inner.y_len() as usize);
        let (u, v) = uv.split_at(*self.inner.uv_len() as usize);
        (y, u, v)
    }

    // TODO What about `as_mut_slices()`?
}

fn main() {
    let mut frame = black_box(Yuv420pFrame::new(black_box(320), black_box(200)));
    let (y, _u, _v) = frame.as_slices();
    eprintln!("{}", y[100]);
}
