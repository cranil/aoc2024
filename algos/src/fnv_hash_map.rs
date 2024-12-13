use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
};

const FNV_1A_OFFSET_BASIS: u64 = 0xcb_f2_9c_e4_84_22_23_25;
const FNV_1A_PRIME: u64 = 0x01_00_00_00_01_b3;

pub struct Fnv1aHasher {
    hash: u64,
}

impl Default for Fnv1aHasher {
    fn default() -> Self {
        Self {
            hash: FNV_1A_OFFSET_BASIS,
        }
    }
}

impl Hasher for Fnv1aHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let Fnv1aHasher { mut hash } = *self;

        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_1A_PRIME);
        }

        self.hash = hash;
    }
}

pub type Fnv1aBuildHasher = BuildHasherDefault<Fnv1aHasher>;

pub type Fnv1aHashMap<K, V> = HashMap<K, V, Fnv1aBuildHasher>;
pub type Fnv1aHashSet<T> = HashSet<T, Fnv1aBuildHasher>;
