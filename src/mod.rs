//! The Golang Templating Language for Rust.
//!
//! ## Example
//! ```rust
//! use gtmpl;
//!
//! let output = gtmpl::template("Finally! Some {{ . }} for Rust", "gtmpl");
//! assert_eq!(&output.unwrap(), "Finally! Some gtmpl for Rust");
//! ```
pub mod error;
mod exec;
pub mod funcs;
mod lexer;
mod node;
mod parse;
mod print_verb;
mod printf;
mod template;
mod utils;

#[doc(inline)]
pub use crate::gtmpl::template::Template;

#[doc(inline)]
pub use crate::gtmpl::exec::Context;

#[doc(inline)]
pub use gtmpl_value::Func;

pub use gtmpl_value::FuncError;

#[doc(inline)]
pub use gtmpl_value::from_value;

pub use error::TemplateError;
pub use gtmpl_value::Value;
