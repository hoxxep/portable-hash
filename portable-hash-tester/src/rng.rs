const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

/// A low-quality, extremely fast RNG function for generating stable fixtures.
///
/// # Example usage
/// ```
/// use portable_hash_tester::rng;
///
/// let mut seed = 0;
/// assert_eq!(rng(&mut seed), 2448534243492233863);
/// assert_eq!(rng(&mut seed), 13722372260377396495);
/// assert_eq!(rng(&mut seed), 9676170207902303383);
/// ```
pub fn rng(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(RAPID_SECRET[0]);
    mix(*seed, RAPID_SECRET[1])
}

fn mix(x: u64, y: u64) -> u64 {
    // u64 x u64 -> u128 product is prohibitively expensive on 32-bit.
    // Decompose into 32-bit parts.
    let lx = x as u32;
    let ly = y as u32;
    let hx = (x >> 32) as u32;
    let hy = (y >> 32) as u32;

    // u32 x u32 -> u64 the low bits of one with the high bits of the other.
    let afull = (lx as u64) * (hy as u64);
    let bfull = (hx as u64) * (ly as u64);

    // Combine, swapping low/high of one of them so the upper bits of the
    // product of one combine with the lower bits of the other.
    afull ^ bfull.rotate_right(32)
}
