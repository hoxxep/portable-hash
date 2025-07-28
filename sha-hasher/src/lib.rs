use portable_hash::{DefaultBuildPortableHasher, PortableHasher, PortableHasherDigest};
use sha2::Digest;

/// A SHA-256 [`PortableHasher`] implementation.
///
/// TODO: key the hasher for DoS resistance?
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

impl PortableHasherDigest for Sha256Hasher {
    type Output = [u8; 32];

    fn digest(&self) -> Self::Output {
        let result = self.hasher.clone().finalize();
        result.into()
    }
}


#[cfg(test)]
mod tests {
    use portable_hash::{BuildPortableHasher, PortableHash, PortableHasher, PortableHasherDigest};
    use super::*;

    #[derive(PortableHash)]
    struct TestObject {
        a: u32,
        b: String,
        c: Vec<u64>,
        d: (u8, u16, u32),
    }

    #[test]
    fn struct_derive() {
        let object = TestObject {
            a: 42,
            b: "Hello, World!".to_string(),
            c: vec![1, 2, 3, 4, 5],
            d: (8, 16, 32),
        };

        let mut hasher = Sha256Hasher::default();
        object.portable_hash(&mut hasher);
        assert_eq!(hasher.finish(), 4525177376694807513);
        assert_eq!(hasher.digest(), [217, 43, 135, 67, 217, 168, 204, 62, 204, 237, 191, 246, 214, 170, 36, 71, 160, 219, 216, 185, 14, 198, 213, 168, 27, 249, 66, 76, 35, 41, 181, 146]);
    }

    #[derive(PortableHash)]
    enum TestEnum {
        UnNamedA(u32),
        UnNamedB(u32),
        NamedA {
            a: u8,
            b: u16,
        },
        NamedB {
            a: u32,
            b: u16,
        },
        UnitA,
        UnitB,
    }

    #[test]
    fn enum_derive() {
        let hasher = Sha256BuildHasher::default();

        let unnamed_a = TestEnum::UnNamedA(42);
        assert_eq!(hasher.hash_one(&unnamed_a), 648569055011963888);

        let unnamed_b = TestEnum::UnNamedB(42);
        assert_eq!(hasher.hash_one(&unnamed_b), 7833144868699154772);

        let named_a = TestEnum::NamedA {
            a: 8,
            b: 16,
        };
        assert_eq!(hasher.hash_one(&named_a), 17990340046588545446);

        let named_b = TestEnum::NamedB {
            a: 8,
            b: 16,
        };
        assert_eq!(hasher.hash_one(&named_b), 11634397432774414619);

        let unit_a = TestEnum::UnitA;
        assert_eq!(hasher.hash_one(&unit_a), 5126029364455812581);

        let unit_b = TestEnum::UnitB;
        assert_eq!(hasher.hash_one(&unit_b), 940095539697581031);
    }
}
