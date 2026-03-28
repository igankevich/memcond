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
    ($(($(#[cfg($attr:meta)])? $((params $($param:ident)+))? (typename $type:ty)))+) => {
        $(
            $(
                #[cfg($attr)]
                #[cfg_attr(docsrs, doc(cfg($attr)))]
            )?
            unsafe impl$(<$($param: Freeze),+>)? Freeze for $type {}
        )+
    };
}

impl_freeze! {
    ((typename ()))
    ((typename bool))
    ((typename char))
    ((typename u8))
    ((typename u16))
    ((typename u32))
    ((typename u64))
    ((typename u128))
    ((typename usize))
    ((typename i8))
    ((typename i16))
    ((typename i32))
    ((typename i64))
    ((typename i128))
    ((typename isize))
    ((typename f32))
    ((typename f64))
    ((typename core::num::NonZero<char>))
    ((typename core::num::NonZero<u8>))
    ((typename core::num::NonZero<u16>))
    ((typename core::num::NonZero<u32>))
    ((typename core::num::NonZero<u64>))
    ((typename core::num::NonZero<u128>))
    ((typename core::num::NonZero<usize>))
    ((typename core::num::NonZero<i8>))
    ((typename core::num::NonZero<i16>))
    ((typename core::num::NonZero<i32>))
    ((typename core::num::NonZero<i64>))
    ((typename core::num::NonZero<i128>))
    ((typename core::num::NonZero<isize>))
    ((typename core::net::Ipv4Addr))
    ((typename core::net::Ipv6Addr))
    ((typename core::net::IpAddr))
    ((typename core::net::SocketAddrV4))
    ((typename core::net::SocketAddrV6))
    ((typename core::net::SocketAddr))
    ((typename core::time::Duration))
    (#[cfg(feature = "alloc")] (typename alloc::string::String))
    (#[cfg(feature = "alloc")] (typename alloc::ffi::CString))
    (#[cfg(feature = "std")] (typename std::ffi::OsString))
    (#[cfg(feature = "std")] (typename std::path::PathBuf))
    ((params T) (typename core::ops::Range<T>))
    ((params T) (typename core::ops::RangeInclusive<T>))
    ((params T) (typename core::ops::RangeFrom<T>))
    ((params T) (typename core::ops::RangeTo<T>))
    ((params T) (typename core::ops::RangeToInclusive<T>))
    ((params T) (typename core::ops::Bound<T>))
    ((params T) (typename Option<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::boxed::Box<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::vec::Vec<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::collections::VecDeque<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::collections::BTreeSet<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::collections::LinkedList<T>))
    (#[cfg(feature = "alloc")] (params T) (typename alloc::collections::BinaryHeap<T>))
    (#[cfg(feature = "std")] (params T) (typename std::collections::HashSet<T>))
    (#[cfg(feature = "std")] (params T0 T1) (typename std::collections::HashMap<T0, T1>))
    (#[cfg(feature = "alloc")] (params T0 T1) (typename alloc::collections::BTreeMap<T0, T1>))
}

unsafe impl<T: Freeze, const N: usize> Freeze for [T; N] {}

macro_rules! impl_freeze_tuple {
    ($(($($param:ident)+))+) => {
        $(unsafe impl<$($param: Freeze),+> Freeze for ($($param,)+) {})+
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
