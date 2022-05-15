use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordy::search;
use std::fs::File;
use std::time::Duration;

pub fn search_benchmark(c: &mut Criterion) {
    let mut file = File::open("dict/index.noun").expect("lol");
    let key = "cellophane".to_string();

    let dur = Duration::from_secs(25);

    let mut group = c.benchmark_group("binary-search");
    group.sample_size(20).measurement_time(dur);
    group.bench_function(
        "search 20",
        |b| b.iter(|| search(black_box(&mut file), black_box(key.clone())))
    );
    group.finish();
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);