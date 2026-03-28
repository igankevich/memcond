#[macro_export]
macro_rules! memcond {
    (
        $visibility:vis struct $outer_type:ident {
            $($inner_name:ident: $inner_type:ty,)+
        }

        impl $outer_type_2:ident {
            const fn $cond:ident(&$self:ident) -> bool {
                $body:stmt
            }
        }
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            #[derive(Debug)]
            pub struct $outer_type {
                $($inner_name: $inner_type,)+
            }

            impl $outer_type {
                //#[inline]
                //#[track_caller]
                //pub const fn new($($inner_name: $inner_type)+) -> Self {
                //    assert!(check($(&$inner_name,)+));
                //    Self { $($inner_name,)+ }
                //}

                #[inline(always)]
                #[forbid(unsafe_code)]
                #[track_caller]
                const fn check(&$self) -> bool {
                    $body
                }

                #[inline]
                #[track_caller]
                pub const fn new($($inner_name: $inner_type)+) ->
                    Result<Self, $crate::memcond!(@return_type $($inner_type,)+)>
                where
                    $($inner_type: $crate::Freeze)+
                {
                    let value = Self { $($inner_name,)+ };
                    if !value.check() {
                        return Err($crate::memcond!(@return_expr $($inner_name,)+));
                    }
                    Ok(value)
                }

                #[inline]
                pub const fn get(&self) -> $crate::memcond!(@return_type_ref $(&$inner_type,)+) {
                    debug_assert!(self.check());
                    unsafe { core::hint::assert_unchecked(self.check()) };
                    $crate::memcond!(@return_expr $(&self.$inner_name,)+)
                }

                #[inline]
                #[track_caller]
                pub /*const*/ fn into_inner(self) -> $crate::memcond!(@return_type $($inner_type,)+) {
                    debug_assert!(self.check());
                    unsafe { core::hint::assert_unchecked(self.check()) };
                    $crate::memcond!(@return_expr $(self.$inner_name,)+)
                }
            }
        }

        $visibility use self::$cond::$outer_type;
    };
    (@return_type $arg:ty,) => {
        $arg
    };
    (@return_type $arg:ty, $($rest:ty,)+) => {
        ($arg, $($rest)+)
    };
    (@return_type_ref &$arg:ty,) => {
        &$arg
    };
    (@return_type_ref &$arg:ty, $(&$rest:ty,)+) => {
        (&$arg, $(&$rest)+)
    };
    (@return_expr $arg:expr,) => {
        $arg
    };
    (@return_expr $arg:expr, $($rest:expr,)+) => {
        ($arg, $($rest)+)
    };
}
