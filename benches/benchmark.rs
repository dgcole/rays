#[macro_use]
extern crate criterion;

#[path="../src/rays.rs"]
pub mod rays;

use criterion::Criterion;
use criterion::ParameterizedBenchmark;
use criterion::Benchmark;


fn criterion_benchmark(c: &mut Criterion) {
    let bm0 = Benchmark::new("160x120x1", |b| b.iter(|| rays::raytrace(160, 120, 1, "test"))).sample_size(50);
    let bm1 = Benchmark::new("160x120x2", |b| b.iter(|| rays::raytrace(160, 120, 2, "test"))).sample_size(50);

    let bm2 = Benchmark::new("150x75x1", |b| b.iter(|| rays::raytrace(150, 75, 1, "test"))).sample_size(50);
    let bm3 = Benchmark::new("150x150x1", |b| b.iter(|| rays::raytrace(150, 150, 1, "test"))).sample_size(50);
    let bm4 = Benchmark::new("300x150x1", |b| b.iter(|| rays::raytrace(300, 150, 1, "test"))).sample_size(50);

    c.bench("Sample Variation", bm0);
    c.bench("Sample Variation", bm1);

    c.bench("Resolution Variation", bm2);
    c.bench("Resolution Variation", bm3);
    c.bench("Resolution Variation", bm4);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);