use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use p3_field::AbstractField;
use p3_field_testing::{
    benchmark_add, benchmark_add_latency, benchmark_add_throughput, benchmark_mul,
    benchmark_mul_latency, benchmark_mul_throughput,
};
use p3_goldilocks::Goldilocks;
use rand::Rng;

type F = Goldilocks;

fn root_7(c: &mut Criterion) {
    c.bench_function("7th_root", |b| {
        b.iter_batched(
            rand::random::<F>,
            |x| x.exp_u64(10540996611094048183),
            BatchSize::SmallInput,
        )
    });

    benchmark_add::<Goldilocks>(c, "");
    benchmark_add_latency::<Goldilocks>(c, "");
    benchmark_add_throughput::<Goldilocks>(c, "");
    benchmark_mul::<Goldilocks>(c, "");
    benchmark_mul_latency::<Goldilocks>(c, "");
    benchmark_mul_throughput::<Goldilocks>(c, "");
}

criterion_group!(goldilocks_arithmetic, root_7);
criterion_main!(goldilocks_arithmetic);
