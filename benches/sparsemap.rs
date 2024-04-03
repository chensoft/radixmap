use criterion::*;
use indexmap::IndexMap;
use sparseset::SparseSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

fn hashmap(c: &mut Criterion) {
    let mut obj = HashMap::new();

    c.bench_function("hashmap-insert", |b| b.iter(|| {
        for i in 0..256usize {
            black_box(obj.insert(i, i));
        }
    }));

    c.bench_function("hashmap-search", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn btreemap(c: &mut Criterion) {
    let mut obj = BTreeMap::new();

    c.bench_function("btreemap-insert", |b| b.iter(|| {
        for i in 0..256usize {
            black_box(obj.insert(i, i));
        }
    }));

    c.bench_function("btreemap-search", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn indexmap(c: &mut Criterion) {
    let mut obj = IndexMap::new();

    c.bench_function("indexmap-insert", |b| b.iter(|| {
        for i in 0..256usize {
            black_box(obj.insert(i, i));
        }
    }));

    c.bench_function("indexmap-search", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn sparse(c: &mut Criterion) {
    let mut obj = SparseSet::with_capacity(256);

    c.bench_function("sparse-insert", |b| b.iter(|| {
        for i in 0..256usize {
            black_box(obj.insert(i, i));
        }
    }));

    c.bench_function("sparse-search", |b| b.iter(|| {
        for i in 0..256usize {
            assert_eq!(obj.get(i), Some(&i));
        }
    }));
}

fn array(c: &mut Criterion) {
    let mut obj = [None; 256];

    c.bench_function("array-insert", |b| b.iter(|| {
        for i in 0..256 {
            obj[i] = Some(i);
        }
    }));

    c.bench_function("array-search", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj[i].as_ref(), Some(&i));
        }
    }));
}

criterion_group!(
    benches,
    hashmap,
    btreemap,
    indexmap,
    sparse,
    array,
);
criterion_main!(benches);