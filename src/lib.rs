#[cfg(feature = "encoding")]
pub use encoding_rs as encodings;

#[cfg(feature = "encoding")]
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
