# forest-ds

<a href="https://crates.io/crates/forest-ds">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/forest-ds">
</a>
<a href="https://docs.rs/forest-ds">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/forest-ds">
</a>

Implementation an arena based Tree structure.

Implemented features:

- Appending children and siblings
- Inserting children and siblings on any node
- Iteration on the structure (`Iter`, `IterMut` and `IntoIter`)
- `serde` feature for serialization

Missing features:

- Parallel iteration with `rayon`
