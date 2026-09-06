//! Tests for the public ID types: parsing, display, ordering, and serialization.

use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;
use wikidata::*;

#[test]
fn display_ids() {
    assert_eq!(Qid(42).to_string(), "Q42");
    assert_eq!(Pid(31).to_string(), "P31");
    assert_eq!(Lid(361).to_string(), "L361");
    assert_eq!(Fid(Lid(3), 11).to_string(), "L3-F11");
    assert_eq!(Sid(Lid(5), 9).to_string(), "L5-S9");
}

#[test]
fn display_boundary_values() {
    assert_eq!(Qid(0).to_string(), "Q0");
    assert_eq!(Qid(u64::MAX).to_string(), "Q18446744073709551615");
    assert_eq!(
        Fid(Lid(u64::MAX), u16::MAX).to_string(),
        "L18446744073709551615-F65535"
    );
    assert_eq!(Sid(Lid(0), 0).to_string(), "L0-S0");
}

#[test]
fn json_urls() {
    assert_eq!(
        Qid(42).json_url(),
        "https://www.wikidata.org/wiki/Special:EntityData/Q42.json"
    );
    assert_eq!(
        Pid(31).json_url(),
        "https://www.wikidata.org/wiki/Special:EntityData/P31.json"
    );
    assert_eq!(
        Lid(361).json_url(),
        "https://www.wikidata.org/wiki/Special:EntityData/L361.json"
    );
}

#[test]
fn parse_simple_ids() {
    assert_eq!(Qid::from_str("Q42"), Ok(Qid(42)));
    assert_eq!(Qid::from_str("Q0"), Ok(Qid(0)));
    assert_eq!(Pid::from_str("P1341"), Ok(Pid(1341)));
    assert_eq!(Lid::from_str("L944114"), Ok(Lid(944_114)));
    assert_eq!(Qid::from_str("Q18446744073709551615"), Ok(Qid(u64::MAX)));
}

#[test]
fn parse_lexeme_subids() {
    assert_eq!(Fid::from_str("L3-F11"), Ok(Fid(Lid(3), 11)));
    assert_eq!(Sid::from_str("L1341-S123"), Ok(Sid(Lid(1341), 123)));
    assert_eq!(Fid::from_str("L0-F0"), Ok(Fid(Lid(0), 0)));
}

#[test]
fn round_trip_display_and_parse() {
    for qid in [Qid(0), Qid(1), Qid(42), Qid(u64::MAX)] {
        assert_eq!(Qid::from_str(&qid.to_string()), Ok(qid));
        assert_eq!(
            WikiId::from_str(&qid.to_string()),
            Ok(WikiId::EntityId(qid))
        );
    }
    for pid in [Pid(0), Pid(31), Pid(u64::MAX)] {
        assert_eq!(Pid::from_str(&pid.to_string()), Ok(pid));
        assert_eq!(
            WikiId::from_str(&pid.to_string()),
            Ok(WikiId::PropertyId(pid))
        );
    }
    for lid in [Lid(0), Lid(361), Lid(u64::MAX)] {
        assert_eq!(Lid::from_str(&lid.to_string()), Ok(lid));
        assert_eq!(
            WikiId::from_str(&lid.to_string()),
            Ok(WikiId::LexemeId(lid))
        );
    }
    for fid in [
        Fid(Lid(0), 0),
        Fid(Lid(3), 11),
        Fid(Lid(u64::MAX), u16::MAX),
    ] {
        assert_eq!(Fid::from_str(&fid.to_string()), Ok(fid));
    }
    for sid in [Sid(Lid(0), 0), Sid(Lid(5), 9), Sid(Lid(u64::MAX), u16::MAX)] {
        assert_eq!(Sid::from_str(&sid.to_string()), Ok(sid));
    }
}

#[test]
fn reject_wrong_prefix() {
    assert_eq!(Qid::from_str("P42"), Err(IdParseError::InvalidPrefix));
    assert_eq!(Pid::from_str("Q1341"), Err(IdParseError::InvalidPrefix));
    assert_eq!(Lid::from_str("Q1"), Err(IdParseError::InvalidPrefix));
    // no prefix at all
    assert_eq!(Pid::from_str("1341"), Err(IdParseError::InvalidPrefix));
    // the prefix is case sensitive
    assert_eq!(Qid::from_str("q42"), Err(IdParseError::InvalidPrefix));
    // leading whitespace is not trimmed
    assert_eq!(Qid::from_str(" Q42"), Err(IdParseError::InvalidPrefix));
    // an empty string has no prefix
    assert_eq!(Qid::from_str(""), Err(IdParseError::InvalidPrefix));
}

#[test]
fn reject_unparseable_numbers() {
    for bad in [
        "Q",
        "Qabc",
        "Q-1",
        "Q4 2",
        "Q42 ",
        "Q4.2",
        "Q18446744073709551616",
    ] {
        assert!(
            matches!(Qid::from_str(bad), Err(IdParseError::UnparseableNumber(_))),
            "expected {bad:?} to fail to parse, got {:?}",
            Qid::from_str(bad)
        );
    }
}

