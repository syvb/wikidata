//! Rust library for Wikidata. It has some support for Wikibase as well, although the main focus is
//! supporting the Wikidata instance.

pub(crate) mod entity;
pub(crate) mod ids;
pub(crate) mod text;

pub use entity::*;
pub use ids::*;
pub use text::*;
