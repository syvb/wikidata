//! Rust library for Wikidata. It has some support for Wikibase as well, although the main focus is
//! supporting the Wikidata instance.

#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

pub(crate) mod entity;
pub(crate) mod ids;
pub(crate) mod text;

pub use entity::*;
pub use ids::*;
pub use text::*;
