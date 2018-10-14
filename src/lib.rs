#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs,
    unused_extern_crates,
    unused_qualifications,
    unused_results
)]

//! Helper crate with types for common Slack objects

#[macro_use]
extern crate serde_derive;

pub mod event;
pub mod message;
pub mod slack;
