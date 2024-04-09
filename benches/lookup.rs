include!("data/plain_1.rs");
include!("data/plain_16.rs");
include!("data/plain_256.rs");
include!("data/plain_512.rs");

use criterion::*;
use radixmap::RadixMap;
use indexmap::IndexMap;
use std::collections::HashMap;

macro_rules! lookup {
    ($test:expr, $kind:tt, $size:literal, $urls:expr, $path:expr) => {{
        let mut map = $kind::new();

        for url in $urls {
            let _ = black_box(map.insert(*url, true));
        }

        assert_eq!(map.len(), $size);

        $test.bench_function(concat!("lookup-plain-", stringify!($size), "-", stringify!($kind)), |b| b.iter(|| {
            black_box(map.get(&$path));
        }));
    }};
}

fn benchmark(c: &mut Criterion) {
    // 1
    lookup!(c, RadixMap, 1, URLS_1, PATH_1);
    lookup!(c, IndexMap, 1, URLS_1, PATH_1);
    lookup!(c, HashMap, 1, URLS_1, PATH_1);

    // 16
    lookup!(c, RadixMap, 16, URLS_16, PATH_16);
    lookup!(c, IndexMap, 16, URLS_16, PATH_16);
    lookup!(c, HashMap, 16, URLS_16, PATH_16);

    // 256
    lookup!(c, RadixMap, 256, URLS_256, PATH_256);
    lookup!(c, IndexMap, 256, URLS_256, PATH_256);
    lookup!(c, HashMap, 256, URLS_256, PATH_256);

    // 512
    lookup!(c, RadixMap, 512, URLS_512, PATH_512);
    lookup!(c, IndexMap, 512, URLS_512, PATH_512);
    lookup!(c, HashMap, 512, URLS_512, PATH_512);
}

criterion_group!(
    benches,
    benchmark,
);
criterion_main!(benches);