use crate::PortableOrd;

macro_rules! trivial_portable_ord {
    ($($ty:ty),*) => {
        $(
            impl PortableOrd for $ty {
                const CAN_USE_UNSTABLE_SORT: bool = true;
                const I_KNOW_WHAT_I_AM_DOING: () = ();
            }
        )*
    };
}

trivial_portable_ord!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl PortableOrd for bool {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for char {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for str {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

macro_rules! impl_hash_tuple {
    () => (

        impl PortableOrd for () {
            const CAN_USE_UNSTABLE_SORT: bool = true;
            const I_KNOW_WHAT_I_AM_DOING: () = ();
        }
    );

    ( $($name:ident)+) => (
        maybe_tuple_doc! {
            $($name)+ @

            impl<$($name: PortableOrd),+> PortableOrd for ($($name,)+) {
                const CAN_USE_UNSTABLE_SORT: bool = $( $name::CAN_USE_UNSTABLE_SORT && )+ true;
                const I_KNOW_WHAT_I_AM_DOING: () = ();
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

impl<T: PortableOrd> PortableOrd for [T] {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd, const LEN: usize> PortableOrd for [T; LEN] {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for &T {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for &mut T {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}
