use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut searcher = hyperexplorer::search::FileSearcher::new();
    c.bench_function("Indexing", |b| b.iter(|| searcher.index("/", true)));
    c.bench_function("Searching", |b| b.iter(|| searcher.search("hyperexplorer", 10)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);