#[test]
fn reject_non_ascii_without_panicking() {
    // slicing off the prefix must not split a multi-byte character
    for bad in ["\u{e9}42", "Q\u{e9}", "Q4\u{fe0f}2", "\u{24c6}42"] {
        assert!(Qid::from_str(bad).is_err(), "{bad:?} should not parse");
        assert!(WikiId::from_str(bad).is_err(), "{bad:?} should not parse");
    }
    for bad in ["L\u{e9}-F1", "L1-F\u{e9}", "L1-\u{e9}1", "\u{e9}1-F1"] {
        assert!(Fid::from_str(bad).is_err(), "{bad:?} should not parse");
        assert!(Sid::from_str(bad).is_err(), "{bad:?} should not parse");
    }
}

#[test]
fn reject_bad_lexeme_subids() {
    // wrong lexeme prefix
    assert_eq!(Fid::from_str("Q3-F2"), Err(IdParseError::InvalidPrefix));
    assert_eq!(Sid::from_str("S1341"), Err(IdParseError::InvalidPrefix));
    // form and sense IDs are not interchangeable
    assert_eq!(Fid::from_str("L1-S2"), Err(IdParseError::InvalidPrefix));
    assert_eq!(Sid::from_str("L1-F2"), Err(IdParseError::InvalidPrefix));
    // an unknown sub-ID letter
    assert_eq!(
        Sid::from_str("L1341-A123"),
        Err(IdParseError::InvalidPrefix)
    );
    // missing the sub-ID entirely
    assert_eq!(Sid::from_str("L1341"), Err(IdParseError::TooFewParts));
    assert_eq!(Fid::from_str("L3"), Err(IdParseError::TooFewParts));
    // one part too many
    assert_eq!(Fid::from_str("L3-F2-S6"), Err(IdParseError::TooManyParts));
    assert_eq!(Fid::from_str("L3-F2-"), Err(IdParseError::TooManyParts));
    // unparseable numbers on either side of the dash
    assert!(matches!(
        Fid::from_str("Labc-F2"),
        Err(IdParseError::UnparseableNumber(_))
    ));
    assert!(matches!(
        Fid::from_str("L1-F"),
        Err(IdParseError::UnparseableNumber(_))
    ));
    // the sub-ID index is a u16
    assert!(matches!(
        Fid::from_str("L1-F65536"),
        Err(IdParseError::UnparseableNumber(_))
    ));
}

#[test]
fn wiki_id_dispatches_on_prefix() {
    assert_eq!(WikiId::from_str("Q1341"), Ok(WikiId::EntityId(Qid(1341))));
    assert_eq!(WikiId::from_str("P1341"), Ok(WikiId::PropertyId(Pid(1341))));
    assert_eq!(WikiId::from_str("L1341"), Ok(WikiId::LexemeId(Lid(1341))));
    // `EntitySchema` IDs are not supported
    assert_eq!(WikiId::from_str("E123"), Err(IdParseError::InvalidPrefix));
    assert_eq!(WikiId::from_str("A123"), Err(IdParseError::InvalidPrefix));
    assert_eq!(WikiId::from_str(""), Err(IdParseError::InvalidPrefix));
    // forms and senses are not entity IDs
    assert!(WikiId::from_str("L1341-F123").is_err());
    assert!(WikiId::from_str("L1341-S123").is_err());
}

#[test]
fn ids_are_ordered_numerically() {
    let mut qids = vec![Qid(10), Qid(2), Qid(1)];
    qids.sort_unstable();
    assert_eq!(qids, vec![Qid(1), Qid(2), Qid(10)]);

    // lexeme sub-IDs sort by lexeme first, then by index
    let mut fids = vec![Fid(Lid(2), 1), Fid(Lid(1), 9), Fid(Lid(1), 10)];
    fids.sort_unstable();
    assert_eq!(fids, vec![Fid(Lid(1), 9), Fid(Lid(1), 10), Fid(Lid(2), 1)]);
}

#[test]
fn ids_work_as_map_and_set_keys() {
    let set: HashSet<Qid> = [Qid(1), Qid(1), Qid(2)].into_iter().collect();
    assert_eq!(set.len(), 2);

    let ordered: BTreeSet<Pid> = [Pid(3), Pid(1)].into_iter().collect();
    assert_eq!(
        ordered.iter().copied().collect::<Vec<_>>(),
        vec![Pid(1), Pid(3)]
    );

    // `WikiId` is `Hash` but not `Ord`, so only hashing is checked
    let ids: HashSet<WikiId> = [WikiId::EntityId(Qid(1)), WikiId::EntityId(Qid(1))]
        .into_iter()
        .collect();
    assert_eq!(ids.len(), 1);
}

#[test]
fn ids_round_trip_through_serde() {
    macro_rules! round_trip {
        ($value:expr) => {{
            let value = $value;
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(serde_json::from_str(&json).ok(), Some(value), "{json}");
        }};
    }
    round_trip!(Qid(42));
    round_trip!(Pid(31));
    round_trip!(Lid(361));
    round_trip!(Fid(Lid(3), 11));
    round_trip!(Sid(Lid(5), 9));
    round_trip!(WikiId::EntityId(Qid(42)));
    round_trip!(WikiId::PropertyId(Pid(31)));
    round_trip!(WikiId::LexemeId(Lid(361)));

    // IDs serialize transparently as their numeric value
    assert_eq!(serde_json::to_string(&Qid(42)).unwrap(), "42");
    assert_eq!(serde_json::to_string(&Fid(Lid(3), 11)).unwrap(), "[3,11]");
    assert_eq!(
        serde_json::to_string(&WikiId::EntityId(Qid(42))).unwrap(),
        r#"{"EntityId":42}"#
    );
}
