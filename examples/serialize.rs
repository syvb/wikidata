use std::{env, fs};
use wikidata::*;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let contents = fs::read_to_string(args.next().unwrap()).unwrap();
    let ent = Entity::from_json(serde_json::from_str(&contents).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&ent).unwrap());
}
