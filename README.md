# Xoz

Xoz is an experimental read-only XML library with the following features:

* small representation of the XML in memory.

* fast iteration over named tags.

* fast text search

## Implementation notes

In order to store compact XML, Xoz uses [succinct data
structures](https://en.wikipedia.org/wiki/Succinct_data_structure).

We make use of:

* [vers](https://crates.io/crates/vers-vecs) - for the succinct tree
  implementation using the Balanced Parentheses technique. This is backed by
  its `RsVec`, a bit vector that supports rank and select. We also uses its
  wavelet matrix implementation for connecting tags to trees.

* [fm-index](https://crates.io/crates/fm-index). This allows fast search over
  compressed text. This is based on vers for both rank/select as well as its
  wavelet matrix.

* [sucds](https://crates.io/crates/sucds) - for a sparse sarray that supports
  rank and select. We use this as an alternative way to connect tags to trees.
