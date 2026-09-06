//! Tests for [`ClaimValueData::parse_snak`]: every datatype, every snaktype, and the error
//! paths for malformed snaks.

mod common;

use common::{entity_id_snak, mainsnak, parse_snak, snak};
use serde_json::json;
use wikidata::*;

// ---------------------------------------------------------------------------
// Snaks taken from real entities
// ---------------------------------------------------------------------------

#[test]
fn id_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q106975887", "P31")).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(5)));

    let data = ClaimValueData::parse_snak(mainsnak("Q1", "P793")).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(323)));
}

#[test]
fn commons_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q42", "P18")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::CommonsMedia("Douglas adams portrait cropped.jpg".to_string())
    );
}

#[test]
fn quantity_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q42", "P2048")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::Quantity {
            amount: 1.96,
            lower_bound: None,
            upper_bound: None,
            unit: Some(Qid(11573)),
        }
    );
}

#[test]
fn external_id_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q42", "P213")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::ExternalID("0000 0000 8045 6315".to_string())
    );
}

#[test]
fn coordinates_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q513", "P625")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::GlobeCoordinate {
            lat: 27.988_055_555_556,
            lon: 86.925_277_777_778,
            precision: 0.000_277_777_777_777_78,
            globe: consts::EARTH,
        }
    );
}

#[test]
fn mono_text_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q42", "P1477")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::MonolingualText(Text {
            text: "Douglas No\u{eb}l Adams".to_string(),
            lang: Lang("en".to_string()),
        })
    );
}

#[test]
fn date_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q42", "P569")).unwrap();
    let ClaimValueData::DateTime {
        date_time,
        precision,
    } = data
    else {
        panic!("expected a DateTime, got {data:?}");
    };
    assert_eq!(date_time.to_string(), "1952-03-11 00:00:00 UTC");
    assert_eq!(precision, 11);
}

#[test]
fn lexeme_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q31928", "P6254")).unwrap();
    assert_eq!(data, ClaimValueData::Lexeme(Lid(361)));

    let data = ClaimValueData::parse_snak(mainsnak("L361", "P5402")).unwrap();
    assert_eq!(data, ClaimValueData::Lexeme(Lid(232_582)));
}

#[test]
fn geo_shape_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q45", "P3896")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::GeoShape("Data:Portugal.map".to_string())
    );
}

#[test]
fn url_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("P6553", "P3254")).unwrap();
    assert_eq!(
        data,
        ClaimValueData::Url(
            "https://www.wikidata.org/wiki/Wikidata:Property_proposal/personal_pronoun".to_string()
        )
    );
}

#[test]
fn string_snak() {
    let data = ClaimValueData::parse_snak(mainsnak("Q1", "P373")).unwrap();
    assert_eq!(data, ClaimValueData::String("Universe".to_string()));
}

// ---------------------------------------------------------------------------
// Snaktypes
// ---------------------------------------------------------------------------

#[test]
fn no_value_and_unknown_value_snaks() {
    // real `novalue` and `somevalue` snaks, which carry no datavalue at all
    assert_eq!(
        ClaimValueData::parse_snak(mainsnak("Q45", "P3238")).unwrap(),
        ClaimValueData::NoValue
    );
    assert_eq!(
        ClaimValueData::parse_snak(mainsnak("Q513", "P2659")).unwrap(),
        ClaimValueData::NoValue
    );
    assert_eq!(
        ClaimValueData::parse_snak(mainsnak("Q1", "P1419")).unwrap(),
        ClaimValueData::UnknownValue
    );
}

#[test]
fn no_value_is_the_default_claim_value() {
    assert_eq!(ClaimValueData::default(), ClaimValueData::NoValue);
}

#[test]
fn unknown_snaktype_is_rejected() {
    let snak = json!({ "snaktype": "sometimesvalue", "datatype": "string" });
    assert_eq!(
        ClaimValueData::parse_snak(snak),
        Err(EntityError::InvalidSnaktype)
    );
}

