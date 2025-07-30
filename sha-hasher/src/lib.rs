use portable_hash::{DefaultBuildPortableHasher, ExtendedPortableHasher, PortableHasher};
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

impl ExtendedPortableHasher for Sha256Hasher {
    type Output = [u8; 32];

    fn digest(&self) -> Self::Output {
        let result = self.hasher.clone().finalize();
        result.into()
    }
}


#[cfg(test)]
mod tests {
    use portable_hash::{BuildPortableHasher, PortableHash, PortableHasher, ExtendedPortableHasher};
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
        assert_eq!(hasher.finish(), 15114829174459170361);
        assert_eq!(hasher.digest(), [57, 126, 89, 104, 13, 169, 194, 209, 154, 89, 137, 46, 240, 254, 41, 255, 110, 158, 34, 166, 227, 201, 55, 51, 118, 163, 31, 35, 208, 245, 158, 175]);
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
