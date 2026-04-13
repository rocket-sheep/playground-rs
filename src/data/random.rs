use rand::RngCore;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::slice;

const SEED: u64 = 1234567890;

pub fn make_deterministic_rng() -> impl RngCore {
    return StdRng::seed_from_u64(SEED);
}

pub fn random_vec<T>(rng: &mut impl RngCore, length: usize) -> Vec<T>
where
    T: Copy + Default,
{
    let mut v = Vec::<T>::with_capacity(length);
    v.resize(length, T::default());
    unsafe {
        let p = &mut v[0] as *mut T as *mut u8;
        let s = slice::from_raw_parts_mut(p, length * size_of::<T>());
        rng.fill_bytes(s);
    }
    return v;
}