#[test]
fn missing_snaktype_or_datatype_is_rejected() {
    assert_eq!(
        ClaimValueData::parse_snak(json!({ "datatype": "string" })),
        Err(EntityError::ExpectedString)
    );
    assert_eq!(
        ClaimValueData::parse_snak(json!({ "snaktype": "novalue" })),
        Err(EntityError::ExpectedString)
    );
    // a non-string datatype is just as bad as a missing one
    assert_eq!(
        ClaimValueData::parse_snak(json!({ "snaktype": "novalue", "datatype": 31 })),
        Err(EntityError::ExpectedString)
    );
}

#[test]
fn value_snak_without_a_datavalue_is_rejected() {
    let snak = json!({ "snaktype": "value", "datatype": "string" });
    assert_eq!(
        ClaimValueData::parse_snak(snak),
        Err(EntityError::InvalidSnaktype)
    );
    // ...as is one whose datavalue has no type
    let snak = json!({
        "snaktype": "value",
        "datatype": "string",
        "datavalue": { "value": "x" },
    });
    assert_eq!(
        ClaimValueData::parse_snak(snak),
        Err(EntityError::InvalidSnaktype)
    );
}

#[test]
fn unknown_datavalue_type_is_rejected() {
    assert_eq!(
        parse_snak("string", "not-a-real-type", json!("x")),
        Err(EntityError::UnknownDatatype)
    );
}

// ---------------------------------------------------------------------------
// String-valued datatypes
// ---------------------------------------------------------------------------

#[test]
fn every_string_datatype_maps_to_its_own_variant() {
    /// Constructor for a `ClaimValueData` variant that wraps a single string.
    type StringVariant = fn(String) -> ClaimValueData;

    let cases: &[(&str, StringVariant)] = &[
        ("string", ClaimValueData::String),
        ("commonsMedia", ClaimValueData::CommonsMedia),
        ("external-id", ClaimValueData::ExternalID),
        ("math", ClaimValueData::MathExpr),
        ("geo-shape", ClaimValueData::GeoShape),
        ("musical-notation", ClaimValueData::MusicNotation),
        ("tabular-data", ClaimValueData::TabularData),
        ("url", ClaimValueData::Url),
    ];
    for (datatype, variant) in cases {
        assert_eq!(
            parse_snak(datatype, "string", json!("value")),
            Ok(variant("value".to_string())),
            "datatype {datatype}"
        );
    }
}

#[test]
fn string_datavalue_with_an_unknown_datatype_is_rejected() {
    assert_eq!(
        parse_snak("wikibase-item", "string", json!("Q5")),
        Err(EntityError::InvalidDatatype)
    );
}

#[test]
fn string_datavalue_must_hold_a_string() {
    assert_eq!(
        parse_snak("string", "string", json!(5)),
        Err(EntityError::ExpectedStringDatatype)
    );
    assert_eq!(
        parse_snak("string", "string", json!(null)),
        Err(EntityError::ExpectedStringDatatype)
    );
}

#[test]
fn empty_strings_are_preserved() {
    assert_eq!(
        parse_snak("string", "string", json!("")),
        Ok(ClaimValueData::String(String::new()))
    );
}

// ---------------------------------------------------------------------------
// Entity ID datatypes
// ---------------------------------------------------------------------------

#[test]
fn every_entity_id_kind_maps_to_its_own_variant() {
    let cases = [
        ("Q5", ClaimValueData::Item(Qid(5))),
        ("P31", ClaimValueData::Property(Pid(31))),
        ("L361", ClaimValueData::Lexeme(Lid(361))),
        ("L361-F1", ClaimValueData::Form(Fid(Lid(361), 1))),
        ("L361-S2", ClaimValueData::Sense(Sid(Lid(361), 2))),
    ];
    for (id, expected) in cases {
        assert_eq!(
            ClaimValueData::parse_snak(entity_id_snak(id)),
            Ok(expected),
            "id {id}"
        );
    }
}

