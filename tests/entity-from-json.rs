//! Tests for [`Entity::from_json`]: the structure it produces from real entities, and the
//! errors it produces from malformed ones.

mod common;

use common::{base_statement, fixture_inner, item_with};
use serde_json::json;
use wikidata::*;

// ---------------------------------------------------------------------------
// Real entities
// ---------------------------------------------------------------------------

#[test]
fn simple_item() {
    let e = common::entity("Q106975887");
    assert_eq!(e.id, WikiId::EntityId(Qid(106_975_887)));
    assert_eq!(e.entity_type, EntityType::Entity);
}

#[test]
fn douglas_adams() {
    let e = common::entity("Q42");
    assert_eq!(e.id, WikiId::EntityId(Qid(42)));
    assert_eq!(e.entity_type, EntityType::Entity);
    assert_eq!(e.labels.len(), 162);
    assert_eq!(e.descriptions.len(), 88);
    assert_eq!(e.aliases.len(), 41);
    assert_eq!(e.sitelinks.len(), 115);
    // one entry per statement, not per property: 259 statements over 227 properties
    assert_eq!(e.claims.len(), 259);

    let en = Lang("en".to_string());
    assert_eq!(e.labels[&en], "Douglas Adams");
    assert_eq!(e.descriptions[&en], "English writer and humorist");
    assert_eq!(
        e.aliases[&en],
        vec![
            "Douglas Noel Adams",
            "Douglas No\u{eb}l Adams",
            "Douglas N. Adams",
        ]
    );
}

#[test]
fn universe() {
    let e = common::entity("Q1");
    assert_eq!(e.id, WikiId::EntityId(Qid(1)));
    assert_eq!(e.claims.len(), 102);
    assert_eq!(e.instances(), vec![Qid(36_906_466)]);
}

#[test]
fn word() {
    let e = common::entity("Q31928");
    assert_eq!(e.id, WikiId::EntityId(Qid(31928)));
    assert_eq!(e.claims.len(), 16);
}

#[test]
fn mount_everest() {
    let e = common::entity("Q513");
    assert_eq!(e.id, WikiId::EntityId(Qid(513)));
    assert_eq!(e.claims.len(), 149);
}

#[test]
fn portugal() {
    let e = common::entity("Q45");
    assert_eq!(e.id, WikiId::EntityId(Qid(45)));
    assert_eq!(e.claims.len(), 540);
    assert_eq!(e.sitelinks.len(), 330);
}

#[test]
fn lexeme() {
    let e = common::entity("L361");
    assert_eq!(e.id, WikiId::LexemeId(Lid(361)));
    assert_eq!(e.entity_type, EntityType::Lexeme);
    assert_eq!(e.claims.len(), 16);
    // lexemes carry `lemmas` rather than `labels`, and their forms and senses are not
    // currently exposed, so the text maps are all empty
    assert!(e.labels.is_empty());
    assert!(e.descriptions.is_empty());
    assert!(e.aliases.is_empty());
    assert!(e.sitelinks.is_empty());
}

#[test]
fn property() {
    let e = common::entity("P6553");
    assert_eq!(e.id, WikiId::PropertyId(Pid(6553)));
    assert_eq!(e.entity_type, EntityType::Property);
    assert_eq!(e.claims.len(), 18);
    assert_eq!(e.labels[&Lang("en".to_string())], "personal pronoun");
    assert!(e.sitelinks.is_empty());
}

#[test]
fn every_fixture_parses_with_its_own_id() {
    for id in common::FIXTURES {
        let expected: WikiId = id.parse().unwrap();
        assert_eq!(common::entity(id).id, expected, "{id}");
    }
}

// ---------------------------------------------------------------------------
// Sitelinks
// ---------------------------------------------------------------------------

#[test]
fn sitelinks_carry_titles_and_badges() {
    let e = common::entity("Q1");
    let bawiki = &e.sitelinks[&SiteName("bawiki".to_string())];
    assert_eq!(bawiki.title, "\u{492}\u{430}\u{43b}\u{4d9}\u{43c}");
    // Q17437798 is "good article"
    assert_eq!(bawiki.badges, vec![Qid(17_437_798)]);

    // most sitelinks have no badges
    let afwiki = &e.sitelinks[&SiteName("afwiki".to_string())];
    assert_eq!(afwiki.title, "Heelal");
    assert!(afwiki.badges.is_empty());
}

