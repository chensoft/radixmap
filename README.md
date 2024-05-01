radixmap
==========================

## ⚠️ Caution: Not Ready for Production! ⚠️

Rust-based Radix Tree for fast prefix lookup, supporting named param, glob and regex

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
- Named param, glob, regex support
- Pre-order, post-order, level-order iterations
- Comprehensive unit tests for correctness

## Example

```rust
use bytes::Bytes;
use radixmap::{RadixMap, RadixResult};

fn main() -> RadixResult<()> {
    let mut map = RadixMap::new();
    map.insert("/api", "api")?;
    map.insert("/api/v1", "v1")?;
    map.insert("/api/v1/user", "user1")?;
    map.insert("/api/v2", "v2")?;
    map.insert("/api/v2/user", "user2")?;

    assert_eq!(map.get(b"/api/v1/user"), Some(&"user1"));
    assert_eq!(map.get(b"/api/v2/user"), Some(&"user2"));

    let mut iter = map.iter(); // pre-order by default

    assert_eq!(iter.next(), Some((&Bytes::from("/api"), &"api")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v1"), &"v1")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v1/user"), &"user1")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2"), &"v2")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user"), &"user2")));
    assert_eq!(iter.next(), None);

    Ok(())
}
```

## Benchmark

- MacBook Air, Apple M2 24G, Sonoma 14.4, Rust 1.77.1

| Name              |              Time               |
|:------------------|:-------------------------------:|

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04, Rust 1.77.1

| Name              |              Time               |
|:------------------|:-------------------------------:|

## Documentation

The documentation is [available here](https://docs.rs/radixmap).

## License

This software is released under the [MIT License](https://github.com/chensoft/radixmap?tab=MIT-1-ov-file).