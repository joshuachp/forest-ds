use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use forest::tree::Tree;

fn criterion_benchmark(c: &mut Criterion) {
    let size: usize = 100;

    c.bench_with_input(BenchmarkId::new("insertion", size), &size, |b, &s| {
        b.iter(|| {
            let mut tree = Tree::new();
            (0..s).for_each(|i| tree.append_child(i));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
