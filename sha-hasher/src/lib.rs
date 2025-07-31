use portable_hash::{DefaultBuildPortableHasher, ExtendedPortableHasher, PortableHasher};
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

impl ExtendedPortableHasher for Sha256Hasher {
    type Output = [u8; 32];

    fn digest(&self) -> Self::Output {
        let result = self.hasher.clone().finalize();
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use portable_hash::{PortableHasher};
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
}
