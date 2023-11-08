use std::ops::{Add, Mul};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;

type F = BabyBear;

fn root_7(c: &mut Criterion) {
    c.bench_function("7th_root", |b| {
        b.iter_batched(
            rand::random::<F>,
            // m = 1725656503
            // 7*m = 1 (mod p-1)
            // p = 2^31 - 2^27 + 1
            |x| x.exp_u64(1725656503),
            BatchSize::SmallInput,
        )
    });
}

fn add_mul(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter_batched(
            || (rand::random::<F>(), rand::random::<F>()),
            |(x, y)| x.add(y),
                BatchSize::SmallInput,
        )
    });

    c.bench_function("mul", |b| {
        b.iter_batched(
            || (rand::random::<F>(), rand::random::<F>()),
            |(x, y)| x.mul(y),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(baby_bear_arithmetic, root_7, add_mul);
criterion_main!(baby_bear_arithmetic);
