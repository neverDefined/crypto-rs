use digest::Digest;

/// Hash arbitrary bytes using any Digest implementation.
/// Returns raw bytes as a Vec<u8>.
pub fn simple_hash<D>(data: &[u8]) -> Vec<u8>
where
    D: Digest,
{
    let mut hasher = D::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()
}

/// A hash function with fixed-size output `N`.
pub trait HashFunction<const N: usize> {
    /// Hash arbitrary data into a fixed-size array of bytes.
    fn hash(data: &[u8]) -> [u8; N];
}

/// Helper that calls a specific HashFunction implementation.
pub fn hash_with<H, const N: usize>(data: &[u8]) -> [u8; N]
where
    H: HashFunction<N>,
{
    H::hash(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use sha2::Sha256;

    #[test]
    fn simple_hash_sha256_hello() {
        let h = simple_hash::<Sha256>(b"hello");
        assert_eq!(
            hex::encode(h),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}

