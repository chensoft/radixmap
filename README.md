radixmap
==========================

This crate is a rust-based radix tree implementation. Radix tree, also known as Trie, is a
space-optimized tree data structure for efficient information retrieval. Its key advantages
are space optimization, fast prefix-based searches, and efficient memory usage. Radix trees
are widely used, especially in HTTP routers.

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

- MacBook Air, Apple M2 24G, Sonoma 14.4, Rust 1.78.0

| Name              |              Time               |
|:------------------|:-------------------------------:|
| lookup-plain-16   | [29.149 ns 29.179 ns 29.215 ns] |
| lookup-plain-64   | [34.797 ns 34.898 ns 35.017 ns] |
| lookup-plain-512  | [51.162 ns 51.479 ns 51.917 ns] |
| lookup-plain-1024 | [57.123 ns 57.782 ns 58.615 ns] |
| insert-plain-16   | [1.3337 µs 1.3370 µs 1.3405 µs] |
| insert-plain-64   | [7.8995 µs 7.9275 µs 7.9570 µs] |
| insert-plain-512  | [103.13 µs 103.30 µs 103.52 µs] |
| insert-plain-1024 | [255.19 µs 255.69 µs 256.26 µs] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04, Rust 1.78.0

| Name              |              Time               |
|:------------------|:-------------------------------:|
| lookup-plain-16   | [42.448 ns 42.469 ns 42.489 ns] |
| lookup-plain-64   | [47.602 ns 47.614 ns 47.625 ns] |
| lookup-plain-512  | [62.200 ns 62.213 ns 62.226 ns] |
| lookup-plain-1024 | [67.797 ns 67.805 ns 67.814 ns] |
| insert-plain-16   | [2.3793 µs 2.3807 µs 2.3826 µs] |
| insert-plain-64   | [13.704 µs 13.709 µs 13.714 µs] |
| insert-plain-512  | [204.39 µs 204.97 µs 205.73 µs] |
| insert-plain-1024 | [482.81 µs 484.23 µs 486.10 µs] |