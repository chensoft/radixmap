include!("data/plain_1.rs");
include!("data/plain_16.rs");
include!("data/plain_256.rs");
include!("data/plain_512.rs");

use criterion::*;
use radixmap::RadixMap;
use indexmap::IndexMap;
use std::collections::HashMap;

macro_rules! insert {
    ($test:expr, $kind:tt, $size:literal, $urls:expr) => {{
        let mut map = $kind::new();

        $test.bench_function(concat!("insert-plain-", stringify!($size), "-", stringify!($kind)), |b| {
            map.clear();

            b.iter(|| {
                for url in $urls {
                    let _ = black_box(map.insert(*url, true));
                }
            })
        });

        assert_eq!(map.len(), $size);
    }};
}

fn benchmark(c: &mut Criterion) {
    // 1
    insert!(c, RadixMap, 1, URLS_1);
    insert!(c, IndexMap, 1, URLS_1);
    insert!(c, HashMap, 1, URLS_1);

    // 16
    insert!(c, RadixMap, 16, URLS_16);
    insert!(c, IndexMap, 16, URLS_16);
    insert!(c, HashMap, 16, URLS_16);

    // 256
    insert!(c, RadixMap, 256, URLS_256);
    insert!(c, IndexMap, 256, URLS_256);
    insert!(c, HashMap, 256, URLS_256);

    // 512
    insert!(c, RadixMap, 512, URLS_512);
    insert!(c, IndexMap, 512, URLS_512);
    insert!(c, HashMap, 512, URLS_512);
}

criterion_group!(
    benches,
    benchmark,
);
criterion_main!(benches);