#[test]
fn bad_entity_ids_are_rejected() {
    for id in [
        "",           // empty
        "E123",       // entity schemas are unsupported
        "X1",         // unknown prefix
        "Q",          // no number
        "Qfive",      // not a number
        "Q-1",        // negative
        "L1-X2",      // unknown sub-ID letter
        "L1-F2-S3",   // too many parts
        "L1-F",       // no sub-ID number
        "L1-F99999",  // sub-ID index overflows a u16
        "\u{e9}1",    // non-ASCII prefix
        "L\u{e9}-F1", // non-ASCII lexeme number
        "L1-F\u{e9}", // non-ASCII sub-ID number
    ] {
        assert_eq!(
            ClaimValueData::parse_snak(entity_id_snak(id)),
            Err(EntityError::BadId),
            "id {id:?} should be rejected"
        );
    }
}

#[test]
fn entity_id_must_be_a_string() {
    // a numeric `id`, and an entity ID given only as `numeric-id`, are both unusable
    for value in [json!({ "id": 5 }), json!({ "numeric-id": 5 })] {
        assert_eq!(
            ClaimValueData::parse_snak(snak("wikibase-item", "wikibase-entityid", value)),
            Err(EntityError::ExpectedString)
        );
    }
}

// ---------------------------------------------------------------------------
// Quantities
// ---------------------------------------------------------------------------

#[test]
fn quantity_with_bounds() {
    let data = parse_snak(
        "quantity",
        "quantity",
        json!({
            "amount": "+1.96",
            "lowerBound": "+1.95",
            "upperBound": "+1.97",
            "unit": "http://www.wikidata.org/entity/Q11573",
        }),
    );
    assert_eq!(
        data,
        Ok(ClaimValueData::Quantity {
            amount: 1.96,
            lower_bound: Some(1.95),
            upper_bound: Some(1.97),
            unit: Some(consts::METRE),
        })
    );
}

#[test]
fn dimensionless_quantities_have_no_unit() {
    // Wikibase writes the unitless unit as the literal string "1"
    let data = parse_snak(
        "quantity",
        "quantity",
        json!({ "amount": "+5", "unit": "1" }),
    );
    assert_eq!(
        data,
        Ok(ClaimValueData::Quantity {
            amount: 5.0,
            lower_bound: None,
            upper_bound: None,
            unit: None,
        })
    );
}

#[test]
fn quantity_requires_an_amount() {
    assert_eq!(
        parse_snak("quantity", "quantity", json!({ "unit": "1" })),
        Err(EntityError::ExpectedNumberString)
    );
    assert_eq!(
        parse_snak("quantity", "quantity", json!({ "amount": "twelve" })),
        Err(EntityError::FloatParse)
    );
}

#[test]
fn quantity_accepts_numeric_and_signed_string_amounts() {
    for (amount, expected) in [
        (json!("+5"), 5.0),
        (json!("-5"), -5.0),
        (json!("5"), 5.0),
        (json!(5), 5.0),
        (json!(-2.5), -2.5),
        (json!("+0"), 0.0),
    ] {
        let data = parse_snak("quantity", "quantity", json!({ "amount": amount }));
        assert_eq!(
            data,
            Ok(ClaimValueData::Quantity {
                amount: expected,
                lower_bound: None,
                upper_bound: None,
                unit: None,
            }),
            "amount {amount}"
        );
    }
}

// ---------------------------------------------------------------------------
// Globe coordinates
// ---------------------------------------------------------------------------

#[test]
fn globe_coordinate_defaults_precision_when_missing() {
    let data = parse_snak(
        "globe-coordinate",
        "globecoordinate",
        json!({
            "latitude": 1.5,
            "longitude": -2.5,
            "globe": "http://www.wikidata.org/entity/Q2",
        }),
    );
    assert_eq!(
        data,
        Ok(ClaimValueData::GlobeCoordinate {
            lat: 1.5,
            lon: -2.5,
            precision: 1.0,
            globe: consts::EARTH,
        })
    );
}

#[test]
fn globe_coordinate_requires_latitude_and_longitude() {
    let data = parse_snak(
        "globe-coordinate",
        "globecoordinate",
        json!({ "longitude": 1.0, "globe": "http://www.wikidata.org/entity/Q2" }),
    );
    assert_eq!(data, Err(EntityError::ExpectedNumberString));
}

