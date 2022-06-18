use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use forest::tree::Tree;

fn criterion_benchmark(c: &mut Criterion) {
    let size: usize = 1_000_000;

    c.bench_with_input(BenchmarkId::new("append_child", size), &size, |b, &s| {
        b.iter(|| {
            let mut tree = Tree::with_capacity(s);
            (0..s).for_each(|i| {
                tree.append_child(i);
            });
        });
    });

    c.bench_with_input(BenchmarkId::new("append_sibling", size), &size, |b, &s| {
        b.iter(|| {
            let mut tree = Tree::with_capacity(s);
            (0..s).for_each(|i| {
                tree.append_sibling(i);
            });
        });
    });

    let mut tree = Tree::with_capacity(size);
    (0..size).for_each(|i| {
        tree.append_child(i);
    });

    c.bench_with_input(BenchmarkId::new("iterate_child", size), &tree, |b, t| {
        b.iter(|| {
            let mut sum = 0;
            for v in t.iter() {
                sum += *v;
            }
            sum
        });
    });

    let mut tree = Tree::with_capacity(size * 2);
    (0..size).for_each(|i| {
        tree.append_child(i);
        tree.append_sibling(i);
    });

    c.bench_with_input(BenchmarkId::new("iterate_mixed", size), &tree, |b, t| {
        b.iter(|| {
            let mut sum = 0;
            for v in t.iter() {
                sum += *v;
            }
            sum
        });
    });

    c.bench_with_input(BenchmarkId::new("iterate_mut", size), &size, move |b, s| {
        b.iter(|| {
            let size = *s;
            let mut tree = Tree::with_capacity(size);
            (0..size).for_each(|i| {
                tree.append_child(i);
            });

            for v in tree.iter_mut() {
                *v += 1;
            }
        });
    });

    c.bench_with_input(
        BenchmarkId::new("iterate_into_iter", size),
        &size,
        move |b, s| {
            b.iter(|| {
                let size = *s;
                let mut tree = Tree::with_capacity(size);
                (0..size).for_each(|i| {
                    tree.append_child(i);
                });

                let mut sum = 0;
                for mut v in tree.into_iterator() {
                    v += 1;
                    sum += v;
                }

                sum
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
