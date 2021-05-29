// test parsing various snaks

#[test]
fn simple_snak_from_json() {
    let j = json::parse(include_str!("../items/Q106975887.json")).unwrap();
    let snak = &j["entities"]["Q106975887"]["claims"]["P31"][0]["mainsnak"];
    println!("{:?}", snak);
    wikidata::ClaimValueData::parse_snak(snak.clone()).unwrap();
}


#[test]
fn complex_snak_from_json() {
    let j = json::parse(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P18"][0]["mainsnak"];
    println!("{:?}", snak);
    wikidata::ClaimValueData::parse_snak(snak.clone()).unwrap();
}

#[test]
fn other_complex_snak_from_json() {
    let j = json::parse(include_str!("../items/Q1.json")).unwrap();
    let snak = &j["entities"]["Q1"]["claims"]["P793"][0]["mainsnak"];
    println!("{:?}", snak);
    wikidata::ClaimValueData::parse_snak(snak.clone()).unwrap();
}
