use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use rand::Rng;
use std::ops::{Add, Mul};

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
    type F = BabyBear;

    c.bench_function("add-latency", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
                // rng.gen::<F>()
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(rng.gen::<F>())
                }
                vec
            },
            |x| x.iter().fold(F::zero(), |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("add-throughput", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
                (
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..1000 {
                    (a, b, c, d, e, f, g, h, i, j) = (
                        a + b,
                        b + c,
                        c + d,
                        d + e,
                        e + f,
                        f + g,
                        g + h,
                        h + i,
                        i + j,
                        j + a,
                    );
                }
                (a, b, c, d, e, f, g, h, i, j)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("mul-latency", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
                // rng.gen::<F>()
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(rng.gen::<F>())
                }
                vec
            },
            |x| x.iter().fold(F::zero(), |x, y| x * *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("mul-throughput", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
                (
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..1000 {
                    (a, b, c, d, e, f, g, h, i, j) = (
                        a * b,
                        b * c,
                        c * d,
                        d * e,
                        e * f,
                        f * g,
                        g * h,
                        h * i,
                        i * j,
                        j * a,
                    );
                }
                (a, b, c, d, e, f, g, h, i, j)
            },
            BatchSize::SmallInput,
        )
    });

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
