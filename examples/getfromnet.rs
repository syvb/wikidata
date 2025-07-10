use wikidata::*;

fn main() {
    for i in 1_usize.. {
        let uri = format!("https://www.wikidata.org/wiki/Special:EntityData/Q{i}.json");
        let res = reqwest::blocking::get(uri).unwrap();
        let text = res.text().unwrap();
        if text.contains("<h1>Not Found</h1><p>No entity with ID ") {
            continue;
        }
        let ent = Entity::from_json(serde_json::from_str(&text).unwrap()).unwrap();
        let _ = ent;
        println!("verified Q{i}");
    }
}
