# Xoz

Xoz is an experimental read-only XML library with the following features:

* small representation of the XML in memory.

* fast text search

* fast iteration over named tags.

## Implementation notes

In order to store compact XML, Xoz uses [succinct data
structures](https://en.wikipedia.org/wiki/Succinct_data_structure).

Unfortunately not all the facilities required by the library
are provided by the same dependencies, so we make use of:

* [vers](https://crates.io/crates/vers-vecs) - for the succinct tree
  implementation using the Balanced Parentheses technique. This is backed by
  its `RsVec`, a bit vector that supports rank and select. We also uses its
  wavelet matrix implementation for connecting tags to trees.

* [sucds](https://crates.io/crates/sucds) - for a sparse sarray that supports
  rank and select. We use this as an alternative way to connect tags to trees.

* [fm-index](https://crates.io/crates/fm-index). This allows fast search over
  compressed text. This uses the [fid
  crate](https://crates.io/crates/fid/0.1.7) which implements another bit
  vector which supports rank and select.

It is hoped that eventually we can consolidate some of this.