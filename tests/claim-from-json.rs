// test parsing various snaks

use wikidata::*;

#[test]
fn simple_snak_from_json() {
    let j: serde_json:: Value = serde_json::from_str(include_str!("../items/Q106975887.json")).unwrap();
    let snak = &j["entities"]["Q106975887"]["claims"]["P31"][0]["mainsnak"];
    println!("{:?}", snak);
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(5)));
}


#[test]
fn complex_snak_from_json() {
    let j: serde_json:: Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P18"][0]["mainsnak"];
    println!("{:?}", snak);
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::CommonsMedia("Douglas adams portrait cropped.jpg".to_string()));
}

#[test]
fn other_complex_snak_from_json() {
    let j: serde_json:: Value = serde_json::from_str(include_str!("../items/Q1.json")).unwrap();
    let snak = &j["entities"]["Q1"]["claims"]["P793"][0]["mainsnak"];
    println!("{:?}", snak);
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(323)));
}
