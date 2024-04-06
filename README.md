radixmap
==========================

Rust-based Radix Tree for fast prefix lookup, supporting params, regex and glob

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][license-badge]][license-url]
[![Documentation][document-badge]][document-url]
[![Build Status][macos-badge]][macos-url]
[![Build Status][linux-badge]][linux-url]
[![Build Status][windows-badge]][windows-url]

[crates-badge]: https://img.shields.io/crates/v/radixmap.svg
[crates-url]: https://crates.io/crates/radixmap
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/chensoft/radixmap?tab=MIT-1-ov-file
[document-badge]: https://docs.rs/radixmap/badge.svg
[document-url]: https://docs.rs/radixmap
[macos-badge]: https://github.com/chensoft/radixmap/actions/workflows/macos.yml/badge.svg
[macos-url]: https://github.com/chensoft/radixmap/actions/workflows/macos.yml
[linux-badge]: https://github.com/chensoft/radixmap/actions/workflows/linux.yml/badge.svg
[linux-url]: https://github.com/chensoft/radixmap/actions/workflows/linux.yml
[windows-badge]: https://github.com/chensoft/radixmap/actions/workflows/windows.yml/badge.svg
[windows-url]: https://github.com/chensoft/radixmap/actions/workflows/windows.yml

## Features

- Fast prefix-based lookup
- RadixMap and RadixSet support
- Standard collection-compatible interfaces
- Named params, regex and glob support
- Pre-order, post-order, level-order iterations
- Comprehensive unit tests for correctness

## Example

```rust
use radixmap::{RadixMap, RadixResult};

fn main() -> RadixResult<()> {
    // creation
    let mut map = RadixMap::new();
    map.insert("/api", "api")?;
    map.insert("/api/v1", "v1")?;
    map.insert("/api/v1/user1", "user1")?;
    map.insert("/api/v2", "v2")?;
    map.insert("/api/v2/user2", "user2")?;

    // searching
    assert_eq!(map.get("/api/v1/user1"), Some(&"user1"));
    assert_eq!(map.get("/api/v2/user2"), Some(&"user2"));

    // iteration
    let mut iter = map.iter(); // pre-order by default

    assert_eq!(iter.next(), Some(("/api", &"api")));
    assert_eq!(iter.next(), Some(("/api/v1", &"v1")));
    assert_eq!(iter.next(), Some(("/api/v1/user1", &"user1")));
    assert_eq!(iter.next(), Some(("/api/v2", &"v2")));
    assert_eq!(iter.next(), Some(("/api/v2/user2", &"user2")));
    assert_eq!(iter.next(), None);

    Ok(())
}
```

## Benchmark

- MacBook Air, Apple M2 24G, Sonoma 14.4.1, Rust 1.77.1

| Name              |              Time               |
|:------------------|:-------------------------------:|

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04, Rust 1.77.1

| Name              |              Time               |
|:------------------|:-------------------------------:|

## Documentation

The documentation is [available here](https://docs.rs/radixmap).

## License

This software is released under the [MIT License](https://github.com/chensoft/radixmap?tab=MIT-1-ov-file).