#[test]
fn globe_coordinate_requires_a_wikidata_globe() {
    // a globe that is not a Wikidata entity URI cannot be represented
    let data = parse_snak(
        "globe-coordinate",
        "globecoordinate",
        json!({ "latitude": 1.0, "longitude": 1.0, "globe": "http://example.com/moon" }),
    );
    assert_eq!(data, Err(EntityError::ExpectedQidString));

    let data = parse_snak(
        "globe-coordinate",
        "globecoordinate",
        json!({ "latitude": 1.0, "longitude": 1.0, "globe": 2 }),
    );
    assert_eq!(data, Err(EntityError::ExpectedUriString));
}

// ---------------------------------------------------------------------------
// Times
// ---------------------------------------------------------------------------

#[test]
fn time_snak_keeps_its_precision() {
    let data = parse_snak(
        "time",
        "time",
        json!({ "time": "+2001-05-11T00:00:00Z", "precision": 11 }),
    );
    let Ok(ClaimValueData::DateTime {
        date_time,
        precision,
    }) = data
    else {
        panic!("expected a DateTime, got {data:?}");
    };
    assert_eq!(date_time.to_string(), "2001-05-11 00:00:00 UTC");
    assert_eq!(precision, 11);
}

#[test]
fn unrepresentable_times_become_unknown_values() {
    // Q1 (the universe) has a start time of about 13.8 billion years ago, which chrono cannot
    // represent; the crate degrades to `UnknownValue` rather than failing the whole entity
    assert_eq!(
        ClaimValueData::parse_snak(mainsnak("Q1", "P580")).unwrap(),
        ClaimValueData::UnknownValue
    );
    for time in ["", "+notayear", "+2001-13-01T00:00:00Z", "+1900-02-29"] {
        assert_eq!(
            parse_snak("time", "time", json!({ "time": time, "precision": 11 })),
            Ok(ClaimValueData::UnknownValue),
            "time {time:?}"
        );
    }
}

#[test]
fn time_snak_requires_a_precision() {
    assert_eq!(
        parse_snak("time", "time", json!({ "time": "+2001-05-11T00:00:00Z" })),
        Err(EntityError::InvalidPrecision)
    );
}

#[test]
fn time_snak_requires_a_time_string() {
    assert_eq!(
        parse_snak("time", "time", json!({ "precision": 11 })),
        Err(EntityError::ExpectedString)
    );
}

// ---------------------------------------------------------------------------
// Monolingual text
// ---------------------------------------------------------------------------

#[test]
fn monolingual_text_needs_text_and_a_language() {
    assert_eq!(
        parse_snak(
            "monolingualtext",
            "monolingualtext",
            json!({ "text": "bonjour", "language": "fr" })
        ),
        Ok(ClaimValueData::MonolingualText(Text {
            text: "bonjour".to_string(),
            lang: Lang("fr".to_string()),
        }))
    );
    assert_eq!(
        parse_snak(
            "monolingualtext",
            "monolingualtext",
            json!({ "text": "hi" })
        ),
        Err(EntityError::ExpectedString)
    );
    assert_eq!(
        parse_snak(
            "monolingualtext",
            "monolingualtext",
            json!({ "language": "en" })
        ),
        Err(EntityError::ExpectedString)
    );
}

// ---------------------------------------------------------------------------
// Every fixture snak parses
// ---------------------------------------------------------------------------

#[test]
fn every_mainsnak_in_every_fixture_parses() {
    for id in common::FIXTURES {
        let fixture = common::fixture(id);
        let claims = fixture["entities"][id]["claims"].as_object().unwrap();
        for (pid, statements) in claims {
            for statement in statements.as_array().unwrap() {
                let snak = statement["mainsnak"].clone();
                assert!(
                    ClaimValueData::parse_snak(snak).is_ok(),
                    "{id} {pid} mainsnak did not parse"
                );
            }
        }
    }
}
