include!("data/plain_16.rs");
include!("data/plain_64.rs");
include!("data/plain_512.rs");
include!("data/plain_1024.rs");

use criterion::*;
use radixmap::RadixMap;

macro_rules! insert {
    ($test:expr, $size:literal, $urls:expr) => {{
        let mut map = RadixMap::new();

        $test.bench_function(concat!("insert-plain-", stringify!($size)), |b| {
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
    insert!(c, 16, PLAIN_URLS_16);
    insert!(c, 64, PLAIN_URLS_64);
    insert!(c, 512, PLAIN_URLS_512);
    insert!(c, 1024, PLAIN_URLS_1024);
}

criterion_group!(
    benches,
    benchmark,
);
criterion_main!(benches);