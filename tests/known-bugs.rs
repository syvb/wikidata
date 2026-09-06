//! Regression tests for bugs that are known but **not yet fixed**.
//!
//! Every test in this file asserts the behavior the crate *should* have, and is `#[ignore]`d
//! so that the suite stays green until the underlying bug is fixed. When one of these is
//! fixed, drop its `#[ignore]` attribute and move the test into the suite it belongs to.
//!
//! Run them with `cargo test -- --ignored`.

mod common;

use common::{base_statement, statement_of};
use serde_json::json;
use wikidata::*;

/// `Entity::from_json` stringifies the sitelink `url` with `Value::to_string`, which keeps the
/// surrounding JSON quotes instead of unwrapping the string.
#[test]
#[ignore = "BUG: sitelink URLs keep their JSON quotes"]
fn sitelink_urls_are_not_quoted() {
    let e = common::entity("Q42");
    let enwiki = &e.sitelinks[&SiteName("enwiki".to_string())];
    assert_eq!(
        enwiki.url.as_deref(),
        Some("https://en.wikipedia.org/wiki/Douglas_Adams")
    );
}

/// `ClaimValue::get_prop_from_snak` sizes its claim vector with
/// `reference_group["snaks"].as_array()`, but `snaks` is an object, so `as_array` returns
/// `None` and the `?` discards the whole statement. Every statement carrying a reference —
/// which is most of them — comes back as `None`.
#[test]
#[ignore = "BUG: get_prop_from_snak drops every statement that has references"]
fn get_prop_from_snak_keeps_statements_with_references() {
    // P19 (place of birth) on Q42 is a normal-rank statement with four reference groups
    let statement = statement_of("Q42", "P19");
    let claim = ClaimValue::get_prop_from_snak(statement, false)
        .expect("a statement with references should still parse");
    assert_eq!(claim.data, ClaimValueData::Item(Qid(350)));
    assert_eq!(claim.references.len(), 4);
    assert!(!claim.references[0].claims.is_empty());
}

/// `parse_wb_time` slices the seconds field as `[0..2]` without checking its length, so a
/// truncated time panics instead of returning an error.
#[test]
#[ignore = "BUG: parse_wb_time panics on a seconds field shorter than two characters"]
fn short_seconds_are_an_error_not_a_panic() {
    for time in ["+2001-12-31T12:34:5", "+2001-12-31T12:34:"] {
        let snak = json!({
            "snaktype": "value",
            "datatype": "time",
            "datavalue": { "type": "time", "value": { "time": time, "precision": 11 } },
        });
        assert_eq!(
            ClaimValueData::parse_snak(snak),
            Ok(ClaimValueData::UnknownValue),
            "time {time:?}"
        );
    }
}

/// `parse_wb_time` strips the era sign with `&time[1..]` after only comparing the first
/// `char`, so a time starting with a multi-byte character panics on a character boundary.
#[test]
#[ignore = "BUG: parse_wb_time panics on a non-ASCII leading character"]
fn non_ascii_times_are_an_error_not_a_panic() {
    let snak = json!({
        "snaktype": "value",
        "datatype": "time",
        "datavalue": { "type": "time", "value": { "time": "\u{e9}2001-12-31", "precision": 11 } },
    });
    assert_eq!(
        ClaimValueData::parse_snak(snak),
        Ok(ClaimValueData::UnknownValue)
    );
}

/// `get_prop_from_snak` builds property IDs with `pid[1..].parse()`, which panics when the
/// key's first character is multi-byte.
#[test]
#[ignore = "BUG: get_prop_from_snak panics on a non-ASCII property key"]
fn non_ascii_qualifier_keys_are_an_error_not_a_panic() {
    let mut statement = base_statement();
    statement["qualifiers"] = json!({
        "\u{e9}1": [{ "snaktype": "novalue", "property": "P1", "datatype": "string" }],
    });
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);
}

/// `parse_wb_time` treats any leading character that is not `+` as the BCE sign, and then
/// strips it. A time without an era sign silently loses its first digit and comes back as a
/// BCE date instead of being rejected.
#[test]
#[ignore = "BUG: times without an era sign are silently parsed as BCE"]
fn times_without_an_era_sign_are_rejected() {
    let snak = json!({
        "snaktype": "value",
        "datatype": "time",
        "datavalue": { "type": "time", "value": { "time": "2001-12-31T00:00:00Z", "precision": 11 } },
    });
    // currently parses as the year -1
    assert_eq!(
        ClaimValueData::parse_snak(snak),
        Ok(ClaimValueData::UnknownValue)
    );
}

/// `consts::PLACE_OF_DEATH` is defined as P570, which is *date* of death (and is already
/// bound to `consts::DATE_OF_DEATH`). Place of death is P20.
#[test]
#[ignore = "BUG: consts::PLACE_OF_DEATH is P570 (date of death) instead of P20"]
fn place_of_death_is_p20() {
    assert_eq!(consts::PLACE_OF_DEATH, Pid(20));
    assert_ne!(consts::PLACE_OF_DEATH, consts::DATE_OF_DEATH);
}

/// Three entries in the unit-suffix table do not match their unit.
#[test]
#[ignore = "BUG: the kelvin unit suffix is written as a degree unit"]
fn kelvin_has_no_degree_sign() {
    // kelvin is not measured in degrees, unlike Celsius and Fahrenheit
    assert_eq!(consts::KELVIN.unit_suffix(), Some(" K"));
}

#[test]
#[ignore = "BUG: milligram per cubic metre is suffixed as milligram per cubic centimetre"]
fn milligram_per_cubic_metre_suffix() {
    assert_eq!(
        consts::MILLIGRAM_PER_CUBIC_METER.unit_suffix(),
        Some(" mg/m\u{b3}")
    );
}

#[test]
#[ignore = "BUG: the milligram per kilogram suffix is missing the 'g' in 'kg'"]
fn milligram_per_kilogram_suffix() {
    assert_eq!(consts::MILLIGRAM_PER_KILOGRAM.unit_suffix(), Some(" mg/kg"));
}
