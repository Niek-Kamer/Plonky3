//! Sanity bench: per-call throughput of `<Goldilocks as Field>::Packing` vs
//! scalar `Goldilocks` for add/sub/mul. Per-call packed ≈ scalar confirms
//! N-lane SIMD parallelism is intact.

use criterion::{Criterion, criterion_group, criterion_main};
use p3_field::Field;
use p3_field_testing::bench_func::{
    benchmark_add_latency, benchmark_add_throughput, benchmark_sub_latency,
    benchmark_sub_throughput,
};
use p3_field_testing::{benchmark_mul_latency, benchmark_mul_throughput};
use p3_goldilocks::Goldilocks;

type F = Goldilocks;
type Packing = <F as Field>::Packing;

const REPS: usize = 200;
const L_REPS: usize = 10 * REPS;

fn bench_scalar(c: &mut Criterion) {
    let name = "Goldilocks_scalar";
    benchmark_add_throughput::<F, REPS>(c, name);
    benchmark_add_latency::<F, L_REPS>(c, name);
    benchmark_sub_throughput::<F, REPS>(c, name);
    benchmark_sub_latency::<F, L_REPS>(c, name);
    benchmark_mul_throughput::<F, REPS>(c, name);
    benchmark_mul_latency::<F, L_REPS>(c, name);
}

fn bench_packed(c: &mut Criterion) {
    let name = core::any::type_name::<Packing>();
    benchmark_add_throughput::<Packing, REPS>(c, name);
    benchmark_add_latency::<Packing, L_REPS>(c, name);
    benchmark_sub_throughput::<Packing, REPS>(c, name);
    benchmark_sub_latency::<Packing, L_REPS>(c, name);
    benchmark_mul_throughput::<Packing, REPS>(c, name);
    benchmark_mul_latency::<Packing, L_REPS>(c, name);
}

criterion_group!(packing_sanity, bench_scalar, bench_packed);
criterion_main!(packing_sanity);
