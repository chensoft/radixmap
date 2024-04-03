const APIS: &[&str] = &[
    "/api/v1"
];

const LONGEST: &str = "/api/v1";

use criterion::*;
use radixmap::RadixMap;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::collections::BTreeMap;

fn radixmap(c: &mut Criterion) {
    let mut map = RadixMap::new();

    c.bench_function("radixmap-insert", |b| b.iter(|| {
        map.clear();

        for url in APIS {
            let _ = black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("radixmap-longest", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

fn hashmap(c: &mut Criterion) {
    let mut map = HashMap::new();

    c.bench_function("hashmap-insert", |b| b.iter(|| {
        map.clear();

        for url in APIS {
            black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("hashmap-longest", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

fn btreemap(c: &mut Criterion) {
    let mut map = BTreeMap::new();

    c.bench_function("btreemap-insert", |b| b.iter(|| {
        map.clear();

        for url in APIS {
            black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("btreemap-longest", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

fn indexmap(c: &mut Criterion) {
    let mut map = IndexMap::new();

    c.bench_function("indexmap-insert", |b| b.iter(|| {
        map.clear();

        for url in APIS {
            black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("indexmap-longest", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

criterion_group!(
    benches,
    radixmap,
    hashmap,
    btreemap,
    indexmap,
);
criterion_main!(benches);