# coordgen-rs
A thin wrapper around [coordgen](https://github.com/schrodinger/coordgenlibs).

This exposes a pair of functions that interface with `libcoordgen` to generate 2D coordinates
for a molecule given its connectivity. The purpose of this crate is to simply generate the
coordinates, it does not perform any drawing functionality on its own.

This crate is not endorsed or supported by the original authors of `coordgen`.

## Molecular Graphs
Molecular graphs can be represented a few different ways. This crate adopts an opinionated
definition where molecular graphs are undirected simple graphs with atomic numbers as node
labels and bond multiplicities as edge labels.

## Implementation
This implementation defines a simple wrapper function with a C-like interface that is
essentially a modified version of the example in the `coordgen` repository. This wrapper does
not allow for any configuration.

## Versioning
| crate version | `coordgen` version |
| --- | --- |
| 0.1.0 | 2.0.3 |

## Example
```rust
// this corresponds to an equivalent to the example in
// https://github.com/schrodinger/coordgenlibs/blob/master/example_dir/example.cpp

// the molecule has a carbon (idx 0) and a nitrogen (idx 1)
let atoms = vec![6u8, 7];
// it has one single bond connecting the two atoms with 
let bonds = vec![[0u16, 1, 1]];

let coords = coordgen::gen_coords(&atoms, &bonds).unwrap();
assert_eq!(coords, vec![(-50.0, 0.0), (0.0, 0.0)])
```
