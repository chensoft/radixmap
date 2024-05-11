include!("data/plain_16.rs");
include!("data/plain_64.rs");
include!("data/plain_512.rs");
include!("data/plain_1024.rs");

use criterion::*;
use radixmap::RadixMap;

macro_rules! lookup {
    ($test:expr, $size:literal, $urls:expr, $path:expr) => {{
        let mut map = RadixMap::new();

        for url in $urls {
            let _ = black_box(map.insert(*url, true));
        }

        assert_eq!(map.len(), $size);

        $test.bench_function(concat!("lookup-plain-", stringify!($size)), |b| b.iter(|| {
            black_box(map.get(&$path));
        }));
    }};
}

fn benchmark(c: &mut Criterion) {
    lookup!(c, 16, PLAIN_URLS_16, PLAIN_PATH_16);
    lookup!(c, 64, PLAIN_URLS_64, PLAIN_PATH_64);
    lookup!(c, 512, PLAIN_URLS_512, PLAIN_PATH_512);
    lookup!(c, 1024, PLAIN_URLS_1024, PLAIN_PATH_1024);
}

criterion_group!(
    benches,
    benchmark,
);
criterion_main!(benches);