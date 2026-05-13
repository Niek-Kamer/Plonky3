//! Sanity-check harness: Poseidon2 width-8 permute_mut on PackedGoldilocksAVX2.
//!
//! Companion to `poseidon1_perf_w8` for cross-Poseidon vpmuludq comparison.

#[cfg(not(target_feature = "avx2"))]
fn main() {
    eprintln!(
        "poseidon2_perf_w8: requires RUSTFLAGS=\"-C target-feature=+avx2\" \
         (or target-cpu=native on an AVX2-capable host)"
    );
    std::process::exit(1);
}

#[cfg(target_feature = "avx2")]
fn main() {
    use std::env;
    use std::hint::black_box;
    use std::time::Instant;

    use p3_goldilocks::{PackedGoldilocksAVX2, default_goldilocks_poseidon2_8};
    use p3_symmetric::Permutation;
    use rand::SeedableRng;
    use rand::distr::{Distribution, StandardUniform};
    use rand::rngs::SmallRng;

    let n_iter: usize = env::args()
        .nth(1)
        .map(|s| s.parse().expect("usage: poseidon2_perf_w8 [n_iterations]"))
        .unwrap_or(1_000_000);

    let perm = default_goldilocks_poseidon2_8();

    let mut rng = SmallRng::seed_from_u64(1);
    let mut state: [PackedGoldilocksAVX2; 8] =
        core::array::from_fn(|_| StandardUniform.sample(&mut rng));

    let t0 = Instant::now();
    for _ in 0..n_iter {
        perm.permute_mut(black_box(&mut state));
    }
    let elapsed = t0.elapsed();
    let _ = black_box(state);

    let ns_per = elapsed.as_nanos() as f64 / n_iter as f64;
    eprintln!("iterations: {n_iter}");
    eprintln!("elapsed:    {:?}", elapsed);
    eprintln!("ns/iter:    {:.2}", ns_per);
}
