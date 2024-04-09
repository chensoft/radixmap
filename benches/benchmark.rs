const APIS: &[&str] = &[
    "/api/v1"
];

const LONGEST: &str = "/api/v1";

use criterion::*;
use radixmap::RadixMap;
use indexmap::IndexMap;
use std::collections::HashMap;

fn radixmap(c: &mut Criterion) {
    let mut map = RadixMap::new();

    c.bench_function("radixmap-insert", |b| b.iter(|| {
        for url in APIS {
            let _ = black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("radixmap-search", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

fn indexmap(c: &mut Criterion) {
    let mut map = IndexMap::new();

    c.bench_function("indexmap-insert", |b| b.iter(|| {
        for url in APIS {
            black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("indexmap-search", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

fn hashmap(c: &mut Criterion) {
    let mut map = HashMap::new();

    c.bench_function("hashmap-insert", |b| b.iter(|| {
        for url in APIS {
            black_box(map.insert(*url, true));
        }
    }));

    c.bench_function("hashmap-search", |b| b.iter(|| {
        black_box(map.get(LONGEST));
    }));
}

criterion_group!(
    benches,
    radixmap,
    indexmap,
    hashmap,
);
criterion_main!(benches);