//! [`PortableHash`] implementations for primitive types and core types.
//!
//! Modified from the `std::hash` module in the rust standard library.

use crate::{PortableHash, PortableHasher};

impl PortableHash for u8 {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(*self)
    }

    /// Override hash_slice to avoid writing a length prefix for bytes.
    #[inline]
    fn portable_hash_slice<H: PortableHasher>(data: &[u8], state: &mut H) {
        state.write_bytes(data)
    }
}

macro_rules! impl_write {
    ($(($ty:ident, $meth:ident),)*) => {$(

        /// We differ from the standard library implementation here, as we do _not_ allow
        /// [`portable_hash_slice`] to transmute unpadded numbers into a byte slice. This is
        /// because we need to control the endianness of the numbers when hashing, and a simple
        /// byte slice would not be portable across platforms. Likewise, `usize` and `isize` are
        /// not portable across platforms as a byte slice.
        impl PortableHash for $ty {
            #[inline]
            fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
                state.$meth(*self)
            }
        }
    )*}
}

impl_write! {
    (u16, write_u16),
    (u32, write_u32),
    (u64, write_u64),
    (u128, write_u128),
    (usize, write_usize),
    (i8, write_i8),
    (i16, write_i16),
    (i32, write_i32),
    (i64, write_i64),
    (i128, write_i128),
    (isize, write_isize),
}

impl PortableHash for bool {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(*self as u8)
    }
}

impl PortableHash for char {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u32(*self as u32)
    }
}

impl PortableHash for str {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_str(self);
    }
}

macro_rules! impl_hash_tuple {
    () => (

        impl PortableHash for () {
            #[inline]
            fn portable_hash<H: PortableHasher>(&self, _state: &mut H) {}
        }
    );

    ( $($name:ident)+) => (
        maybe_tuple_doc! {
            $($name)+ @

            impl<$($name: PortableHash),+> PortableHash for ($($name,)+) where last_type!($($name,)+): ?Sized {
                #[allow(non_snake_case)]
                #[inline]
                fn portable_hash<S: PortableHasher>(&self, state: &mut S) {
                    let ($(ref $name,)+) = *self;
                    $($name.portable_hash(state);)+
                }
            }
        }
    );
}

macro_rules! maybe_tuple_doc {
    ($a:ident @ $item:item) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        $item
    };
    ($a:ident $($rest_a:ident)+ @ $item:item) => {
        #[doc(hidden)]
        $item
    };
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_hash_tuple! {}
impl_hash_tuple! { T }
impl_hash_tuple! { T B }
impl_hash_tuple! { T B C }
impl_hash_tuple! { T B C D }
impl_hash_tuple! { T B C D E }
impl_hash_tuple! { T B C D E F }
impl_hash_tuple! { T B C D E F G }
impl_hash_tuple! { T B C D E F G H }
impl_hash_tuple! { T B C D E F G H I }
impl_hash_tuple! { T B C D E F G H I J }
impl_hash_tuple! { T B C D E F G H I J K }
impl_hash_tuple! { T B C D E F G H I J K L }

impl<T: PortableHash> PortableHash for [T] {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        PortableHash::portable_hash_slice(self, state)
    }
}

impl<T: ?Sized + PortableHash> PortableHash for &T {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        (**self).portable_hash(state);
    }
}

impl<T: ?Sized + PortableHash> PortableHash for &mut T {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        (**self).portable_hash(state);
    }
}

