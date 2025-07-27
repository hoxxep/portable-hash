mod impls;

pub trait PortableHash {
    /// Computes the hash of the object using the specified hasher.
    fn hash<H: PortableHasher>(&self, state: &mut H);

    /// Feed a slice of this type into the given [`PortableHasher`].
    fn hash_slice<H: PortableHasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        // TODO: adding a length prefix here differs from the standard library.
        state.write_len_prefix(data.len());
        for item in data.iter() {
            item.hash(state);
        }
    }
}

pub trait PortableHasher {
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
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_i64(i as i64);
    }

    // TODO: review the addition of write_bytes.
    // TODO: link up the impl for write_bytes
    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_len_prefix(bytes.len());
        self.write(bytes);
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write_len_prefix(s.as_bytes().len());
        self.write(s.as_bytes());
    }

    #[inline]
    fn write_len_prefix(&mut self, len: usize) {
        self.write_usize(len);
    }

    /// Finalizes the hash computation and returns the hash value.
    fn finish(&self) -> u64;
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
        x.hash(&mut hasher);
        hasher.finish()
    }

    fn digest_one<T>(&self, x: T) -> <Self::PortableHasher as PortableHasherDigest>::Output
    where
        T: PortableHash,
        Self::PortableHasher: PortableHasherDigest,
    {
        let mut hasher = self.build_hasher();
        x.hash(&mut hasher);
        hasher.digest()
    }
}

pub struct DefaultBuildPortableHasher<H: PortableHasher + Default> {
    hasher: std::marker::PhantomData<H>,
}

impl<H: PortableHasher + Default> BuildPortableHasher for DefaultBuildPortableHasher<H> {
    type PortableHasher = H;

    fn build_hasher(&self) -> Self::PortableHasher {
        H::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::PortableHash;

    struct TestObject {
        a: u32,
        b: String,
        c: Vec<u64>,
        d: (u8, u16, u32),
    }

    impl PortableHash for TestObject {
        fn hash<H: super::PortableHasher>(&self, state: &mut H) {
            self.a.hash(state);
            self.b.hash(state);
            self.c.hash(state);
            self.d.hash(state);
        }
    }

    #[test]
    fn it_works() {
        let _object = TestObject {
            a: 42,
            b: "Hello, World!".to_string(),
            c: vec![1, 2, 3, 4, 5],
            d: (8, 16, 32),
        };
    }
}
