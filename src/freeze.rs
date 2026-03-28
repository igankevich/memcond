/// This trait marks types that are internally immutable.
///
/// Has the same meaning as [`Freeze`](core::marker::Freeze) but can be implemented for types
/// outside the standard library.
///
/// # Safety
///
/// It's safe to implement this trait for any type that is _not_ internally mutable,
/// i.e. can't be changed through an immutable reference.
pub unsafe trait Freeze {}

macro_rules! impl_freeze {
    ($($type:ty)+) => {
        $(unsafe impl Freeze for $type {})+
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_freeze_1 {
    ($($type:ty)+) => {
        $(
            //#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
            unsafe impl<T: Freeze> Freeze for $type {}
        )+
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_freeze_2 {
    ($($type:ty)+) => {
        $(
            unsafe impl<T0: Freeze, T1: Freeze> Freeze for $type {}
        )+
    };
}

impl_freeze! {
    ()
    bool
    char
    u8
    u16
    u32
    u64
    u128
    usize
    i8
    i16
    i32
    i64
    i128
    isize
    f32
    f64
    core::num::NonZero<char>
    core::num::NonZero<u8>
    core::num::NonZero<u16>
    core::num::NonZero<u32>
    core::num::NonZero<u64>
    core::num::NonZero<u128>
    core::num::NonZero<usize>
    core::num::NonZero<i8>
    core::num::NonZero<i16>
    core::num::NonZero<i32>
    core::num::NonZero<i64>
    core::num::NonZero<i128>
    core::num::NonZero<isize>
    core::net::Ipv4Addr
    core::net::Ipv6Addr
    core::net::IpAddr
    core::net::SocketAddrV4
    core::net::SocketAddrV6
    core::net::SocketAddr
    core::time::Duration
}

#[cfg(feature = "alloc")]
impl_freeze! {
    alloc::string::String
    alloc::ffi::CString
}

#[cfg(feature = "std")]
impl_freeze! {
    std::ffi::OsString
    std::path::PathBuf
}

impl_freeze_1! {
    core::ops::Range<T>
    core::ops::RangeInclusive<T>
    core::ops::RangeFrom<T>
    core::ops::RangeTo<T>
    core::ops::RangeToInclusive<T>
    core::ops::Bound<T>
}

#[cfg(feature = "alloc")]
impl_freeze_1! {
    alloc::boxed::Box<T>
    alloc::vec::Vec<T>
    alloc::collections::VecDeque<T>
    alloc::collections::BTreeSet<T>
    alloc::collections::LinkedList<T>
    alloc::collections::BinaryHeap<T>
    Option<T>
}

#[cfg(feature = "std")]
impl_freeze_1! {
    std::collections::HashSet<T>
}

#[cfg(feature = "alloc")]
impl_freeze_2! {
    alloc::collections::BTreeMap<T0, T1>
}

#[cfg(feature = "std")]
impl_freeze_2! {
    std::collections::HashMap<T0, T1>
}

unsafe impl<T: Freeze, const N: usize> Freeze for [T; N] {}

macro_rules! impl_freeze_tuple {
    ($(($head:ident $($tail:ident)*))+) => {
        $(unsafe impl<$head: Freeze $(, $tail: Freeze)*> Freeze for ($head $(, $tail)*,) {})+
    };
}

impl_freeze_tuple! {
    (T0)
    (T0 T1)
    (T0 T1 T2)
    (T0 T1 T2 T3)
    (T0 T1 T2 T3 T4)
    (T0 T1 T2 T3 T4 T5)
    (T0 T1 T2 T3 T4 T5 T6)
    (T0 T1 T2 T3 T4 T5 T6 T7)
    (T0 T1 T2 T3 T4 T5 T6 T7 T8)
    (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9)
}
