use alloc::format;

use alloc::vec::Vec;
use criterion::{black_box, BatchSize, Criterion};
use p3_field::Field;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

pub fn benchmark_square<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    c.bench_function(&format!("{} square", name), |b| {
        b.iter(|| black_box(black_box(x).square()))
    });
}

pub fn benchmark_inv<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    c.bench_function(&format!("{} inv", name), |b| {
        b.iter(|| black_box(black_box(x)).inverse())
    });
}

pub fn benchmark_add<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    let y = rng.gen::<F>();
    c.bench_function(&format!("{} add", name), |b| {
        b.iter(|| black_box(black_box(x) + black_box(y)))
    });
}

pub fn benchmark_mul<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    let y = rng.gen::<F>();
    c.bench_function(&format!("{} mul", name), |b| {
        b.iter(|| black_box(black_box(x) * black_box(y)))
    });
}

pub fn benchmark_add_latency<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    c.bench_function("add-latency", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
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
}

pub fn benchmark_add_throughput<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
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
}

pub fn benchmark_mul_latency<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    c.bench_function("mul-latency", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::thread_rng();
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
}

pub fn benchmark_mul_throughput<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
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
}
