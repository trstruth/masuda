use criterion::{criterion_group, criterion_main, Criterion};
use masuda::generators::LinearCongruential;

fn criterion_benchmark(c: &mut Criterion) {
    let mut lcrng = LinearCongruential::new(0);

    c.bench_function("LCRNG Method 1", |b| b.iter(|| for _ in 0..1000 {
        lcrng.method_1();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
