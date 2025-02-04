# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.4](https://github.com/wyatt-herkamp/digestible/compare/digestible-v0.2.3...digestible-v0.2.4) - 2025-02-04

### Other

- Fix Hash And HashSet implementation

## 0.2.3 (Unreleased)
- Implement Digestible for [std::collections::{HashMap, HashSet}](https://doc.rust-lang.org/std/collections/index.html)
- Implement Digestible for [alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque}](https://doc.rust-lang.org/alloc/collections/index.html)
- Implements Digestible  [serde_json::Value](https://docs.rs/serde_json/1.0.135/serde_json/enum.Value.html) and [serde_json::Number](https://docs.rs/serde_json/1.0.135/serde_json/struct.Number.html)  in `serde_json` feature
- Implements Digestible for `uuid::Uuid` in `uuid` feature
- Implements Digestible for chrono types in `chrono` feature
- More testing
## 0.2.2 (2023-10-13)
- Fixed Unresolved path for `core::any`

## 0.1.0 (2023-08-26)
Initial release. The MVP of Digestible.


[0.1.0]: https://github.com/wyatt-herkamp/digestible/releases/tag/0.1.0
