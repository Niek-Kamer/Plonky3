//! Generic-path Poseidon2-Goldilocks bench. Bypasses the aarch64 cfg alias at
//! `goldilocks/src/poseidon2.rs:52` (which routes `Poseidon2Goldilocks` to the
//! fused kernel on aarch64) by constructing the generic `Poseidon2<…>` type
//! explicitly. aarch64-only — on other arches the alias already resolves to
//! generic and `poseidon.rs` covers it.

#[cfg(target_arch = "aarch64")]
mod bench_impl {
    use core::any::type_name;

    use criterion::{BatchSize, Criterion};
    use p3_field::{Field, PrimeCharacteristicRing};
    use p3_goldilocks::{
        GOLDILOCKS_POSEIDON2_RC_8_EXTERNAL_FINAL, GOLDILOCKS_POSEIDON2_RC_8_EXTERNAL_INITIAL,
        GOLDILOCKS_POSEIDON2_RC_8_INTERNAL, GOLDILOCKS_POSEIDON2_RC_12_EXTERNAL_FINAL,
        GOLDILOCKS_POSEIDON2_RC_12_EXTERNAL_INITIAL, GOLDILOCKS_POSEIDON2_RC_12_INTERNAL,
        GOLDILOCKS_POSEIDON2_RC_16_EXTERNAL_FINAL, GOLDILOCKS_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        GOLDILOCKS_POSEIDON2_RC_16_INTERNAL, Goldilocks, Poseidon2ExternalLayerGoldilocks,
        Poseidon2InternalLayerGoldilocks,
    };
    use p3_poseidon2::{ExternalLayerConstants, Poseidon2};
    use p3_symmetric::Permutation;
    use rand::distr::{Distribution, StandardUniform};
    use rand::rngs::SmallRng;
    use rand::{RngExt, SeedableRng};

    type F = Goldilocks;
    type Packing = <F as Field>::Packing;
    const D: u64 = 7;

    type GenericP2<const W: usize> = Poseidon2<
        Goldilocks,
        Poseidon2ExternalLayerGoldilocks<W>,
        Poseidon2InternalLayerGoldilocks,
        W,
        D,
    >;

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

    pub fn bench_poseidon2_generic(c: &mut Criterion) {
        let packed_name = type_name::<Packing>();

        let p8: GenericP2<8> = Poseidon2::new(
            ExternalLayerConstants::new(
                GOLDILOCKS_POSEIDON2_RC_8_EXTERNAL_INITIAL.to_vec(),
                GOLDILOCKS_POSEIDON2_RC_8_EXTERNAL_FINAL.to_vec(),
            ),
            GOLDILOCKS_POSEIDON2_RC_8_INTERNAL.to_vec(),
        );
        bench_permute::<_, F, 8>(c, "poseidon2_generic_scalar/width=8", &p8);
        bench_permute::<_, Packing, 8>(
            c,
            &format!("poseidon2_generic_packed<{packed_name}>/width=8"),
            &p8,
        );

        let p12: GenericP2<12> = Poseidon2::new(
            ExternalLayerConstants::new(
                GOLDILOCKS_POSEIDON2_RC_12_EXTERNAL_INITIAL.to_vec(),
                GOLDILOCKS_POSEIDON2_RC_12_EXTERNAL_FINAL.to_vec(),
            ),
            GOLDILOCKS_POSEIDON2_RC_12_INTERNAL.to_vec(),
        );
        bench_permute::<_, F, 12>(c, "poseidon2_generic_scalar/width=12", &p12);
        bench_permute::<_, Packing, 12>(
            c,
            &format!("poseidon2_generic_packed<{packed_name}>/width=12"),
            &p12,
        );

        let p16: GenericP2<16> = Poseidon2::new(
            ExternalLayerConstants::new(
                GOLDILOCKS_POSEIDON2_RC_16_EXTERNAL_INITIAL.to_vec(),
                GOLDILOCKS_POSEIDON2_RC_16_EXTERNAL_FINAL.to_vec(),
            ),
            GOLDILOCKS_POSEIDON2_RC_16_INTERNAL.to_vec(),
        );
        bench_permute::<_, F, 16>(c, "poseidon2_generic_scalar/width=16", &p16);
        bench_permute::<_, Packing, 16>(
            c,
            &format!("poseidon2_generic_packed<{packed_name}>/width=16"),
            &p16,
        );
    }
}

#[cfg(target_arch = "aarch64")]
criterion::criterion_group!(poseidon_generic, bench_impl::bench_poseidon2_generic);
#[cfg(target_arch = "aarch64")]
criterion::criterion_main!(poseidon_generic);

#[cfg(not(target_arch = "aarch64"))]
fn main() {}
