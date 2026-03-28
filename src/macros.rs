#[doc(hidden)]
#[macro_export]
macro_rules! ret {
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

#[doc(hidden)]
#[macro_export]
macro_rules! deref {
    ($outer_type:ident $inner_name:ident: $inner_type:ty) => {
        impl core::ops::Deref for $outer_type {
            type Target = $inner_type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$inner_name
            }
        }
    };
    ($outer_type:ident $inner_name:ident: $inner_type:ty $(, $rest_name:ident: $rest_type:ty)+) => {
        // No Deref implementation for multiple arguments.
    };
}

// TODO whitelist derives

#[macro_export]
macro_rules! memcond {
    (
        const fn $cond:ident($($inner_name:ident: &$inner_type:ty$(,)?)+) -> bool {
            $body:stmt
        }

        $visibility:vis struct $outer_type:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            #[inline(always)]
            #[forbid(unsafe_code)]
            #[track_caller]
            const fn check($($inner_name: &$inner_type,)+) -> bool {
                $body
            }

            #[derive(Debug)]
            pub struct $outer_type {
                $($inner_name: $inner_type,)+
            }

            impl $outer_type {
                #[inline(always)]
                const fn check(&self) -> bool {
                    check($(&self.$inner_name,)+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($inner_name: $inner_type)+) ->
                    Result<Self, $crate::ret!(@return_type $($inner_type,)+)>
                where
                    $($inner_type: $crate::Freeze)+
                {
                    if !check($(&$inner_name,)+) {
                        return Err($crate::ret!(@return_expr $($inner_name,)+));
                    }
                    Ok(Self { $($inner_name,)+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $inner_name(&self) -> &$inner_type {
                        debug_assert!(self.check());
                        unsafe { core::hint::assert_unchecked(self.check()) };
                        &self.$inner_name
                    }
                )+

                #[inline]
                #[track_caller]
                pub fn into_inner(self) -> $crate::ret!(@return_type $($inner_type,)+) {
                    debug_assert!(self.check());
                    unsafe { core::hint::assert_unchecked(self.check()) };
                    $crate::ret!(@return_expr $(self.$inner_name,)+)
                }
            }

            $crate::deref!($outer_type $($inner_name: $inner_type)+);
        }

        $visibility use self::$cond::$outer_type;
    };
}

#[macro_export]
macro_rules! memcond_copy {
    (
        const fn $cond:ident($($inner_name:ident: $inner_type:ty$(,)?)+) -> bool {
            $body:stmt
        }

        $visibility:vis struct $outer_type:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            #[inline(always)]
            #[forbid(unsafe_code)]
            #[track_caller]
            const fn check($($inner_name: $inner_type,)+) -> bool {
                $body
            }

            #[derive(Debug, Clone, Copy)]
            pub struct $outer_type {
                $($inner_name: $inner_type,)+
            }

            impl $outer_type {
                #[inline(always)]
                const fn check(&self) -> bool {
                    check($(self.$inner_name,)+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($inner_name: $inner_type)+) ->
                    Result<Self, $crate::ret!(@return_type $($inner_type,)+)>
                where
                    $($inner_type: $crate::Freeze)+
                {
                    if !check($($inner_name,)+) {
                        return Err($crate::ret!(@return_expr $($inner_name,)+));
                    }
                    Ok(Self { $($inner_name,)+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $inner_name(self) -> $inner_type {
                        debug_assert!(self.check());
                        unsafe { core::hint::assert_unchecked(self.check()) };
                        self.$inner_name
                    }
                )+
            }

            $crate::deref!($outer_type $($inner_name: $inner_type)+);
        }

        $visibility use self::$cond::$outer_type;
    };
}
