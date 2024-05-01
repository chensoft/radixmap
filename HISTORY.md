## [0.2.0] - 2024-05-0x

### Added

- Return glob result

### Changed

- Use Bytes to store path
- Use &[u8] instead of &str

## [0.1.0] - 2024-04-10

### Added

- RadixMap and RadixSet
- HashMap-like interfaces
- Multiple traversal orders
- Named param, glob and regex

## Todo

- invalid utf8 test with regex matches: &[0xffu8, 0xfe, 0x65];
- index & indexmut
- intoiterator
- 
- benchmark
- plain only mode
- item sep customizable
- re-balance on removed node
- invalid utf8 test: &[0xffu8, 0xfe, 0x65];
- entry support