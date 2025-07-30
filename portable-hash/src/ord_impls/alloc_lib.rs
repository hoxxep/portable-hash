extern crate alloc;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    ffi::CString,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};

use crate::PortableOrd;

impl<T: PortableOrd + ToOwned> PortableOrd for Cow<'_, T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Box<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Rc<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Arc<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<K, V> PortableOrd for BTreeMap<K, V>
where
    K: PortableOrd,
    V: PortableOrd,
{
    const CAN_USE_UNSTABLE_SORT: bool = K::CAN_USE_UNSTABLE_SORT && V::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<K> PortableOrd for BTreeSet<K>
where
    K: PortableOrd,
{
    const CAN_USE_UNSTABLE_SORT: bool = K::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for LinkedList<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for VecDeque<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for CString {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for String {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl<T: PortableOrd> PortableOrd for Vec<T> {
    const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}
