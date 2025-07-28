use portable_hash::{PortableHasher, PortableHasherDigest};
use sha2::Digest;

/// A SHA-256 `PortableHasher` implementation.
#[derive(Default, Clone)]
pub struct Sha256Hasher {
    hasher: sha2::Sha256,
}

impl PortableHasher for Sha256Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    fn finish(&self) -> u64 {
        let result = self.hasher.clone().finalize();
        u64::from_le_bytes(result[0..8].try_into().unwrap())
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
    use portable_hash::{PortableHash, PortableHasher, PortableHasherDigest};
    use super::*;

    #[derive(PortableHash)]
    struct TestObject {
        a: u32,
        b: String,
        c: Vec<u64>,
        d: (u8, u16, u32),
    }

    #[test]
    fn it_works() {
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
}
