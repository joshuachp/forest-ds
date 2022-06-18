# forest-ds

Implementing an arena based Tree structure.

Implemented features:

- Appending children and siblings
- Inserting children and siblings on any node
- Iteration on the structure (`Iter`, `IterMut` and `IntoIter`)
- `serde` feature for serialization

Missing features:

- Removal of a node
- Parallel iteration with `rayon`
