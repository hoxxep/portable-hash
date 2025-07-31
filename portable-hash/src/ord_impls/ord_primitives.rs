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

    ( $(($name:ident $other:ident))+) => (
        maybe_tuple_doc! {
            $($name)+ @

            impl<$($name: PortableOrd),+> PortableOrd for ($($name,)+) where last_type!($($name,)+): ?Sized {
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

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_hash_tuple! {}
impl_hash_tuple! { (T TO) }
impl_hash_tuple! { (T TO) (B BO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) (H HO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) (H HO) (I IO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) (H HO) (I IO) (J JO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) (H HO) (I IO) (J JO) (K KO) }
impl_hash_tuple! { (T TO) (B BO) (C CO) (D DO) (E EO) (F FO) (G GO) (H HO) (I IO) (J JO) (K KO) (L LO) }

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
