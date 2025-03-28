# Xoz

Xoz is read-only XML library with the following features:

* small representation of the XML in memory.

* fast iteration over named tags.

* fast text search

## Implementation notes

In order to store compact XML, Xoz uses [succinct data
structures](https://en.wikipedia.org/wiki/Succinct_data_structure). I wrote a
[gentle introduction to succinct data
structures](https://blog.startifact.com/posts/succinct/) on my blog.

For instance, an 86 meg XML file takes up only about 102 megabytes of memory.

We make use of:

* [vers](https://crates.io/crates/vers-vecs) - for the succinct tree
  implementation using the Balanced Parentheses technique. This is backed by
  its `RsVec`, a bit vector that supports rank and select. We also uses its
  wavelet matrix implementation for connecting tags to trees.

* [fm-index](https://crates.io/crates/fm-index). This allows fast search over
  compressed text. This is based on vers for both rank/select as well as its
  wavelet matrix. (at the time of writing this isn't integrated yet)

* [sucds](https://crates.io/crates/sucds) - right now still used for arrays
  that use the minimal amounts of bits, but we aim to build this on top of
  vers.