//! # HFST Optimized Lookup for Rust.
//!
//! This crate is a port of `hfst-optimized-lookup` to Rust.
//!
//! To read a transducer, use [`transducers::read_transducer()`] function with a
//! `&[u8]` buffer of binary contents.  Created transducers will have a
//! [`transducers::Transducer`] trait.
//!
//! Only weighted transducers are supported at the moment.
//!
//! # Examples
//!
//! Examples below use [`analyser-gt-desc.hfstol`](https://models.uralicnlp.com/nightly/udm/index.html)
//! analyser file for the Udmurt language.
//!
//! ```no_run
//! let content = std::fs::read("./analyser-gt-desc.hfstol").unwrap();
//! let t = hfstol::read_transducer(&content).unwrap();
//! println!("{:?}", t.lookup("лэсьтӥськонъёс"));
//! // Ok([
//! //     ("лэсьтӥськыны+V+Der/Он+Pl+Nom", 0.0),
//! //     ("лэсьтӥськон+N+Pl+Nom", 0.0),
//! // ])
//! ```

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod transducers;
pub mod trie;

mod mutex;
mod parser_utils;

pub use transducers::{
    Error,
    Transducer,
    read_transducer,
};
