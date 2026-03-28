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
        impl $crate::private::core::ops::Deref for $outer_type {
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

#[doc(hidden)]
#[macro_export]
macro_rules! derive {
    (
        () // Not yet checked attributes.
        ($($attr:ident)*) // Already checked attributes.
        ($outer_type:ident) // The name of the struct.
        $(($inner_name:ident $inner_type:ty))+ // Struct fields.
    ) => {
        #[derive($($crate::private::$attr,)*)]
        pub struct $outer_type {
            $($inner_name: $inner_type,)+
        }
    };
    // The rest of the macro checks which derive implementations are safe to use with the generated struct.
    // We can only use those implementation that doesn't make our precondition fail.
    ((Clone $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Clone) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((Copy $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Copy) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((PartialEq $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* PartialEq) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((Eq $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Eq) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((PartialOrd $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* PartialOrd) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((Ord $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Ord) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((Hash $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Hash) ($outer_type) $(($inner_name $inner_type))+}
    };
    ((Debug $($attr:ident)*) ($($checked_attr:ident)*) ($outer_type:ident) $(($inner_name:ident $inner_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Debug) ($outer_type) $(($inner_name $inner_type))+}
    };
}

#[macro_export]
macro_rules! memcond {
    (
        const fn $cond:ident($($inner_name:ident: &$inner_type:ty$(,)?)+) -> bool {
            $body:stmt
        }

        $(#[derive($($attr:ident$(,)?)*)])*
        $visibility:vis struct $outer_type:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            $crate::derive!{
                ($($($attr)*)*) () ($outer_type) $(($inner_name $inner_type))+
            }

            impl $outer_type {
                #[inline(always)]
                #[forbid(unsafe_code)]
                #[track_caller]
                pub const fn check($($inner_name: &$inner_type,)+) -> bool {
                    $body
                }

                #[inline(always)]
                const fn do_check(&self) -> bool {
                    Self::check($(&self.$inner_name,)+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($inner_name: $inner_type)+) ->
                    $crate::private::Result<Self, $crate::ret!(@return_type $($inner_type,)+)>
                where
                    $($inner_type: $crate::Freeze)+
                {
                    if !Self::check($(&$inner_name,)+) {
                        return $crate::private::Result::Err($crate::ret!(@return_expr $($inner_name,)+));
                    }
                    $crate::private::Result::Ok(Self { $($inner_name,)+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $inner_name(&self) -> &$inner_type {
                        $crate::private::core::debug_assert!(self.do_check());
                        unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
                        &self.$inner_name
                    }
                )+

                #[inline]
                #[track_caller]
                pub fn into_inner(self) -> $crate::ret!(@return_type $($inner_type,)+) {
                    $crate::private::core::debug_assert!(self.do_check());
                    unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
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

        $(#[derive($($attr:ident$(,)?)*)])*
        $visibility:vis struct $outer_type:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            $crate::derive!{
                ($($($attr)*)*) (Clone Copy) ($outer_type) $(($inner_name $inner_type))+
            }

            impl $outer_type {

                #[inline(always)]
                #[forbid(unsafe_code)]
                #[track_caller]
                pub const fn check($($inner_name: $inner_type,)+) -> bool {
                    $body
                }

                #[inline(always)]
                const fn do_check(&self) -> bool {
                    Self::check($(self.$inner_name,)+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($inner_name: $inner_type)+) ->
                    $crate::private::Result<Self, $crate::ret!(@return_type $($inner_type,)+)>
                where
                    $($inner_type: $crate::Freeze)+
                {
                    if !Self::check($($inner_name,)+) {
                        return $crate::private::Result::Err($crate::ret!(@return_expr $($inner_name,)+));
                    }
                    $crate::private::Result::Ok(Self { $($inner_name,)+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $inner_name(self) -> $inner_type {
                        $crate::private::core::debug_assert!(self.do_check());
                        unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
                        self.$inner_name
                    }
                )+
            }

            $crate::deref!($outer_type $($inner_name: $inner_type)+);
        }

        $visibility use self::$cond::$outer_type;
    };
}
