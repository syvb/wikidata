use wikidata::*;

#[test]
fn simple_item() {
    let j: serde_json::Value =
        serde_json::from_str(include_str!("../items/Q106975887.json")).unwrap();
    Entity::from_json(j).unwrap();
}

#[test]
fn douglas_adams() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    Entity::from_json(j).unwrap();
}

#[test]
fn universe() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q1.json")).unwrap();
    Entity::from_json(j).unwrap();
}

#[test]
fn word() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q31928.json")).unwrap();
    Entity::from_json(j).unwrap();
}

#[test]
fn mount_everest() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q513.json")).unwrap();
    Entity::from_json(j).unwrap();
}

#[test]
fn portugal() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q45.json")).unwrap();
    Entity::from_json(j).unwrap();
}