#[test]
fn sitelinks_are_optional() {
    let mut item = item_with(base_statement());
    item["sitelinks"] = json!({});
    assert!(Entity::from_json(item).unwrap().sitelinks.is_empty());

    // ...as are all of the text maps
    let e = Entity::from_json(item_with(base_statement())).unwrap();
    assert!(e.sitelinks.is_empty());
    assert!(e.labels.is_empty());
    assert!(e.descriptions.is_empty());
    assert!(e.aliases.is_empty());
}

#[test]
fn sitelinks_without_a_url_parse() {
    // entity dumps omit `url`; only `Special:EntityData` includes it
    let mut item = item_with(base_statement());
    item["sitelinks"] = json!({
        "enwiki": { "site": "enwiki", "title": "Thing", "badges": [] },
    });
    let e = Entity::from_json(item).unwrap();
    let link = &e.sitelinks[&SiteName("enwiki".to_string())];
    assert_eq!(link.title, "Thing");
    assert_eq!(link.url, None);
}

#[test]
fn sitelinks_need_a_title_and_badges() {
    let mut item = item_with(base_statement());
    item["sitelinks"] = json!({ "enwiki": { "badges": [] } });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedSiteTitleString)
    );

    let mut item = item_with(base_statement());
    item["sitelinks"] = json!({ "enwiki": { "title": "Thing" } });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedSiteBadgesArray)
    );
}

// ---------------------------------------------------------------------------
// Labels, descriptions, and aliases
// ---------------------------------------------------------------------------

#[test]
fn text_maps_are_keyed_by_language() {
    let mut item = item_with(base_statement());
    item["labels"] = json!({
        "en": { "language": "en", "value": "thing" },
        "fr": { "language": "fr", "value": "chose" },
    });
    item["descriptions"] = json!({ "en": { "language": "en", "value": "a thing" } });
    item["aliases"] = json!({
        "en": [
            { "language": "en", "value": "item" },
            { "language": "en", "value": "object" },
        ],
    });
    let e = Entity::from_json(item).unwrap();
    assert_eq!(e.labels[&Lang("en".to_string())], "thing");
    assert_eq!(e.labels[&Lang("fr".to_string())], "chose");
    assert_eq!(e.descriptions[&Lang("en".to_string())], "a thing");
    assert_eq!(e.aliases[&Lang("en".to_string())], vec!["item", "object"]);
}

#[test]
fn malformed_text_maps_are_rejected() {
    let mut item = item_with(base_statement());
    item["labels"] = json!([]);
    assert_eq!(Entity::from_json(item), Err(EntityError::ExpectedObject));

    let mut item = item_with(base_statement());
    item["labels"] = json!({ "en": { "language": "en" } });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedLangString)
    );

    let mut item = item_with(base_statement());
    item["labels"] = json!({ "en": { "language": "en", "value": 5 } });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedKeyvalTextString)
    );

    let mut item = item_with(base_statement());
    item["aliases"] = json!({ "en": {} });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedAliasArray)
    );
}

// ---------------------------------------------------------------------------
// The `entities` wrapper
// ---------------------------------------------------------------------------

#[test]
fn bare_entity_objects_are_accepted() {
    let bare = fixture_inner("Q42");
    let from_bare = Entity::from_json(bare).unwrap();
    assert_eq!(from_bare, common::entity("Q42"));
}

#[test]
fn wrapper_must_hold_exactly_one_entity() {
    assert_eq!(
        Entity::from_json(json!({ "entities": {} })),
        Err(EntityError::NoEntities)
    );
    assert_eq!(
        Entity::from_json(json!({ "entities": { "Q1": {}, "Q2": {} } })),
        Err(EntityError::MultipleEntities)
    );
    assert_eq!(
        Entity::from_json(json!({ "entities": [] })),
        Err(EntityError::ExpectedObject)
    );
}

// ---------------------------------------------------------------------------
// Required fields
// ---------------------------------------------------------------------------

#[test]
fn entity_needs_an_id() {
    let mut item = item_with(base_statement());
    item.as_object_mut().unwrap().remove("id");
    assert_eq!(Entity::from_json(item), Err(EntityError::ExpectedObject));

    let mut item = item_with(base_statement());
    item["id"] = json!(1);
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedKeyvalTextString)
    );

    let mut item = item_with(base_statement());
    item["id"] = json!("X1");
    assert_eq!(Entity::from_json(item), Err(EntityError::NoId));
}

