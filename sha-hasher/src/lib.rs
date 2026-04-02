use portable_hash::{DefaultBuildPortableHasher, PortableHasher, PortableHasherOutput};
use sha2::Digest;

/// A SHA-256 [`PortableHasher`] implementation.
#[derive(Default, Clone)]
pub struct Sha256Hasher {
    hasher: sha2::Sha256,
}

/// A SHA-256 [`portable_hash::BuildPortableHasher`] that instantiates a default [`Sha256Hasher`].
pub type Sha256BuildHasher = DefaultBuildPortableHasher<Sha256Hasher>;

impl PortableHasher for Sha256Hasher {
    fn finish(&self) -> u64 {
        let result = self.hasher.clone().finalize();
        u64::from_le_bytes(result[0..8].try_into().unwrap())
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}

impl PortableHasherOutput<[u8; 32]> for Sha256Hasher {
    fn finalize(&self) -> [u8; 32] {
        let result = self.hasher.clone().finalize();
        result.into()
    }
}

impl PortableHasherOutput<u128> for Sha256Hasher {
    fn finalize(&self) -> u128 {
        let result = self.hasher.clone().finalize();
        u128::from_le_bytes(result[0..16].try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use portable_hash::{BuildPortableHasher, PortableHash, PortableHasher};
    use super::*;

    /// Test against the portable hasher fixture framework.
    #[test]
    fn test_portable_hasher() {
        portable_hash_tester::test_portable_hasher(Sha256BuildHasher::default(), "tests/fixtures.csv");
    }

    /// Confirm the sha.update() method is bytewise compatible with subsequent calls.
    #[test]
    fn test_sha_understanding() {
        let mut hasher = Sha256Hasher::default();
        hasher.write_u8(1);
        hasher.write_u8(0);
        let hash1 = hasher.finish();

        let mut hasher = Sha256Hasher::default();
        hasher.write_u16(1);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    fn hash_one<T: PortableHash>(value: &T) -> u64 {
        Sha256BuildHasher::default().hash_one(value)
    }

    // ---- Index-mode enum ----

    /// Index-mode enum: discriminants are based on declaration order (0, 1, 2, ...).
    /// All discriminants use write_u64.
    #[derive(PortableHash)]
    #[portable_hash(discriminant = "index")]
    enum IndexEnum {
        UnNamedA(u32),
        UnNamedB(u32),
        NamedA { a: u8, b: u16 },
        NamedB { a: u32, b: u16 },
        UnitA,
        UnitB,
    }

    #[test]
    fn test_index_enum_variants_differ() {
        // Same payload, different discriminant => different hash.
        assert_ne!(hash_one(&IndexEnum::UnNamedA(42)), hash_one(&IndexEnum::UnNamedB(42)));
        assert_ne!(hash_one(&IndexEnum::NamedA { a: 1, b: 2 }), hash_one(&IndexEnum::NamedB { a: 1, b: 2 }));
        assert_ne!(hash_one(&IndexEnum::UnitA), hash_one(&IndexEnum::UnitB));
    }

    #[test]
    fn test_index_enum_uses_write_u64() {
        // Index mode uses write_u64 for all discriminants.
        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(0); // discriminant for UnNamedA
        42u32.portable_hash(&mut hasher);
        assert_eq!(hash_one(&IndexEnum::UnNamedA(42)), hasher.finish(), "index 0 via write_u64");

        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(4); // discriminant for UnitA (5th variant, index 4)
        assert_eq!(hash_one(&IndexEnum::UnitA), hasher.finish(), "index 4 via write_u64");
    }

    // ---- Name-mode enum (default) ----

    /// Name-mode enum: discriminants are FNV-1a hashes of variant names.
    /// Reordering variants should not change hash output.
    #[derive(PortableHash)]
    enum NameEnum {
        UnNamedA(u32),
        UnNamedB(u32),
        NamedA { a: u8, b: u16 },
        NamedB { a: u32, b: u16 },
        UnitA,
        UnitB,
    }

    /// Same variants as NameEnum but in a different declaration order.
    #[derive(PortableHash)]
    enum NameEnumReordered {
        UnitB,
        NamedB { a: u32, b: u16 },
        UnNamedA(u32),
        UnitA,
        NamedA { a: u8, b: u16 },
        UnNamedB(u32),
    }

    #[test]
    fn test_name_enum_reorder_safe() {
        assert_eq!(hash_one(&NameEnum::UnNamedA(42)), hash_one(&NameEnumReordered::UnNamedA(42)));
        assert_eq!(hash_one(&NameEnum::UnNamedB(42)), hash_one(&NameEnumReordered::UnNamedB(42)));
        assert_eq!(hash_one(&NameEnum::NamedA { a: 1, b: 2 }), hash_one(&NameEnumReordered::NamedA { a: 1, b: 2 }));
        assert_eq!(hash_one(&NameEnum::NamedB { a: 1, b: 2 }), hash_one(&NameEnumReordered::NamedB { a: 1, b: 2 }));
        assert_eq!(hash_one(&NameEnum::UnitA), hash_one(&NameEnumReordered::UnitA));
        assert_eq!(hash_one(&NameEnum::UnitB), hash_one(&NameEnumReordered::UnitB));
    }

    #[test]
    fn test_name_enum_variants_differ() {
        assert_ne!(hash_one(&NameEnum::UnNamedA(42)), hash_one(&NameEnum::UnNamedB(42)));
        assert_ne!(hash_one(&NameEnum::UnitA), hash_one(&NameEnum::UnitB));
    }

    #[test]
    fn test_name_enum_differs_from_index_enum() {
        assert_ne!(hash_one(&NameEnum::UnNamedA(42)), hash_one(&IndexEnum::UnNamedA(42)));
    }

    // ---- Manual discriminant override ----

    #[derive(PortableHash)]
    enum ManualDiscriminantEnum {
        #[portable_hash(discriminant = 100)]
        A(u32),
        B(u32),
    }

    #[test]
    fn test_manual_discriminant() {
        let hash_a = hash_one(&ManualDiscriminantEnum::A(42));
        let hash_b = hash_one(&ManualDiscriminantEnum::B(42));
        assert_ne!(hash_a, hash_b);

        // Verify A uses the manual discriminant (100u64 written via write_u64).
        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(100);
        42u32.portable_hash(&mut hasher);
        assert_eq!(hash_a, hasher.finish(), "manual discriminant = 100");
    }

    // ---- Rename attribute ----

    #[derive(PortableHash)]
    enum RenamedEnum {
        #[portable_hash(rename = "OldName")]
        NewName(u32),
        Other(u32),
    }

    #[derive(PortableHash)]
    enum OriginalEnum {
        OldName(u32),
        Other(u32),
    }

    #[test]
    fn test_rename_preserves_hash() {
        assert_eq!(
            hash_one(&RenamedEnum::NewName(42)),
            hash_one(&OriginalEnum::OldName(42)),
            "rename should produce same hash as original name",
        );
        assert_eq!(
            hash_one(&RenamedEnum::Other(99)),
            hash_one(&OriginalEnum::Other(99)),
            "non-renamed variants should match",
        );
    }

    // ---- Index-mode with manual override ----

    #[derive(PortableHash)]
    #[portable_hash(discriminant = "index")]
    enum IndexWithOverride {
        A(u32),
        #[portable_hash(discriminant = 999)]
        B(u32),
        C(u32),
    }

    #[test]
    fn test_index_with_manual_override() {
        let hash_a = hash_one(&IndexWithOverride::A(42));
        let hash_b = hash_one(&IndexWithOverride::B(42));
        let hash_c = hash_one(&IndexWithOverride::C(42));
        assert_ne!(hash_a, hash_b);
        assert_ne!(hash_b, hash_c);
        assert_ne!(hash_a, hash_c);

        // A uses index 0 via write_u64.
        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(0);
        42u32.portable_hash(&mut hasher);
        assert_eq!(hash_a, hasher.finish(), "A uses index 0");

        // B uses manual override 999 via write_u64.
        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(999);
        42u32.portable_hash(&mut hasher);
        assert_eq!(hash_b, hasher.finish(), "B uses manual discriminant 999");
    }

    // ---- Explicit Rust discriminant values ----

    #[derive(PortableHash)]
    #[portable_hash(discriminant = "index")]
    enum ExplicitDiscriminants {
        A = 5,
        B,      // auto-incremented to 6
        C = 20,
        D,      // auto-incremented to 21
    }

    #[test]
    fn test_explicit_rust_discriminants() {
        // Verify explicit discriminant values are used.
        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(5);
        assert_eq!(hash_one(&ExplicitDiscriminants::A), hasher.finish(), "A = 5");

        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(6);
        assert_eq!(hash_one(&ExplicitDiscriminants::B), hasher.finish(), "B = 6 (auto)");

        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(20);
        assert_eq!(hash_one(&ExplicitDiscriminants::C), hasher.finish(), "C = 20");

        let mut hasher = Sha256Hasher::default();
        hasher.write_u64(21);
        assert_eq!(hash_one(&ExplicitDiscriminants::D), hasher.finish(), "D = 21 (auto)");
    }

    // ---- discriminant_width attribute ----

    #[derive(PortableHash)]
    #[portable_hash(discriminant = "index", discriminant_width = "u8")]
    enum WidthU8Enum {
        A,
        B,
        C,
    }

    #[test]
    fn test_width_u8() {
        let mut hasher = Sha256Hasher::default();
        hasher.write_u8(0);
        assert_eq!(hash_one(&WidthU8Enum::A), hasher.finish(), "width u8, A=0");

        let mut hasher = Sha256Hasher::default();
        hasher.write_u8(2);
        assert_eq!(hash_one(&WidthU8Enum::C), hasher.finish(), "width u8, C=2");
    }

}
