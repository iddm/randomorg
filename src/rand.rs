//! A rand_core::Rng implementation.
//! Available with the `rand` feature.
//!
//! # Usage
//!
//! ```rust,no_run
//! use rand_core::RngCore;
//! use randomorg::Random;
//!
//! let mut random = Random::new("API KEY HERE");
//! let mut key = [0u8; 16];
//! random.fill_bytes(&mut key);
//! let random_u64 = random.next_u64();
//! ```

impl From<crate::Error> for rand_core::Error {
    fn from(e: crate::Error) -> rand_core::Error {
        rand_core::Error::new(e)
    }
}

impl crate::Random {
    fn rand_next_i32(&self) -> Result<i32, crate::Error> {
        Ok(self
            .generate_integers(std::i32::MIN, std::i32::MAX, 1, true)?
            .result
            .random
            .data[0])
    }
}

// Optimize the implementation:
// 1. on `next_u**` calls collect as much integers as possible and store them in some
// structure. Return new integers from the vector of this structure and remove it from it.
// Once the vector is empty, request another portion.
// 2. Don't panic after a single attempt of `try_fill_bytes` in the `fill_bytes`.
// Try like 3 times and only then panic. The implementation must not be fallible if possible.
impl rand_core::RngCore for crate::Random {
    fn next_u32(&mut self) -> u32 {
        self.rand_next_i32()
            .expect("Could not request an integer (u32).") as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.rand_next_i32()
            .expect("Could not request an integer (u64).") as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Err(e) = self.try_fill_bytes(dest) {
            panic!("Error: {}", e);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        let limit = dest.len();
        if limit > std::u16::MAX as usize {
            return Err(crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Maximum length is 100, mandated by the random.org service.",
            ))
            .into());
        }
        let res = self.generate_blobs(limit as u16, 8)?.result.random.data;
        for i in 0..limit {
            dest[i] = res[i].parse().map_err(crate::Error::from)?;
        }
        Ok(())
    }
}

impl rand_core::CryptoRng for crate::Random {}

/// A fallible implementation, which uses the supplied implementation as a fallback,
/// when the random.org service is unavailable.
///
/// # Usage
///
/// ```rust,ignore
/// use rand_os::OsRng;
/// use rand_core::RngCore;
/// use randomorg::{rand::FallibleRandom, Random};
///
/// let random = Random::new("API KEY HERE");
/// let fallible = FallibleRandom { fallback: OsRng, random };
/// let mut key = [0u8; 16];
/// fallible.fill_bytes(&mut key);
/// let random_u64 = fallible.next_u64();
/// ```
pub struct FallibleRandom<T: rand_core::RngCore> {
    /// Fallback generator, used when `Random` does not work.
    pub fallback: T,
    /// A random.org generator.
    pub random: crate::Random,
}

impl<T: rand_core::RngCore> rand_core::RngCore for FallibleRandom<T> {
    fn next_u32(&mut self) -> u32 {
        self.random
            .rand_next_i32()
            .map(|n| n as u32)
            .unwrap_or_else(|_| self.fallback.next_u32())
    }

    fn next_u64(&mut self) -> u64 {
        self.random
            .rand_next_i32()
            .map(|n| n as u64)
            .unwrap_or_else(|_| self.fallback.next_u64())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Err(e) = self.try_fill_bytes(dest) {
            log::warn!(
                "Could not fill the bytes with FallibleRandom(randomorg): {}",
                e
            );
            self.fallback.fill_bytes(dest);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        if let Err(e) = self.random.try_fill_bytes(dest) {
            log::warn!(
                "Could not fill the bytes with FallibleRandom(randomorg): {}",
                e
            );
            self.fallback.try_fill_bytes(dest)
        } else {
            Ok(())
        }
    }
}

impl<T: rand_core::RngCore> rand_core::CryptoRng for FallibleRandom<T> {}
