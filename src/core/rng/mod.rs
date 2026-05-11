//! Utilities for using random numbers in the game

use std::convert::Infallible;

use std::hash::Hash;

use rand::{TryRng, prelude::*, rngs::ChaCha20Rng};
use rand_seeder::Seeder;

/// A random number generator for the game
/// 
/// # Rng
/// 
/// [`ChaCha20Rng`] is roughly four times slower than
/// [`Xoshiro256PlusPlus`](rand::rngs::Xoshiro256PlusPlus), but it's
/// cryptographically secure, and allows for deterministic multithreading which
/// is well worth the tradeoff.
/// 
/// # Multithreading
/// 
/// Supports 2^64 streams of 2^68 words (word: u32) each. That's 1.18059162e21
/// bytes per stream before overflowing.
/// 
/// 
pub struct GameRng {
    inner: ChaCha20Rng
}

impl GameRng {
    /// Sets the parallel stream of the rng.
    pub fn set_stream(&mut self, stream: u64) {
        self.inner.set_stream(stream);
    }
}

impl TryRng for GameRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.inner.try_next_u32()
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.inner.try_next_u64()
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.inner.try_fill_bytes(dst)
    }
}

impl SeedableRng for GameRng {
    type Seed = GameSeed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self { inner: ChaCha20Rng::from_seed(seed.inner) }
    }
}











#[derive(Debug, Default, Clone)]
/// A seed for the game's rng
pub struct GameSeed {
    inner: [u8; 32]
}

impl GameSeed {
    /// Constructs a game seed from raw bytes
    pub fn from_raw(raw: [u8; 32]) -> Self {
        Self { inner: raw }
    }
}

impl AsRef<[u8]> for GameSeed {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl AsMut<[u8]> for GameSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

/// Uses [`rand_seeder`] to create a [`GameSeed`] from any type that
/// implements [`Hash`].
impl<T: Hash> From<T> for GameSeed {
    fn from(value: T) -> Self {
        Seeder::from(value).make_seed()
    }
}













#[cfg(test)]
mod test {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::*;

    #[test]
    fn reproducibility_raw() {
        let raw_seed: usize = 0;

        let seed = {
            let mut seed = GameSeed::default();

            seed.as_mut()[{32 - size_of::<usize>()}..].copy_from_slice(&raw_seed.to_le_bytes());

            seed
        };

        let mut rng = GameRng::from_seed(seed);

        let mut buf = [0u8; 4];

        rng.fill_bytes(&mut buf);

        assert_eq!(buf, [118, 184, 224, 173]);
    }

    #[test]
    fn reproducibility() {
        let seed = GameSeed::from("The bytes of this string literal are the seed content");

        let mut rng = GameRng::from_seed(seed);

        let mut buf = [0u8; 4];

        rng.fill_bytes(&mut buf);

        assert_eq!(buf, [82, 121, 20, 106]);
    }

    #[test]
    fn reproducibility_parallel() {
        let seed = GameSeed::from("The bytes of this string literal are the seed content");

        let buf = (0..4).into_par_iter()
                .map(|stream| {
                    let mut rng = GameRng::from_seed(seed.clone());
                    rng.set_stream(stream);
                    rng.random::<u8>()
                }).collect::<Vec<u8>>();

        assert_eq!(buf, [82, 224, 86, 58]);
    }
}