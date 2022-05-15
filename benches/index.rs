use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordy::Index;
use std::time::Duration;

pub fn index_benchmark(c: &mut Criterion) {
    let dur = Duration::from_secs(40);

    let mut index = Index::with_capacity(
        117953,
        21499,
        4475,
        11540
    );

    let mut group = c.benchmark_group("index-search");
    group.sample_size(20).measurement_time(dur);
    group.bench_function(
        "index 20",
        |b| b.iter(|| index.parse_file(black_box("dict/index.noun")))
    );
    group.finish();
}

criterion_group!(benches, index_benchmark);
criterion_main!(benches);