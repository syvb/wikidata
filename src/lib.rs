//! Rust library for Wikidata. It has some support for Wikibase as well, although the main focus is
//! supporting the Wikidata instance.
//!
//! ## A note on serialization
//! Many items in this crate implement [`serde::Serialize`] and [`serde::Deserialize`]. Note that
//! the JSON serialization of entities provided by these traits is not the same as the
//! serialization used by Wikidata in data dumps and `Special:EntityData`, but is instead a
//! serialization specific to this crate.

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
