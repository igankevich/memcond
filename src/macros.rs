#[doc(hidden)]
#[macro_export]
macro_rules! return_type {
    (($arg:ty)) => {
        $arg
    };
    (($arg:ty) $(($rest:ty))+) => {
        ($arg$(, $rest)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! return_expr {
    (($arg:expr)) => {
        $arg
    };
    (($arg:expr) $(($rest:expr))+) => {
        ($arg$(, $rest)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! deref {
    ($struct:ident ($arg_name:ident: $arg_type:ty)) => {
        impl $crate::private::core::ops::Deref for $struct {
            type Target = $arg_type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$arg_name
            }
        }
    };
    ($struct:ident ($arg_name:ident: $arg_type:ty) $(($rest_name:ident: $rest_type:ty))+) => {
        // No Deref implementation for multiple arguments.
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! derive {
    (
        () // Not yet checked attributes.
        ($($attr:ident)*) // Already checked attributes.
        ($struct:ident) // The name of the struct.
        $(($arg_name:ident $arg_type:ty))+ // Struct fields.
    ) => {
        #[derive($($crate::private::$attr,)*)]
        pub struct $struct {
            $($arg_name: $arg_type,)+
        }
    };
    // The rest of the macro checks which derive implementations are safe to use with the generated struct.
    // We can only use those implementation that doesn't make our precondition fail.
    ((Clone $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Clone) ($struct) $(($arg_name $arg_type))+}
    };
    ((Copy $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Copy) ($struct) $(($arg_name $arg_type))+}
    };
    ((PartialEq $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* PartialEq) ($struct) $(($arg_name $arg_type))+}
    };
    ((Eq $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Eq) ($struct) $(($arg_name $arg_type))+}
    };
    ((PartialOrd $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* PartialOrd) ($struct) $(($arg_name $arg_type))+}
    };
    ((Ord $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Ord) ($struct) $(($arg_name $arg_type))+}
    };
    ((Hash $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Hash) ($struct) $(($arg_name $arg_type))+}
    };
    ((Debug $($attr:ident)*) ($($checked_attr:ident)*) ($struct:ident) $(($arg_name:ident $arg_type:ty))+) => {
        $crate::derive!{($($attr)*) ($($checked_attr)* Debug) ($struct) $(($arg_name $arg_type))+}
    };
}

#[macro_export]
macro_rules! memcond_ref {
    (
        const fn $cond:ident($($arg_name:ident: &$arg_type:ty),+) -> bool $body:block

        $(#[derive($($attr:ident),*)])*
        $visibility:vis struct $struct:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            $crate::derive!{
                ($($($attr)*)*) () ($struct) $(($arg_name $arg_type))+
            }

            impl $struct {
                #[inline(always)]
                #[forbid(unsafe_code)]
                #[track_caller]
                pub const fn check($($arg_name: &$arg_type),+) -> bool {
                    $body
                }

                #[inline(always)]
                const fn do_check(&self) -> bool {
                    Self::check($(&self.$arg_name),+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($arg_name: $arg_type),+) ->
                    $crate::private::Result<Self, $crate::return_type!($(($arg_type))+)>
                where
                    $($arg_type: $crate::Freeze,)+
                {
                    if !Self::check($(&$arg_name),+) {
                        return $crate::private::Result::Err($crate::return_expr!($(($arg_name))+));
                    }
                    $crate::private::Result::Ok(Self { $($arg_name,)+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $arg_name(&self) -> &$arg_type {
                        $crate::private::core::debug_assert!(self.do_check());
                        unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
                        &self.$arg_name
                    }
                )+

                #[inline]
                #[track_caller]
                pub fn into_inner(self) -> $crate::return_type!($(($arg_type))+) {
                    $crate::private::core::debug_assert!(self.do_check());
                    unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
                    $crate::return_expr!($((self.$arg_name))+)
                }
            }

            $crate::deref!($struct $(($arg_name: $arg_type))+);
        }

        $visibility use self::$cond::$struct;
    };
}

#[macro_export]
macro_rules! memcond {
    (
        const fn $cond:ident($($arg_name:ident: $arg_type:ty),+) -> bool $body:block

        $(#[derive($($attr:ident),*)])*
        $visibility:vis struct $struct:ident;
    ) => {
        mod $cond {
            #[allow(unused)]
            use super::*;

            $crate::derive!{
                ($($($attr)*)*) () ($struct) $(($arg_name $arg_type))+
            }

            impl $struct {

                #[inline(always)]
                #[forbid(unsafe_code)]
                #[track_caller]
                pub const fn check($($arg_name: $arg_type),+) -> bool $body

                #[inline(always)]
                const fn do_check(&self) -> bool {
                    Self::check($(self.$arg_name),+)
                }

                #[inline]
                #[track_caller]
                pub const fn new($($arg_name: $arg_type,)+) ->
                    $crate::private::Result<Self, $crate::return_type!($(($arg_type))+)>
                where
                    $($arg_type: $crate::Freeze,)+
                {
                    if !Self::check($($arg_name,)+) {
                        return $crate::private::Result::Err($crate::return_expr!($(($arg_name))+));
                    }
                    $crate::private::Result::Ok(Self { $($arg_name),+ })
                }

                $(
                    #[inline]
                    #[track_caller]
                    pub const fn $arg_name(self) -> $arg_type {
                        $crate::private::core::debug_assert!(self.do_check());
                        unsafe { $crate::private::core::hint::assert_unchecked(self.do_check()) };
                        self.$arg_name
                    }
                )+
            }

            $crate::deref!($struct $(($arg_name: $arg_type))+);
        }

        $visibility use self::$cond::$struct;
    };
}
