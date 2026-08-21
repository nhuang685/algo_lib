use std::time;

/// modified from https://github.com/EbTech/rust-algorithms/blob/master/src/rng.rs
/// A xoshiro256++ random number generator.
///
/// This is a simplified version of the `SmallRng` implementation from the
/// excellent `rand` crate, keeping only essential features.
///
/// The xoshiro256++ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has excellent statistical properties.
///
/// * Source: [Docs.rs](https://docs.rs/rand/0.8.4/src/rand/rngs/xoshiro256plusplus.rs.html)
/// * Theory: [Xorshift - Wikipedia](https://en.wikipedia.org/wiki/Xorshift)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro256PlusPlus {
    s: [u64; 4],
}

impl Xoshiro256PlusPlus {
    /// Construct a new RNG from a 64-bit seed.
    pub fn with_seed(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut seed = <[u64; 4]>::default();
        for chunk in &mut seed {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *chunk = z;
        }
        Self { s: seed }
    }
    pub fn new() -> Self {
        Self::with_seed(
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64,
        )
    }

    /// Generate a random `u32`.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Generate a random `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result_plusplus = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);

        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = self.s[3].rotate_left(45);

        result_plusplus
    }
}
impl Default for Xoshiro256PlusPlus {
    fn default() -> Self {
        Self::new()
    }
}
pub type SmallRng = Xoshiro256PlusPlus;
