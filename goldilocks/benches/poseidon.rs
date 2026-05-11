use core::any::type_name;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_goldilocks::poseidon1::{default_goldilocks_poseidon1_8, default_goldilocks_poseidon1_12};
use p3_goldilocks::{
    Goldilocks, default_goldilocks_poseidon2_8, default_goldilocks_poseidon2_12,
    default_goldilocks_poseidon2_16,
};
use p3_symmetric::Permutation;
use rand::distr::{Distribution, StandardUniform};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

type F = Goldilocks;
type Packing = <F as Field>::Packing;

fn bench_permute<Perm, T, const W: usize>(c: &mut Criterion, name: &str, perm: &Perm)
where
    T: Copy + PrimeCharacteristicRing,
    Perm: Permutation<[T; W]>,
    StandardUniform: Distribution<T>,
{
    let mut rng = SmallRng::seed_from_u64(1);
    c.bench_function(name, |b| {
        b.iter_batched(
            || core::array::from_fn::<T, W, _>(|_| rng.random()),
            |input| perm.permute(input),
            BatchSize::SmallInput,
        );
    });
}

fn bench_poseidon2(c: &mut Criterion) {
    let packed_name = type_name::<Packing>();

    let p8 = default_goldilocks_poseidon2_8();
    bench_permute::<_, F, 8>(c, "poseidon2_scalar/width=8", &p8);
    bench_permute::<_, Packing, 8>(c, &format!("poseidon2_packed<{packed_name}>/width=8"), &p8);

    let p12 = default_goldilocks_poseidon2_12();
    bench_permute::<_, F, 12>(c, "poseidon2_scalar/width=12", &p12);
    bench_permute::<_, Packing, 12>(
        c,
        &format!("poseidon2_packed<{packed_name}>/width=12"),
        &p12,
    );

    let p16 = default_goldilocks_poseidon2_16();
    bench_permute::<_, F, 16>(c, "poseidon2_scalar/width=16", &p16);
    bench_permute::<_, Packing, 16>(
        c,
        &format!("poseidon2_packed<{packed_name}>/width=16"),
        &p16,
    );
}

fn bench_poseidon1(c: &mut Criterion) {
    let packed_name = type_name::<Packing>();

    let p8 = default_goldilocks_poseidon1_8();
    bench_permute::<_, F, 8>(c, "poseidon1_scalar/width=8", &p8);
    bench_permute::<_, Packing, 8>(c, &format!("poseidon1_packed<{packed_name}>/width=8"), &p8);

    let p12 = default_goldilocks_poseidon1_12();
    bench_permute::<_, F, 12>(c, "poseidon1_scalar/width=12", &p12);
    bench_permute::<_, Packing, 12>(
        c,
        &format!("poseidon1_packed<{packed_name}>/width=12"),
        &p12,
    );
}

criterion_group!(goldilocks_poseidon, bench_poseidon2, bench_poseidon1);
criterion_main!(goldilocks_poseidon);
