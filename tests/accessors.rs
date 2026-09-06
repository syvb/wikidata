//! Tests for the convenience accessors on [`Entity`], [`ClaimValue`], and [`ReferenceGroup`],
//! plus [`ClaimValue::get_prop_from_snak`].

mod common;

use common::{base_statement, item_with, statement_of};
use serde_json::json;
use wikidata::*;

// ---------------------------------------------------------------------------
// Entity::instances
// ---------------------------------------------------------------------------

#[test]
fn instances_lists_every_instance_of_value() {
    assert_eq!(common::entity("Q42").instances(), vec![consts::HUMAN]);
    assert_eq!(
        common::entity("Q45").instances(),
        vec![Qid(3_624_078), Qid(6256), Qid(20_181_813)]
    );
    assert_eq!(
        common::entity("Q513").instances(),
        vec![Qid(8502), Qid(570_116)]
    );
}

#[test]
fn instances_is_empty_without_instance_of() {
    assert!(common::entity("L361").instances().is_empty());
}

#[test]
fn instances_ignores_non_item_values() {
    let mut statement = base_statement();
    statement["mainsnak"] = json!({
        "snaktype": "somevalue", "property": "P31", "datatype": "wikibase-item",
    });
    let e = Entity::from_json(item_with(statement)).unwrap();
    assert_eq!(e.claims.len(), 1);
    assert!(e.instances().is_empty());
}

#[test]
fn instances_ignores_other_properties() {
    let mut item = item_with(base_statement());
    item["claims"] = json!({
        "P279": [{
            "id": "Q1$a", "rank": "normal",
            "mainsnak": { "snaktype": "value", "property": "P279", "datatype": "wikibase-item",
                          "datavalue": { "type": "wikibase-entityid", "value": { "id": "Q5" } } },
        }],
    });
    assert!(Entity::from_json(item).unwrap().instances().is_empty());
}

// ---------------------------------------------------------------------------
// Entity::start_time / end_time
// ---------------------------------------------------------------------------

#[test]
fn start_and_end_time_come_from_birth_and_death_dates() {
    let e = common::entity("Q42");
    assert_eq!(
        e.start_time().map(|t| t.to_string()),
        Some("1952-03-11 00:00:00 UTC".to_string())
    );
    assert_eq!(
        e.end_time().map(|t| t.to_string()),
        Some("2001-05-11 00:00:00 UTC".to_string())
    );
}

#[test]
fn start_and_end_time_are_none_without_those_properties() {
    // only P569/P570 are consulted: Q1 has a P580 (start time) but no date of birth
    let e = common::entity("Q1");
    assert!(e.pid_claims(Pid(580)).next().is_some());
    assert_eq!(e.start_time(), None);
    assert_eq!(e.end_time(), None);

    let e = common::entity("Q45");
    assert_eq!(e.start_time(), None);
    assert_eq!(e.end_time(), None);
}

#[test]
fn start_time_ignores_non_datetime_values() {
    let mut statement = base_statement();
    statement["mainsnak"] = json!({
        "snaktype": "somevalue", "property": "P569", "datatype": "time",
    });
    let mut item = item_with(statement);
    let claims = item["claims"].as_object_mut().unwrap();
    let statement = claims.remove("P31").unwrap();
    claims.insert("P569".to_string(), statement);
    let e = Entity::from_json(item).unwrap();
    assert_eq!(e.claims.len(), 1);
    assert_eq!(e.start_time(), None);
}

// ---------------------------------------------------------------------------
// Entity::pid_claims
// ---------------------------------------------------------------------------

#[test]
fn pid_claims_yields_every_statement_for_a_property() {
    let e = common::entity("Q42");
    let educated_at: Vec<_> = e.pid_claims(consts::EDUCATED_AT).collect();
    assert_eq!(educated_at.len(), 2);
    let values: Vec<_> = educated_at.iter().map(|c| c.data.clone()).collect();
    assert_eq!(
        values,
        vec![
            ClaimValueData::Item(Qid(691_283)),
            ClaimValueData::Item(Qid(4_961_791)),
        ]
    );
}

#[test]
fn pid_claims_is_empty_for_an_absent_property() {
    let e = common::entity("Q42");
    assert_eq!(e.pid_claims(Pid(999_999_999)).count(), 0);
}

#[test]
fn pid_claims_covers_every_claim_exactly_once() {
    let e = common::entity("Q42");
    let mut pids: Vec<Pid> = e.claims.iter().map(|(pid, _)| *pid).collect();
    pids.sort_unstable();
    pids.dedup();
    let total: usize = pids.iter().map(|pid| e.pid_claims(*pid).count()).sum();
    assert_eq!(total, e.claims.len());
}

// ---------------------------------------------------------------------------
// Entity::claim_by_id
// ---------------------------------------------------------------------------

#[test]
fn claim_by_id_finds_a_statement_and_its_property() {
    let e = common::entity("Q42");
    let (pid, claim) = e
        .claim_by_id("Q42$285E0C13-9674-4131-9556-51B316A57AEE")
        .unwrap();
    assert_eq!(pid, consts::NOMINATED_FOR);
    assert_eq!(claim.rank, Rank::Normal);
    assert_eq!(claim.id, "Q42$285E0C13-9674-4131-9556-51B316A57AEE");
}

