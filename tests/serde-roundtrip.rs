//! The crate's own `serde` representation (which is deliberately *not* the Wikibase JSON
//! format) must survive a round trip and stay stable.

mod common;

use chrono::{TimeZone, Utc};
use std::collections::BTreeMap;
use wikidata::*;

fn round_trip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("serializes");
    let parsed: T = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(&parsed, value, "round trip changed the value: {json}");
}

#[test]
fn every_fixture_round_trips() {
    for id in common::FIXTURES {
        let entity = common::entity(id);
        round_trip(&entity);
    }
}

#[test]
fn every_claim_value_variant_round_trips() {
    let variants = vec![
        ClaimValueData::CommonsMedia("A.jpg".to_string()),
        ClaimValueData::GlobeCoordinate {
            lat: -1.5,
            lon: 2.5,
            precision: 0.001,
            globe: consts::EARTH,
        },
        ClaimValueData::Item(Qid(42)),
        ClaimValueData::Property(Pid(31)),
        ClaimValueData::String("text".to_string()),
        ClaimValueData::MonolingualText(Text {
            text: "bonjour".to_string(),
            lang: Lang("fr".to_string()),
        }),
        ClaimValueData::MultilingualText(vec![
            Text {
                text: "hello".to_string(),
                lang: Lang("en".to_string()),
            },
            Text {
                text: "bonjour".to_string(),
                lang: Lang("fr".to_string()),
            },
        ]),
        ClaimValueData::ExternalID("0000-0002".to_string()),
        ClaimValueData::Quantity {
            amount: 1.96,
            lower_bound: Some(1.95),
            upper_bound: Some(1.97),
            unit: Some(consts::METRE),
        },
        ClaimValueData::DateTime {
            date_time: Utc.with_ymd_and_hms(1952, 3, 11, 0, 0, 0).unwrap(),
            precision: 11,
        },
        ClaimValueData::Url("https://iter.ca/".to_string()),
        ClaimValueData::MathExpr("E = mc^2".to_string()),
        ClaimValueData::GeoShape("Data:Portugal.map".to_string()),
        ClaimValueData::MusicNotation("\\relative c' { c d e f }".to_string()),
        ClaimValueData::TabularData("Data:A.tab".to_string()),
        ClaimValueData::Lexeme(Lid(361)),
        ClaimValueData::Form(Fid(Lid(361), 1)),
        ClaimValueData::Sense(Sid(Lid(361), 2)),
        ClaimValueData::NoValue,
        ClaimValueData::UnknownValue,
    ];
    for variant in &variants {
        round_trip(variant);
    }
}

#[test]
fn supporting_types_round_trip() {
    round_trip(&Rank::Deprecated);
    round_trip(&Rank::Normal);
    round_trip(&Rank::Preferred);
    round_trip(&EntityType::Entity);
    round_trip(&EntityType::Property);
    round_trip(&EntityType::Lexeme);
    round_trip(&Lang("en".to_string()));
    round_trip(&SiteName("enwiki".to_string()));
    round_trip(&Text {
        text: "hi".to_string(),
        lang: Lang("en".to_string()),
    });
    round_trip(&SitelinkValue {
        title: "Douglas Adams".to_string(),
        badges: vec![Qid(17_437_798)],
        url: Some("https://en.wikipedia.org/wiki/Douglas_Adams".to_string()),
    });
    round_trip(&SitelinkValue::default());
    round_trip(&ReferenceGroup {
        claims: vec![(consts::STATED_IN, ClaimValueData::Item(Qid(5)))],
        hash: "deadbeef".to_string(),
    });
    round_trip(&ClaimValue::default());
    round_trip(&ClaimValue {
        data: ClaimValueData::Item(Qid(5)),
        rank: Rank::Preferred,
        id: "Q1$a".to_string(),
        qualifiers: vec![(Pid(585), ClaimValueData::NoValue)],
        references: vec![ReferenceGroup {
            claims: vec![],
            hash: String::new(),
        }],
    });
    round_trip(&Entity {
        id: WikiId::EntityId(Qid(1)),
        claims: vec![],
        entity_type: EntityType::Entity,
        descriptions: BTreeMap::new(),
        labels: BTreeMap::new(),
        aliases: BTreeMap::new(),
        sitelinks: BTreeMap::new(),
    });
}

#[test]
fn representation_is_stable() {
    // guards against an accidental change to the wire format
    assert_eq!(
        serde_json::to_string(&ClaimValueData::Item(Qid(5))).unwrap(),
        r#"{"Item":5}"#
    );
    assert_eq!(
        serde_json::to_string(&ClaimValueData::NoValue).unwrap(),
        r#""NoValue""#
    );
    assert_eq!(
        serde_json::to_string(&ClaimValueData::Sense(Sid(Lid(1), 2))).unwrap(),
        r#"{"Sense":[1,2]}"#
    );
    assert_eq!(
        serde_json::to_string(&Rank::Preferred).unwrap(),
        r#""Preferred""#
    );
    assert_eq!(
        serde_json::to_string(&EntityType::Lexeme).unwrap(),
        r#""Lexeme""#
    );
    assert_eq!(
        serde_json::to_string(&Text {
            text: "hi".to_string(),
            lang: Lang("en".to_string()),
        })
        .unwrap(),
        r#"{"text":"hi","lang":"en"}"#
    );
    assert_eq!(
        serde_json::to_string(&SitelinkValue {
            title: "T".to_string(),
            badges: vec![Qid(1)],
            url: None,
        })
        .unwrap(),
        r#"{"title":"T","badges":[1],"url":null}"#
    );
}

#[test]
fn defaults_are_the_documented_ones() {
    assert_eq!(ClaimValueData::default(), ClaimValueData::NoValue);
    assert_eq!(Rank::default(), Rank::Normal);
    let claim = ClaimValue::default();
    assert_eq!(claim.data, ClaimValueData::NoValue);
    assert_eq!(claim.rank, Rank::Normal);
    assert!(claim.id.is_empty());
    assert!(claim.qualifiers.is_empty());
    assert!(claim.references.is_empty());
}

#[test]
fn ranks_are_ordered_from_worst_to_best() {
    assert!(Rank::Deprecated < Rank::Normal);
    assert!(Rank::Normal < Rank::Preferred);
    let mut ranks = vec![Rank::Preferred, Rank::Deprecated, Rank::Normal];
    ranks.sort_unstable();
    assert_eq!(ranks, vec![Rank::Deprecated, Rank::Normal, Rank::Preferred]);
    assert_eq!(ranks.iter().max(), Some(&Rank::Preferred));
}
