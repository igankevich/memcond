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
                self.do_check();
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

/// Same as [`memcond`](crate::memcond) but expects condition arguments to be passed by reference.
///
/// Additionally it generates `fn into_inner() -> Args` method which drops the structure
/// and returns condition arguments that were used to initialize it.
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
                const fn do_check(&self) {
                    $crate::private::core::debug_assert!(Self::check($(&self.$arg_name),+));
                    unsafe { $crate::private::core::hint::assert_unchecked(Self::check($(&self.$arg_name),+)) };
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
                        self.do_check();
                        &self.$arg_name
                    }
                )+

                #[inline]
                #[track_caller]
                pub fn into_inner(self) -> $crate::return_type!($(($arg_type))+) {
                    self.do_check();
                    $crate::return_expr!($((self.$arg_name))+)
                }
            }

            $crate::deref!($struct $(($arg_name: $arg_type))+);
        }

        $visibility use $cond::$struct;
    };
}

/// This macro generates a struct that checks the provided condition in the constructor
/// and enforces this condition via [`assert_unchecked`](core::hint::assert_unchecked)
/// in each getter.
///
/// The following methods are generated:
/// - `const fn new(args) -> Result<Self, Args>`. This is a constructor that checks the condition
///   and returns the provided arguments if it doesn't hold.
/// - `const fn arg0(self) -> Arg0`. This is a getter; it's generated for each argument. The getter
///   enforces the condition via `assert_unchecked`.
///
/// If there is only one argument, [`Deref`](core::ops::Deref) implementation is generated as well.
///
/// In addition to that the following derive macros from the standard library can be applied to
/// the structure: `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Debug`.
/// In general derive macros are unsafe in this context because they have access to the fields.
/// If you need some other trait implementations (e.g. `Serialize`, `Deserialize`), it's best to implement them
/// manually via `impl ... for ...`.
///
/// # Safety
///
/// - The generated structure is placed in its own module. This prevents implementations from
///   accessing its fields.
/// - The condition is written as `const fn` which prevents it from using global internally mutable variables.
/// - The condition forbids using unsafe code inside.
/// - Condition variables are internally immutable. This is enforced via [`Freeze`](crate::Freeze)
///   trait.
///
/// So, it shouldn't be possible to make the condition false after it was verified in the
/// constructor and all the arguments were safely placed in the struct.
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
                const fn do_check(&self) {
                    $crate::private::core::debug_assert!(Self::check($(self.$arg_name),+));
                    unsafe { $crate::private::core::hint::assert_unchecked(Self::check($(self.$arg_name),+)) };
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
                        self.do_check();
                        self.$arg_name
                    }
                )+
            }

            $crate::deref!($struct $(($arg_name: $arg_type))+);
        }

        $visibility use $cond::$struct;
    };
}
