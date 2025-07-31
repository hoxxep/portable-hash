#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

#![warn(missing_docs)]
#![deny(unused_must_use)]
#![deny(unsafe_code)]

mod hash_impls;
mod ord_impls;

/// A derive macro for [`PortableHash`].
pub use portable_hash_macros::PortableHash;

/// A trait for types that can be hashed in a portable way, inspired by [`std::hash::Hash`].
///
/// This trait is similar to the standard library's `std::hash::Hash`, but it is designed to be
/// portable across different platforms and compiler versions. `PortableHash` types can be hashed
/// by any hasher that implements the [`PortableHasher`] trait.
///
/// Any types that implement this trait must guarantee that the hashing logic is stable across:
/// - All platforms.
/// - All rust compiler versions.
/// - All minor versions of your crate.
///
/// # Example Derive Usage
/// ```
/// use portable_hash::PortableHash;
///
/// #[derive(PortableHash)]
/// struct MyStruct {
///     a: u32,
///     b: String,
///     c: Vec<u64>,
/// }
/// ```
///
/// # Example Manual Implementation
/// ```
/// use portable_hash::{PortableHash, PortableHasher};
///
/// struct MyStruct {
///     a: u32,
///     b: String,
///     c: Vec<u64>,
/// }
///
/// impl PortableHash for MyStruct {
///     fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
///         state.write_u32(self.a);
///         state.write_str(&self.b);
///         state.write_len_prefix(self.c.len());
///         for item in &self.c {
///             item.portable_hash(state);
///         }
///     }
/// }
/// ```
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

/// A trait marker that determines if a type's `Ord` implementation is guaranteed to be portable
/// across platforms and compiler versions.
///
/// This tells `PortableHash` that the type can be used as the key in an ordered collection, such
/// as a `BTreeMap` or `BTreeSet`, and the hashing order will be consistent across all platforms.
///
/// TODO(stabilisation): further document the requirements for types that implement this trait.
pub trait PortableOrd: Ord {
    /// Denotes whether unstable sorting can be used for this type. Set to true if and
    /// only if `a == b` implies `a` and `b` are fully indistinguishable.
    const CAN_USE_UNSTABLE_SORT: bool;

    /// Do you really though?
    const I_KNOW_WHAT_I_AM_DOING: ();
}

/// A trait for hashers that can hash any [`PortableHash`] type, inspired by [`std::hash::Hasher`].
///
/// Your crate must provide the following guarantees when implementing PortableHasher:
/// - **Do not use** `Hasher::*` methods to implement `PortableHasher::*` methods. The
///   `std::hash` traits may change behaviour between compiler versions, please do not mix and
///   match them.
/// - The hash output must be stable across all platforms, compiler versions, and minor versions
///   of your crate.
/// - Integer types are hashed consistently across all platforms, explicitly choosing little-endian
///   or big-endian encoding, and hashing `usize` and `isize` in a platform-independent way.
/// - Use `default-features = false` when adding portable-hash as a dependency, so that end users
///   can opt to disable the `std` feature.
///
/// End users likely want to use a [`BuildPortableHasher`] implementation to create a hasher,
/// instead of directly using `PortableHasher`. This enables caching the hasher's seed once and
/// instantiating multiple hashers of the same seed.
///
/// TODO(stabilisation): should this trait's methods be renamed to avoid conflicting with the std
///   `Hasher` trait methods?
///
/// TODO(stabilisation): should we recommend implementing `PortableHasher` on an entirely separate
///   type from the one that implements `Hasher` to avoid accidentally mixing the two? Does rust
///   prevent confusing the two trait methods sufficiently?
pub trait PortableHasher {
    /// Finalize the hash computation and return the hash value.
    fn finish(&self) -> u64;

    /// Write a byte slice to the hasher.
    fn write(&mut self, bytes: &[u8]);

