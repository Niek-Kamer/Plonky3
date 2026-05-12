//! Synthetic dep-graph microbench. Mirrors Poseidon2 width=8's per-round
//! dependency structure (S-box chain + cross-element mixing) without MDS or
//! round constants. Measures an OoO-aware upper bound on per-state speedup
//! for this dep pattern.
//!
//! Validation gate: dynamic vpmuludq per call must match real Poseidon (1204
//! for width=8, D=7); otherwise the dep pattern isn't representative.

use core::any::type_name;
use core::array;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_goldilocks::Goldilocks;
use rand::distr::{Distribution, StandardUniform};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

const ROUNDS_F_HALF: usize = 4;
const ROUNDS_P: usize = 22;
const WIDTH: usize = 8;

/// S-box for D=7. Matches `exp_const_u64::<7>`'s dep chain.
#[inline(always)]
fn sbox<T: PrimeCharacteristicRing + Copy>(x: T, rc: T) -> T {
    let x_rc = x + rc;
    let x2 = x_rc.square();
    let x4 = x2.square();
    let x3 = x2 * x_rc;
    x3 * x4
}

/// Lightweight cross-element mixing — sum of all elements added back to each.
/// Not MDS-correct, but preserves the round-to-round data dependency.
#[inline(always)]
fn mix<T: PrimeCharacteristicRing + Copy>(state: &mut [T; WIDTH]) {
    let mut sum = state[0];
    for s in &state[1..] {
        sum = sum + *s;
    }
    for s in state.iter_mut() {
        *s = *s + sum;
    }
}

#[inline(always)]
fn external_round<T: PrimeCharacteristicRing + Copy>(state: &mut [T; WIDTH], rc: &[T; WIDTH]) {
    for i in 0..WIDTH {
        state[i] = sbox(state[i], rc[i]);
    }
    mix(state);
}

#[inline(always)]
fn internal_round<T: PrimeCharacteristicRing + Copy>(state: &mut [T; WIDTH], rc: T) {
    state[0] = sbox(state[0], rc);
    mix(state);
}

/// Mirrors Poseidon2 width-8 structure (4 ext + 22 int + 4 ext rounds);
/// 344 mul-class ops total.
#[inline(never)]
fn synthetic_perm<T: PrimeCharacteristicRing + Copy>(
    state: &mut [T; WIDTH],
    rc_external_init: &[[T; WIDTH]; ROUNDS_F_HALF],
    rc_internal: &[T; ROUNDS_P],
    rc_external_term: &[[T; WIDTH]; ROUNDS_F_HALF],
) {
    for round in 0..ROUNDS_F_HALF {
        external_round(state, &rc_external_init[round]);
    }
    for round in 0..ROUNDS_P {
        internal_round(state, rc_internal[round]);
    }
    for round in 0..ROUNDS_F_HALF {
        external_round(state, &rc_external_term[round]);
    }
}

fn bench_synthetic<T>(c: &mut Criterion, name: &str)
where
    T: Copy + PrimeCharacteristicRing,
    StandardUniform: Distribution<T>,
{
    let mut rng = SmallRng::seed_from_u64(1);
    let rc_init: [[T; WIDTH]; ROUNDS_F_HALF] =
        array::from_fn(|_| array::from_fn(|_| rng.random()));
    let rc_internal: [T; ROUNDS_P] = array::from_fn(|_| rng.random());
    let rc_term: [[T; WIDTH]; ROUNDS_F_HALF] =
        array::from_fn(|_| array::from_fn(|_| rng.random()));

    c.bench_function(name, |b| {
        b.iter_batched(
            || array::from_fn::<T, WIDTH, _>(|_| rng.random()),
            |mut input| synthetic_perm(&mut input, &rc_init, &rc_internal, &rc_term),
            BatchSize::SmallInput,
        );
    });
}

fn bench_main(c: &mut Criterion) {
    type F = Goldilocks;
    type Packing = <F as Field>::Packing;
    let pname = type_name::<Packing>();

    bench_synthetic::<F>(c, "synthetic_depgraph/scalar");
    bench_synthetic::<Packing>(c, &format!("synthetic_depgraph/packed<{pname}>"));
}

criterion_group!(synthetic_depgraph, bench_main);
criterion_main!(synthetic_depgraph);