#[test]
fn entity_needs_a_known_type() {
    let mut item = item_with(base_statement());
    item.as_object_mut().unwrap().remove("type");
    assert_eq!(Entity::from_json(item), Err(EntityError::NoEntityType));

    let mut item = item_with(base_statement());
    item["type"] = json!("form");
    assert_eq!(Entity::from_json(item), Err(EntityError::NoEntityType));
}

#[test]
fn entity_type_is_taken_from_the_type_field_not_the_id() {
    // the crate does not cross-check the two; whichever the JSON says wins
    let mut item = item_with(base_statement());
    item["type"] = json!("property");
    let e = Entity::from_json(item).unwrap();
    assert_eq!(e.id, WikiId::EntityId(Qid(1)));
    assert_eq!(e.entity_type, EntityType::Property);
}

#[test]
fn entity_needs_claims() {
    let mut item = item_with(base_statement());
    item.as_object_mut().unwrap().remove("claims");
    assert_eq!(Entity::from_json(item), Err(EntityError::NoClaims));

    let mut item = item_with(base_statement());
    item["claims"] = json!([]);
    assert_eq!(Entity::from_json(item), Err(EntityError::ExpectedObject));

    // an empty claim map is fine
    let mut item = item_with(base_statement());
    item["claims"] = json!({});
    assert!(Entity::from_json(item).unwrap().claims.is_empty());
}

#[test]
fn claims_must_be_keyed_by_a_pid() {
    let mut item = item_with(base_statement());
    item["claims"] = json!({ "Q31": [base_statement()] });
    assert_eq!(Entity::from_json(item), Err(EntityError::BadId));

    let mut item = item_with(base_statement());
    item["claims"] = json!({ "P31": base_statement() });
    assert_eq!(
        Entity::from_json(item),
        Err(EntityError::ExpectedClaimArray)
    );
}

// ---------------------------------------------------------------------------
// Statements
// ---------------------------------------------------------------------------

#[test]
fn statement_fields_are_required() {
    let cases: &[(&str, EntityError)] = &[
        ("id", EntityError::NoClaimId),
        ("rank", EntityError::NoRank),
        ("mainsnak", EntityError::MissingMainsnak),
    ];
    for (field, expected) in cases {
        let mut statement = base_statement();
        statement.as_object_mut().unwrap().remove(*field);
        assert_eq!(
            Entity::from_json(item_with(statement)),
            Err(expected.clone()),
            "removing {field}"
        );
    }
}

#[test]
fn every_rank_is_recognized() {
    for (raw, expected) in [
        ("normal", Rank::Normal),
        ("preferred", Rank::Preferred),
        ("deprecated", Rank::Deprecated),
    ] {
        let mut statement = base_statement();
        statement["rank"] = json!(raw);
        let e = Entity::from_json(item_with(statement)).unwrap();
        assert_eq!(e.claims[0].1.rank, expected);
    }

    let mut statement = base_statement();
    statement["rank"] = json!("excellent");
    assert_eq!(
        Entity::from_json(item_with(statement)),
        Err(EntityError::UnknownRank)
    );
}

#[test]
fn deprecated_statements_are_kept() {
    // `Entity::from_json` preserves deprecated statements; it is up to the caller to skip them
    let e = common::entity("Q42");
    let (pid, claim) = e
        .claim_by_id("Q42$bf7e1294-4f0f-3511-ab5f-81f47f5c98cb")
        .expect("Q42 has a deprecated statement");
    assert_eq!(pid, Pid(2021));
    assert_eq!(claim.rank, Rank::Deprecated);

    let preferred = e
        .claim_by_id("Q42$BE724F6B-6981-4DE9-B90C-338768A4BFC4")
        .unwrap()
        .1;
    assert_eq!(preferred.rank, Rank::Preferred);
}

// ---------------------------------------------------------------------------
// Qualifiers
// ---------------------------------------------------------------------------

#[test]
fn qualifiers_follow_the_qualifier_order() {
    let mut statement = base_statement();
    statement["qualifiers-order"] = json!(["P585", "P1686"]);
    statement["qualifiers"] = json!({
        "P1686": [{ "snaktype": "value", "datatype": "string", "property": "P1686",
                    "datavalue": { "type": "string", "value": "a work" } }],
        "P585": [{ "snaktype": "novalue", "datatype": "time", "property": "P585" }],
    });
    let e = Entity::from_json(item_with(statement)).unwrap();
    let qualifiers = &e.claims[0].1.qualifiers;
    assert_eq!(
        qualifiers,
        &vec![
            (Pid(585), ClaimValueData::NoValue),
            (Pid(1686), ClaimValueData::String("a work".to_string())),
        ]
    );
}

