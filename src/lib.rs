#![doc(html_logo_url = "https://kdl.dev/logo.svg")]
#![doc = include_str!("../README.md")]

pub use document::*;
pub use error::*;
pub use internal_model::*;

mod document;
mod error;
mod internal_model;
mod nom_compat;
mod parser;