    /// Write a single byte to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 16-bit unsigned integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 32-bit unsigned integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 64-bit unsigned integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 128-bit unsigned integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 64-bit unsigned size to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_usize(&mut self, i: usize) {
        // Portability: always write usize as a full-sized u64 to remain platform independent.
        self.write_u64(i as u64);
    }

    /// Write a single byte to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 16-bit signed integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 32-bit signed integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 64-bit signed integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a 128-bit signed integer to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way.
    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write_short(i.to_le_bytes());
    }

    /// Write a signed size to the hasher.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the number is hashed in an
    /// endian-agnostic way, and that the
    #[inline]
    fn write_isize(&mut self, i: isize) {
        // Portability: always write isize as a full-sized i64 to remain platform independent.
        self.write_i64(i as i64);
    }

    /// Write a length prefix to the hasher.
    ///
    /// The length of a collection should be written to the hasher using this method before any of
    /// the collection's items are written to the hasher. This is done to prevent various types of
    /// collision attacks on collections.
    ///
    /// The `PortableHasher` implementation is responsible for ensuring the length is written in a
    /// portable way, such as using a fixed-width integer type and handling endianness consistently.
    #[inline]
    fn write_len_prefix(&mut self, len: usize) {
        self.write_usize(len);
    }

    /// Write a string slice to the hasher.
    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write_len_prefix(s.as_bytes().len());
        self.write(s.as_bytes());
    }

    /// `write_bytes` may be called by types that can safely serialize `[T]` into `[u8]` safely.
    ///
    /// Currently, this is primarily called for unpadded numeric slices, such as `&[u8]`, `&[u16]`.
    /// To provide some DoS resistance, the hasher should consume the length of the slice somehow.
    /// For hashers that do not care for DoS resistance, or for algoirthms that incorporate the
    /// length of the slice on every call to `self.write`, `write_bytes` can be overridden to remove
    /// the `write_len_prefix` call.
    ///
    /// **Warning:** very, very few types are portable across platforms as a byte slice. While this
    /// method is a big performance win on `std::hash::Hasher`, it's rarely portable! Padding, type
    /// sizes, and endianness can all vary between platforms. It is not recommended to transmute
    /// other types into a byte slice for this method. This method should only be used on true
    /// `&[u8]` byte slices, such as serialized file or image data.
    ///
    /// Origin for this idea: https://github.com/rust-lang/rust/pull/134134#issuecomment-2535503144
    ///
    /// TODO(stabilisation): review the addition of write_bytes.
    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_len_prefix(bytes.len());
        self.write(bytes);
    }

    /// Write a fixed-size array of bytes to the hasher.
    ///
    /// This method allows for optimizations when writing small fixed-size arrays to the hasher. The
    /// numeric `write_*` methods call this by default.
    ///
    /// For example, some hashers such as foldhash have internal buffers that are used to store
    /// integer types. `write_short` could be used to write other small non-integer types into the
    /// buffer directly, gated behind an `if bytes.len() < LEN` statement that's evaluated at
    /// compile time.
    ///
    /// Origin for this idea: [`rustc-stable-hash`](https://github.com/rust-lang/rustc-stable-hash/blob/6780c967c1b9b0f5b49c9cc24d1b97ed584ec3ae/src/stable_hasher.rs#L51)
    ///
    /// TODO(stabilisation): review the addition of write_short.
    #[inline]
    fn write_short<const LEN: usize>(&mut self, bytes: [u8; LEN]) {
        self.write(bytes.as_slice())
    }
}

/// An alternative to [`PortableHasher::finish`] with a hasher-specific output type.
///
/// TODO(stabilisation): merge this trait into [`PortableHasher`], it makes little sense to be separate.
///
/// TODO(stabilisation): could we enable hashers to have multiple output types, or extensible output types?
pub trait ExtendedPortableHasher: PortableHasher {
    /// The type of output produced by the hasher.
    type Output;

    /// Finalizes the hash computation and returns the hasher-specific output value.
    ///
    /// TODO(stabilisation): review the naming and addition of this method. Should this be finish(), and move the other to finish_u64()?
    fn digest(&self) -> Self::Output;
}

/// A trait for building multiple [`PortableHasher`]s that use the same seed.
///
/// Similar to [`std::hash::BuildHasher`], but for portable hashers. This lets us cache random
/// seeds for hashers and quickly instantiate multiple hashers with the same seed.
pub trait BuildPortableHasher {
    /// The type of hasher that this builder creates.
    type PortableHasher: PortableHasher;

    /// Creates a new instance of the hasher.
    fn build_hasher(&self) -> Self::PortableHasher;

    /// Hash an object, returning a u64 hash value.
    fn hash_one<T>(&self, x: T) -> u64
    where
        T: PortableHash,
    {
        let mut hasher = self.build_hasher();
        x.portable_hash(&mut hasher);
        hasher.finish()
    }

    /// Hash an object, returning a hasher-specific output value.
    fn digest_one<T>(&self, x: T) -> <Self::PortableHasher as ExtendedPortableHasher>::Output
    where
        T: PortableHash,
        Self::PortableHasher: ExtendedPortableHasher,
    {
        let mut hasher = self.build_hasher();
        x.portable_hash(&mut hasher);
        hasher.digest()
    }
}

/// A default implementation of [`BuildPortableHasher`] that instantiates the [`PortableHasher`]
/// using the [`Default`] trait.
pub struct DefaultBuildPortableHasher<H: PortableHasher + Default> {
    hasher: core::marker::PhantomData<H>,
}

impl<H: PortableHasher + Default> BuildPortableHasher for DefaultBuildPortableHasher<H> {
    type PortableHasher = H;

    fn build_hasher(&self) -> Self::PortableHasher {
        H::default()
    }
}

impl<H: PortableHasher + Default> Default for DefaultBuildPortableHasher<H> {
    fn default() -> Self {
        Self {
            hasher: core::marker::PhantomData,
        }
    }
}
