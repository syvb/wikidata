// test parsing various snaks

use wikidata::*;

#[test]
fn id_snak() {
    let j: serde_json::Value =
        serde_json::from_str(include_str!("../items/Q106975887.json")).unwrap();
    let snak = &j["entities"]["Q106975887"]["claims"]["P31"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(5)));

    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q1.json")).unwrap();
    let snak = &j["entities"]["Q1"]["claims"]["P793"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::Item(Qid(323)));
}

#[test]
fn commons_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P18"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        data,
        ClaimValueData::CommonsMedia("Douglas adams portrait cropped.jpg".to_string())
    );
}

#[test]
fn quantity_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P2048"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        data,
        ClaimValueData::Quantity {
            amount: 1.96,
            lower_bound: None,
            upper_bound: None,
            unit: Some(Qid(11573))
        }
    );
}

#[test]
fn external_id_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P213"][0]["mainsnak"];
    println!("{snak:?}");

    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        data,
        ClaimValueData::ExternalID("0000 0000 8045 6315".to_string())
    );
}

#[test]
fn coordinates_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q513.json")).unwrap();
    let snak = &j["entities"]["Q513"]["claims"]["P625"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        data,
        ClaimValueData::GlobeCoordinate {
            lat: 27.988055555556,
            lon: 86.925277777778,
            precision: 0.00027777777777778,
            globe: Qid(2)
        }
    );
}

#[test]
fn mono_text_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P1477"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        data,
        ClaimValueData::MonolingualText(Text {
            text: "Douglas Noël Adams".to_string(),
            lang: Lang("en".to_string())
        })
    );
}

#[test]
fn date_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q42.json")).unwrap();
    let snak = &j["entities"]["Q42"]["claims"]["P569"][0]["mainsnak"];
    println! {"{snak:?}"};
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(
        &format!("{data:?}"),
        "DateTime { date_time: 1952-03-11T00:00:00Z, precision: 11 }",
    );
}

#[test]
fn lexeme_snak() {
    let j: serde_json::Value = serde_json::from_str(include_str!("../items/Q31928.json")).unwrap();
    let snak = &j["entities"]["Q31928"]["claims"]["P6254"][0]["mainsnak"];
    println!("{snak:?}");
    let data = ClaimValueData::parse_snak(snak.clone()).unwrap();
    assert_eq!(data, ClaimValueData::Lexeme(Lid(361)));
}
