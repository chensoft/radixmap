## [0.2.3] - 2024-05-21

### Added

- Access data using raw path

## [0.2.2] - 2024-05-19

### Changed

- Return empty captured content

### Fixed

- The pattern '/*' should match the path '/'

## [0.2.1] - 2024-05-04

### Changed

- Remove recursion calls on node's lookup
- Do not return captured data if node not found
- Do not return tuple on lookup

## [0.2.0] - 2024-05-01

### Added

- Return glob result
- Impl Index & IndexMut for map

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

- entry support, use raw node
- intoiterator post order
- remove Pack, add regular special to Node?
- special use Vec instead of IndexMap
- named params benchmark
- plain only mode
- item sep customizable
- re-balance on removed node
- /auth & /{:as\d+}, cant match the second one
- r"/{ip:[0-9.]+}/{fields:[a-zA-Z+]*}" cant match /1.1.1.1, only /1.1.1.1/
- regex support multiple named captures
- regex support syntax like {1,5}