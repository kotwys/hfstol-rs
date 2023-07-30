# HFST Optimized Lookup for Rust.

This crate is a port of University of Helsinki's [`hfst-optimized-lookup`] to
Rust as a library.

**⚠️ The crate functionality is not on-par with the original implementation
yet!**  Only weighted transducers are supported at the moment.

The crate can be built for WebAssembly.  Also, the crate can be compiled under
`#![no_std]` when setting `default-features = false`, but actual performance in
such an environment has not been tested.

## Examples

To read a transducer, use `hfstol::read_transducer()` function with a `&[u8]`
buffer of binary contents.  Created transducers will have a `hfstol::Transducer`
trait.

Examples below use [`analyser-gt-desc.hfstol`] analyser file for the Udmurt
language.

```no_run
let content = std::fs::read("./analyser-gt-desc.hfstol").unwrap();
let t = hfstol::read_transducer(&content).unwrap();
println!("{:?}", t.lookup("лэсьтӥськонъёс"));
// Ok([
//     ("лэсьтӥськыны+V+Der/Он+Pl+Nom", 0.0),
//     ("лэсьтӥськон+N+Pl+Nom", 0.0),
// ])
```

See `hfstol::Transducer` trait documentation for more info.

[`hfst-optimized-lookup`]: https://github.com/hfst/hfst/blob/master/tools/src/hfst-optimized-lookup.cc
[`analyser-gt-desc.hfstol`]: https://models.uralicnlp.com/nightly/udm/index.html
