use core::{
    cmp::{Ordering, Reverse},
    convert::Infallible,
    marker::{PhantomData, PhantomPinned},
    mem::ManuallyDrop,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
        Wrapping,
    },
    task::Poll,
    time::Duration,
};

#[rustversion::since(1.64)]
use core::ffi::CStr;

#[rustversion::since(1.74)]
use core::num::Saturating;

use crate::PortableOrd;

impl<T: PortableOrd> PortableOrd for Option<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd, E: PortableOrd> PortableOrd for Result<T, E> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT && E::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Ordering {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Reverse<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Infallible {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

#[rustversion::since(1.64)]
impl PortableOrd for CStr {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T> PortableOrd for PhantomData<T> {
    /// PhantomData is always `Ordering::Equal` as it holds no value, so the below is always true.
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for PhantomPinned {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for ManuallyDrop<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

macro_rules! impl_non_zero {
    ($int:ident) => {
        impl PortableOrd for $int {
            const CAN_USE_UNSTABLE_SORT: bool = true;
            const I_KNOW_WHAT_I_AM_DOING: () = ();
        }
    };
}

impl_non_zero!(NonZeroI8);
impl_non_zero!(NonZeroI16);
impl_non_zero!(NonZeroI32);
impl_non_zero!(NonZeroI64);
impl_non_zero!(NonZeroI128);
impl_non_zero!(NonZeroIsize);
impl_non_zero!(NonZeroU8);
impl_non_zero!(NonZeroU16);
impl_non_zero!(NonZeroU32);
impl_non_zero!(NonZeroU64);
impl_non_zero!(NonZeroU128);
impl_non_zero!(NonZeroUsize);

#[rustversion::since(1.74)]
impl<T: PortableOrd> PortableOrd for Saturating<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Wrapping<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

#[rustversion::since(1.79)]
impl<T: PortableOrd + core::ops::Deref<Target: PortableOrd>> PortableOrd for core::pin::Pin<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Poll<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Duration {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}
