//! Tests for the entity/property constants and the unit-suffix table.

use wikidata::*;

/// Every unit in the suffix table, with the suffix it should produce.
///
/// Three units are deliberately missing here because the suffix they currently produce is
/// wrong; see `tests/known-bugs.rs`.
const UNIT_SUFFIXES: &[(Qid, &str)] = &[
    (consts::METRE, " m"),
    (consts::KILOMETRE, " km"),
    (consts::CENTIMETRE, " cm"),
    (consts::MILLIMETRE, " mm"),
    (consts::SQUARE_METRE, " m\u{b2}"),
    (consts::SQUARE_KILOMETRE, " km\u{b2}"),
    (consts::SQUARE_CENTIMETRE, " cm\u{b2}"),
    (consts::SQUARE_MILLIMETRE, " mm\u{b2}"),
    (consts::CUBIC_METRE, " m\u{b3}"),
    (consts::CUBIC_KILOMETRE, " km\u{b3}"),
    (consts::CUBIC_CENTIMETRE, " cm\u{b3}"),
    (consts::CUBIC_MILLIMETRE, " mm\u{b3}"),
    (consts::GRAM, " g"),
    (consts::MILLIGRAM, " mg"),
    (consts::KILOGRAM_PER_CUBIC_METRE, " kg/m\u{b3}"),
    (consts::GRAM_PER_CUBIC_CENTIMETRE, " g/cm\u{b3}"),
    (consts::MILLILITRE_PER_LITRE, " ml/l"),
    (consts::PARTS_PER_MILLION, " ppm"),
    (consts::GRAM_PER_KILOGRAM, " g/kg"),
    (consts::DEGREE_CELSIUS, " \u{b0}C"),
    (consts::DEGREE_FAHRENHEIT, " \u{b0}F"),
    (consts::KILOMETRE_PER_HOUR, " km/h"),
    (consts::ASTRONOMICAL_UNIT, " AU"),
    (consts::DEGREE, "\u{b0}"),
];

#[test]
fn known_units_have_the_right_suffix() {
    for (qid, suffix) in UNIT_SUFFIXES {
        assert_eq!(qid.unit_suffix(), Some(*suffix), "{qid}");
    }
}

#[test]
fn unknown_units_have_no_suffix() {
    // an entity that is not a unit at all
    assert_eq!(consts::HUMAN.unit_suffix(), None);
    assert_eq!(consts::EARTH.unit_suffix(), None);
    // a unit that is simply not in the table
    assert_eq!(consts::PARSEC.unit_suffix(), None);
    assert_eq!(consts::HECTARE.unit_suffix(), None);
    // an ID that does not exist
    assert_eq!(Qid(u64::MAX).unit_suffix(), None);
    assert_eq!(Qid(0).unit_suffix(), None);
}

#[test]
fn unit_suffix_is_usable_in_a_const_context() {
    const METRE_SUFFIX: Option<&str> = consts::METRE.unit_suffix();
    assert_eq!(METRE_SUFFIX, Some(" m"));
}

#[test]
fn suffixes_are_never_empty() {
    for (qid, _) in UNIT_SUFFIXES {
        assert!(!qid.unit_suffix().unwrap().is_empty(), "{qid}");
    }
}

#[test]
fn well_known_entity_ids() {
    assert_eq!(consts::EARTH, Qid(2));
    assert_eq!(consts::HUMAN, Qid(5));
    assert_eq!(consts::METRE, Qid(11573));
    assert_eq!(consts::KILOGRAM, Qid(11570));
    assert_eq!(consts::SECOND, Qid(11574));
    assert_eq!(consts::KELVIN, Qid(11579));
    assert_eq!(consts::LIGHT_YEAR, Qid(531));
    assert_eq!(consts::ASTRONOMICAL_UNIT, Qid(1811));
    assert_eq!(consts::DEGREE, Qid(28390));
    assert_eq!(consts::EURO, Qid(4916));
}

#[test]
fn well_known_property_ids() {
    assert_eq!(consts::INSTANCE_OF, Pid(31));
    assert_eq!(consts::DATE_OF_BIRTH, Pid(569));
    assert_eq!(consts::DATE_OF_DEATH, Pid(570));
    assert_eq!(consts::PLACE_OF_BIRTH, Pid(19));
    assert_eq!(consts::SEX_OR_GENDER, Pid(21));
    assert_eq!(consts::CITIZENSHIP, Pid(27));
    assert_eq!(consts::SPOUSE, Pid(26));
    assert_eq!(consts::EDUCATED_AT, Pid(69));
    assert_eq!(consts::AWARD_RECEIVED, Pid(166));
    assert_eq!(consts::STATED_IN, Pid(248));
    assert_eq!(consts::REFERENCE_URL, Pid(854));
    assert_eq!(consts::HEIGHT, Pid(2048));
    assert_eq!(consts::NOMINATED_FOR, Pid(1411));
    assert_eq!(consts::TITLE, Pid(1476));
}

#[test]
fn constants_match_the_data_they_are_used_against() {
    // the accessors on `Entity` rely on these, so pin them to a real entity
    let adams = common_entity();
    assert_eq!(
        adams.pid_claims(consts::INSTANCE_OF).count(),
        1,
        "P31 should be instance of"
    );
    assert!(adams.pid_claims(consts::DATE_OF_BIRTH).next().is_some());
    assert!(adams.pid_claims(consts::DATE_OF_DEATH).next().is_some());
    assert_eq!(adams.pid_claims(consts::EDUCATED_AT).count(), 2);
}

fn common_entity() -> Entity {
    let json = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    Entity::from_json(json).unwrap()
}