#[test]
fn claim_by_id_is_case_sensitive() {
    let e = common::entity("Q42");
    // this statement really does have a lowercase "q42" prefix on Wikidata
    assert!(
        e.claim_by_id("q42$881F40DC-0AFE-4FEB-B882-79600D234273")
            .is_some()
    );
    assert!(
        e.claim_by_id("Q42$881F40DC-0AFE-4FEB-B882-79600D234273")
            .is_none()
    );
}

#[test]
fn claim_by_id_returns_none_for_an_unknown_id() {
    let e = common::entity("Q42");
    assert!(e.claim_by_id("").is_none());
    assert!(e.claim_by_id("Q42$not-a-real-statement").is_none());
}

// ---------------------------------------------------------------------------
// ClaimValue::qualifier_pid_claims and ReferenceGroup::pid_claims
// ---------------------------------------------------------------------------

#[test]
fn qualifier_pid_claims_filters_by_property() {
    let e = common::entity("Q42");
    let claim = e
        .claim_by_id("Q42$14ec162d-4a7c-3515-19ad-32b0e14fbb44")
        .unwrap()
        .1;
    assert_eq!(claim.qualifier_pid_claims(Pid(2096)).count(), 5);
    assert_eq!(claim.qualifier_pid_claims(Pid(585)).count(), 1);
    assert_eq!(claim.qualifier_pid_claims(Pid(999_999)).count(), 0);
}

#[test]
fn reference_group_pid_claims_filters_by_property() {
    let e = common::entity("Q42");
    let group = &e
        .claim_by_id("q42$881F40DC-0AFE-4FEB-B882-79600D234273")
        .unwrap()
        .1
        .references[0];
    let mut urls = group.pid_claims(consts::REFERENCE_URL);
    assert_eq!(
        urls.next(),
        Some(&ClaimValueData::Url(
            "http://highgatecemetery.org/visit/who".to_string()
        ))
    );
    assert_eq!(urls.next(), None);
    assert_eq!(group.pid_claims(Pid(999_999)).count(), 0);
}

// ---------------------------------------------------------------------------
// ClaimValue::get_prop_from_snak
// ---------------------------------------------------------------------------

#[test]
fn get_prop_from_snak_parses_a_statement() {
    // P1411 statement 2: nominated for, with two qualifiers and no references
    let claim = ClaimValue::get_prop_from_snak(statement_of("Q42", "P1411"), false).unwrap();
    assert_eq!(claim.id, "Q42$1B3C484C-643E-45D0-B01C-F6DAD3D1F88E");
    assert_eq!(claim.rank, Rank::Normal);
    assert_eq!(claim.data, ClaimValueData::Item(Qid(3_414_212)));
    assert_eq!(claim.qualifiers.len(), 2);
    assert!(claim.references.is_empty());
}

#[test]
fn get_prop_from_snak_can_skip_the_id() {
    let claim = ClaimValue::get_prop_from_snak(statement_of("Q42", "P1411"), true).unwrap();
    assert_eq!(claim.id, "");

    // with `skip_id` set, a statement without an ID is still usable
    let mut statement = base_statement();
    statement.as_object_mut().unwrap().remove("id");
    assert!(ClaimValue::get_prop_from_snak(statement.clone(), true).is_some());
    assert!(ClaimValue::get_prop_from_snak(statement, false).is_none());
}

#[test]
fn get_prop_from_snak_drops_deprecated_statements() {
    let statement = common::fixture("Q42")["entities"]["Q42"]["claims"]["P2021"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["rank"] == "deprecated")
        .unwrap()
        .clone();
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);
}

#[test]
fn get_prop_from_snak_keeps_preferred_statements() {
    let mut statement = base_statement();
    statement["rank"] = json!("preferred");
    let claim = ClaimValue::get_prop_from_snak(statement, false).unwrap();
    assert_eq!(claim.rank, Rank::Preferred);
}

#[test]
fn get_prop_from_snak_rejects_bad_statements() {
    // an unknown or missing rank
    let mut statement = base_statement();
    statement["rank"] = json!("excellent");
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);

    let mut statement = base_statement();
    statement.as_object_mut().unwrap().remove("rank");
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);

    // an unparseable mainsnak
    let mut statement = base_statement();
    statement["mainsnak"] = json!({ "snaktype": "bogus", "datatype": "string" });
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);

    let mut statement = base_statement();
    statement.as_object_mut().unwrap().remove("mainsnak");
    assert_eq!(ClaimValue::get_prop_from_snak(statement, false), None);
}

#[test]
fn get_prop_from_snak_reads_qualifiers() {
    let mut statement = base_statement();
    statement["qualifiers"] = json!({
        "P585": [{ "snaktype": "novalue", "property": "P585", "datatype": "time" }],
        "P1686": [{ "snaktype": "value", "property": "P1686", "datatype": "string",
                    "datavalue": { "type": "string", "value": "a work" } }],
    });
    let claim = ClaimValue::get_prop_from_snak(statement, false).unwrap();
    assert_eq!(claim.qualifiers.len(), 2);
    assert_eq!(
        claim.qualifier_pid_claims(Pid(1686)).next(),
        Some(&ClaimValueData::String("a work".to_string()))
    );
}

#[test]
fn get_prop_from_snak_ignores_unparseable_qualifiers() {
    let mut statement = base_statement();
    statement["qualifiers"] = json!({
        "P585": [{ "snaktype": "bogus", "property": "P585", "datatype": "time" }],
    });
    let claim = ClaimValue::get_prop_from_snak(statement, false).unwrap();
    assert!(claim.qualifiers.is_empty());
}
