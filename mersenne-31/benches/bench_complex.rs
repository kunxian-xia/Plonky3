use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::Rng;
use p3_field::AbstractField;
use p3_mersenne_31::{Mersenne31, Mersenne31Complex};

fn bench_m31_complex(c: &mut Criterion) {
    type F = Mersenne31Complex<Mersenne31>;
    c.bench_function("complex-add-latency", |b|

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
    );

    c.bench_function("complex-add-throughput", |b| {
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

    c.bench_function("complex-mul-latency", |b|

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
    );

    c.bench_function("complex-mul-throughput", |b| {
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

criterion_group!(mersenne31_arithmetics, bench_m31_complex);
criterion_main!(mersenne31_arithmetics);
