//! [`PortableHash`] implementations for standard library types.

use std::collections::BTreeSet;
use crate::{PortableHash, PortableHasher};

impl<K> PortableHash for BTreeSet<K>
where
    K: PortableHash,
{
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_len_prefix(self.len());
        for item in self {
            item.portable_hash(state);
        }
    }
}

impl<K, V> PortableHash for std::collections::BTreeMap<K, V>
where
    K: PortableHash,
    V: PortableHash,
{
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_len_prefix(self.len());
        for (key, value) in self {
            key.portable_hash(state);
            value.portable_hash(state);
        }
    }
}

impl<T> PortableHash for std::collections::LinkedList<T>
where
    T: PortableHash,
{
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_len_prefix(self.len());
        for item in self {
            item.portable_hash(state);
        }
    }
}

impl<T> PortableHash for std::collections::VecDeque<T>
where
    T: PortableHash,
{
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_len_prefix(self.len());
        for item in self {
            item.portable_hash(state);
        }
    }
}
