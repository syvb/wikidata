//! Shared helpers for the integration test suites.
//!
//! The entity JSON in `items/` is real data fetched from Wikidata and baked in with
//! `include_str!`, so the test suite never needs network access.

#![allow(dead_code)]

use serde_json::{Value, json};
use wikidata::{ClaimValueData, Entity, EntityError};

/// Every fixture available in `items/`.
pub const FIXTURES: &[&str] = &[
    "Q1",
    "Q42",
    "Q45",
    "Q513",
    "Q31928",
    "Q106975887",
    "L361",
    "P6553",
];

/// The raw JSON text of a fixture, by entity ID.
///
/// # Panics
/// If `id` is not one of [`FIXTURES`].
pub fn raw(id: &str) -> &'static str {
    match id {
        "Q1" => include_str!("../../items/Q1.json"),
        "Q42" => include_str!("../../items/Q42.json"),
        "Q45" => include_str!("../../items/Q45.json"),
        "Q513" => include_str!("../../items/Q513.json"),
        "Q31928" => include_str!("../../items/Q31928.json"),
        "Q106975887" => include_str!("../../items/Q106975887.json"),
        "L361" => include_str!("../../items/L361.json"),
        "P6553" => include_str!("../../items/P6553.json"),
        other => panic!("no fixture for {other}"),
    }
}

/// A fixture as `serde_json::Value`, in the multi-entity `Special:EntityData` shape.
pub fn fixture(id: &str) -> Value {
    serde_json::from_str(raw(id)).expect("fixture is valid JSON")
}

/// The bare entity object of a fixture, without the `entities` wrapper.
pub fn fixture_inner(id: &str) -> Value {
    fixture(id)["entities"][id].clone()
}

/// A fixture parsed into an [`Entity`].
pub fn entity(id: &str) -> Entity {
    Entity::from_json(fixture(id)).expect("fixture parses as an entity")
}

/// The `mainsnak` of the first statement of `pid` on a fixture.
pub fn mainsnak(id: &str, pid: &str) -> Value {
    fixture(id)["entities"][id]["claims"][pid][0]["mainsnak"].clone()
}

/// The first raw statement of `pid` on a fixture.
pub fn statement_of(id: &str, pid: &str) -> Value {
    fixture(id)["entities"][id]["claims"][pid][0].clone()
}

/// Builds a `value` snak out of a datatype, a datavalue type, and a datavalue value.
pub fn snak(datatype: &str, value_type: &str, value: Value) -> Value {
    json!({
        "snaktype": "value",
        "property": "P1",
        "datatype": datatype,
        "datavalue": { "type": value_type, "value": value },
    })
}

/// Parses a snak built by [`snak`].
pub fn parse_snak(
    datatype: &str,
    value_type: &str,
    value: Value,
) -> Result<ClaimValueData, EntityError> {
    ClaimValueData::parse_snak(snak(datatype, value_type, value))
}

/// A `wikibase-entityid` snak for the given entity ID string.
pub fn entity_id_snak(id: &str) -> Value {
    snak("wikibase-item", "wikibase-entityid", json!({ "id": id }))
}

/// A minimal, well-formed statement, ready to be mutated into an error case.
pub fn base_statement() -> Value {
    json!({
        "id": "Q1$00000000-0000-0000-0000-000000000000",
        "rank": "normal",
        "mainsnak": { "snaktype": "novalue", "property": "P31", "datatype": "wikibase-item" },
    })
}

/// A minimal item wrapping a single statement on `P31`.
pub fn item_with(statement: Value) -> Value {
    json!({
        "id": "Q1",
        "type": "item",
        "claims": { "P31": [statement] },
    })
}
