use criterion::*;
use indexmap::IndexMap;
use sparseset::SparseSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

fn hashmap(c: &mut Criterion) {
    let mut obj = HashMap::new();

    for i in 0..256usize {
        obj.insert(i, i);
    }

    c.bench_function("hashmap", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn btreemap(c: &mut Criterion) {
    let mut obj = BTreeMap::new();

    for i in 0..256usize {
        obj.insert(i, i);
    }

    c.bench_function("btreemap", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn indexmap(c: &mut Criterion) {
    let mut obj = IndexMap::new();

    for i in 0..256usize {
        obj.insert(i, i);
    }

    c.bench_function("indexmap", |b| b.iter(|| {
        for i in 0..256 {
            assert_eq!(obj.get(&i), Some(&i));
        }
    }));
}

fn sparse(c: &mut Criterion) {
    let mut obj = SparseSet::with_capacity(256);

    for i in 0..256usize {
        obj.insert(i, i);
    }

    c.bench_function("sparse", |b| b.iter(|| {
        for i in 0..256usize {
            assert_eq!(obj.get(i), Some(&i));
        }
    }));
}

fn array(c: &mut Criterion) {
    let mut obj = [None; 256];

    for i in 0..256 {
        obj[i] = Some(i);
    }

    c.bench_function("array", |b| b.iter(|| {
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