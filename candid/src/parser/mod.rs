//! Provides parser for Candid type and value.
//!  * `str.parse::<IDLProg>()` parses the Candid signature file to Candid AST.
//!  * `str.parse::<IDLArgs>()` parses the Candid value in text format to a struct `IDLArg` that can be used for serialization and deserialization between Candid and an enum type `IDLValue` in Rust.

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod grammar;

pub mod lexer;
pub mod types;
pub mod value;

pub type ParserError = lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>;