#[test]
fn a_qualifier_order_without_qualifiers_is_rejected() {
    let mut statement = base_statement();
    statement["qualifiers-order"] = json!(["P585"]);
    assert_eq!(
        Entity::from_json(item_with(statement)),
        Err(EntityError::QualifiersOrderButNoObject)
    );

    let mut statement = base_statement();
    statement["qualifiers-order"] = json!(["P585"]);
    statement["qualifiers"] = json!({});
    assert_eq!(
        Entity::from_json(item_with(statement)),
        Err(EntityError::QualiferOrderNamesNonQualifier)
    );
}

#[test]
fn statements_without_a_qualifier_order_have_no_qualifiers() {
    let mut statement = base_statement();
    statement["qualifiers"] = json!({
        "P585": [{ "snaktype": "novalue", "datatype": "time", "property": "P585" }],
    });
    let e = Entity::from_json(item_with(statement)).unwrap();
    assert!(e.claims[0].1.qualifiers.is_empty());
}

#[test]
fn real_qualifiers_are_parsed() {
    let e = common::entity("Q42");
    let claim = e
        .claim_by_id("Q42$14ec162d-4a7c-3515-19ad-32b0e14fbb44")
        .unwrap()
        .1;
    // P585 (point in time) once, then P2096 (media legend) five times
    assert_eq!(claim.qualifiers.len(), 6);
    assert_eq!(claim.qualifiers[0].0, Pid(585));
    assert_eq!(claim.qualifier_pid_claims(Pid(2096)).count(), 5);
}

// ---------------------------------------------------------------------------
// References
// ---------------------------------------------------------------------------

#[test]
fn references_follow_the_snak_order() {
    let mut statement = base_statement();
    statement["references"] = json!([{
        "hash": "deadbeef",
        "snaks-order": ["P248"],
        "snaks": {
            "P248": [{ "snaktype": "value", "datatype": "wikibase-item", "property": "P248",
                       "datavalue": { "type": "wikibase-entityid", "value": { "id": "Q5" } } }],
        },
    }]);
    let e = Entity::from_json(item_with(statement)).unwrap();
    let references = &e.claims[0].1.references;
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].hash, "deadbeef");
    assert_eq!(
        references[0].claims,
        vec![(consts::STATED_IN, ClaimValueData::Item(Qid(5)))]
    );
}

#[test]
fn malformed_references_are_rejected() {
    let reference = json!({
        "hash": "deadbeef",
        "snaks-order": ["P248"],
        "snaks": {
            "P248": [{ "snaktype": "novalue", "datatype": "wikibase-item", "property": "P248" }],
        },
    });
    let cases: &[(&str, EntityError)] = &[
        ("hash", EntityError::NoHash),
        ("snaks-order", EntityError::NoSnakOrder),
        ("snaks", EntityError::NoReferenceSnaks),
    ];
    for (field, expected) in cases {
        let mut reference = reference.clone();
        reference.as_object_mut().unwrap().remove(*field);
        let mut statement = base_statement();
        statement["references"] = json!([reference]);
        assert_eq!(
            Entity::from_json(item_with(statement)),
            Err(expected.clone()),
            "removing {field}"
        );
    }

    // a snaks-order entry with no matching snak
    let mut reference = reference.clone();
    reference["snaks-order"] = json!(["P248", "P854"]);
    let mut statement = base_statement();
    statement["references"] = json!([reference]);
    assert_eq!(
        Entity::from_json(item_with(statement)),
        Err(EntityError::SnaksOrderIncludesNonSnak)
    );
}

#[test]
fn real_references_are_parsed() {
    let e = common::entity("Q42");
    let claim = e
        .claim_by_id("q42$881F40DC-0AFE-4FEB-B882-79600D234273")
        .unwrap()
        .1;
    assert_eq!(claim.references.len(), 2);
    let group = &claim.references[0];
    assert_eq!(group.hash, "e4f9e55d169fadcbf86b00425f1cce94ce788679");
    assert_eq!(group.claims.len(), 7);
    assert_eq!(
        group.pid_claims(consts::REFERENCE_URL).next(),
        Some(&ClaimValueData::Url(
            "http://highgatecemetery.org/visit/who".to_string()
        ))
    );
}

#[test]
fn statements_without_references_have_none() {
    let e = Entity::from_json(item_with(base_statement())).unwrap();
    assert!(e.claims[0].1.references.is_empty());
}
