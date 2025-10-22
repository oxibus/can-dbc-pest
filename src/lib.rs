#![doc = include_str!("../README.md")]

// Re-publish types to avoid requiring users to depend on pest and encoding_rs directly
#[cfg(feature = "encodings")]
pub use encoding_rs as encodings;
pub use pest::error::Error;
pub use pest::iterators::{Pair, Pairs};
pub use pest::Parser;

#[cfg(feature = "encodings")]
#[must_use]
pub fn decode_cp1252(bytes: &[u8]) -> Option<std::borrow::Cow<'_, str>> {
    let (cow, _, had_errors) = encodings::WINDOWS_1252.decode(bytes);
    if had_errors {
        None
    } else {
        Some(cow)
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "src/dbc.pest"]
pub struct DbcParser;
