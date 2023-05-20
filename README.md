# forest-ds

[![Crates.io](https://img.shields.io/crates/v/forest-ds)](https://crates.io/crates/forest-ds)
[![docs.rs](https://img.shields.io/docsrs/forest-ds)](https://docs.rs/forest-ds/)
[![CI](https://github.com/github/joshuachp/forest-ds/workflows/test.yml/badge.svg)](https://github.com/joshuachp/forest-ds/blob/main/.github/workflows/test.yml)
[![codecov](https://codecov.io/gh/joshuachp/forest-ds/branch/main/graph/badge.svg?token=KYDH1J83U9)](https://codecov.io/gh/joshuachp/forest-ds)

Implementation an arena based Tree structure.

Implemented features:

- Appending children and siblings
- Inserting children and siblings on any node
- Iteration on the structure (`Iter`, `IterMut` and `IntoIter`)
- `serde` feature for serialization

Missing features:

- Parallel iteration with `rayon`
