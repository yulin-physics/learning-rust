mod sort;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn insertion_sorter_benchmark(c: &mut Criterion) {
    c.bench_function("sort 20", |b| {
        b.iter(|| sort::insertion_sorter(black_box(&mut vec![20])))
    });
}

criterion_group!(benches, insertion_sorter_benchmark);
criterion_main!(benches);
