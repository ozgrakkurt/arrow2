#![doc = include_str!("doc/lib.md")]
#![deny(missing_docs)]
// So that we have more control over what is `unsafe` inside an `unsafe` block
#![allow(unused_unsafe)]
//
#![allow(clippy::len_without_is_empty)]
// this landed on 1.60. Let's not force everyone to bump just yet
#![allow(clippy::unnecessary_lazy_evaluations)]
// Trait objects must be returned as a &Box<dyn Array> so that they can be cloned
#![allow(clippy::borrowed_box)]
// Allow type complexity warning to avoid API break.
#![allow(clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "nightly_build", feature(build_hasher_simple_hash_one))]

#[macro_use]
pub mod array;
pub mod bitmap;
pub mod buffer;
pub mod chunk;
pub mod error;
#[cfg(feature = "io_ipc")]
#[cfg_attr(docsrs, doc(cfg(feature = "io_ipc")))]
pub mod mmap;

pub mod offset;
pub mod scalar;
pub mod trusted_len;
pub mod types;

pub mod compute;
pub mod io;
pub mod temporal_conversions;

pub mod datatypes;

pub mod ffi;
pub mod util;

// so that documentation gets test
#[cfg(any(test, doctest))]
mod docs;

// re-exported because we return `Either` in our public API
pub use either::Either;
