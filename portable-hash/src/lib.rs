#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

/// A derive macro for [`PortableHash`].
pub use portable_hash_macros::PortableHash;

pub trait PortableHash {
    /// Computes the hash of the object using the specified hasher.
    fn portable_hash<H: PortableHasher>(&self, state: &mut H);

    /// Feed a slice of this type into the given [`PortableHasher`].
    fn portable_hash_slice<H: PortableHasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        // TODO(stability): we diverge from the standard library here by adding a length prefix.
        // the standard library implements the len_prefix in `impl PortableHasher for [T]`, but we
        // allow hash_slice to be overridden by some types to instead use the `write_bytes` method,
        // which allows the hasher to determine how/whether to handle a length prefix in these cases.
        state.write_len_prefix(data.len());
        for item in data.iter() {
            item.portable_hash(state);
        }
    }
}

pub trait PortableHasher {
    /// Finalizes the hash computation and returns the hash value.
    fn finish(&self) -> u64;

    /// Writes a byte slice to the hasher.
    fn write(&mut self, bytes: &[u8]);

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write(&[i]);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_i64(i as i64);
    }

    #[inline]
    fn write_len_prefix(&mut self, len: usize) {
        self.write_usize(len);
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write_len_prefix(s.as_bytes().len());
        self.write(s.as_bytes());
    }

    // TODO(stabilisation): review the addition of write_bytes.
    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_len_prefix(bytes.len());
        self.write(bytes);
    }
}

/// An alternative to `PortableHasher::finish` with a hasher-specific output type.
pub trait PortableHasherDigest {
    type Output;

    // TODO: review the naming and addition of this method.
    fn digest(&self) -> Self::Output;
}

pub trait BuildPortableHasher {
    type PortableHasher: PortableHasher;

    /// Creates a new instance of the hasher.
    fn build_hasher(&self) -> Self::PortableHasher;

    fn hash_one<T>(&self, x: T) -> u64
    where
        T: PortableHash,
    {
        let mut hasher = self.build_hasher();
        x.portable_hash(&mut hasher);
        hasher.finish()
    }

    fn digest_one<T>(&self, x: T) -> <Self::PortableHasher as PortableHasherDigest>::Output
    where
        T: PortableHash,
        Self::PortableHasher: PortableHasherDigest,
    {
        let mut hasher = self.build_hasher();
        x.portable_hash(&mut hasher);
        hasher.digest()
    }
}

pub struct DefaultBuildPortableHasher<H: PortableHasher + Default> {
    hasher: core::marker::PhantomData<H>,
}

impl<H: PortableHasher + Default> BuildPortableHasher for DefaultBuildPortableHasher<H> {
    type PortableHasher = H;

    fn build_hasher(&self) -> Self::PortableHasher {
        H::default()
    }
}
