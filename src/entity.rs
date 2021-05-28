use crate::ids::{consts, Fid, Lid, Pid, Qid, Sid};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// A Wikibase entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub claims: Vec<(Pid, ClaimValue)>,
}

/// Data relating to a claim value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimValueData {
    CommonsMedia(String),
    GlobeCoordinate {
        //supported
        lat: f64,
        lon: f64,
        precision: f64,
        globe: Qid,
    },
    Item(Qid),
    Property(Pid),
    Stringg(String),
    MonolingualText {
        text: String,
        lang: String,
    },
    ExternalID(String),
    Quantity {
        amount: f64, // technically it could exceed the bound, but meh
        lower_bound: Option<f64>,
        upper_bound: Option<f64>,
        unit: Option<Qid>, // *could* be any IRI but in practice almost all are Wikidata entity IRIs
    },
    DateTime {
        date_time: DateTime<chrono::offset::Utc>,
        /// 0 - billion years
        /// 1 - 100 million years
        /// 2 - 10 million years
        /// 3 - 1 million years
        /// 4 - 100k years
        /// 5 - 10k years
        /// 6 - 1000 years
        /// 7 - 100 years
        /// 8 - decade
        /// 9 - year
        /// 10 - month
        /// 11 - day
        /// 12 - hour (deprecated)
        /// 13 - minute (deprecated)
        /// 14 - second (deprecated)
        precision: u8,
    },
    Url(String),
    MathExpr(String),
    GeoShape(String),
    MusicNotation(String),
    TabularData(String),
    Lexeme(Lid),
    Form(Fid),
    Sense(Sid),
    NoValue,
    UnknownValue,
}

impl Default for ClaimValueData {
    fn default() -> Self {
        ClaimValueData::NoValue
    }
}

/// A statement rank.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Deprecated,
    Normal,
    Preferred,
}

impl Default for Rank {
    fn default() -> Self {
        Rank::Normal
    }
}

/// A group of claims that make up a single reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceGroup {
    pub claims: Vec<(Pid, ClaimValueData)>,
}

/// A claim value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClaimValue {
    pub data: ClaimValueData,
    pub rank: Rank,
    pub id: String,
    pub qualifiers: Vec<(Pid, ClaimValueData)>,
    pub references: Vec<ReferenceGroup>,
}

impl Entity {
    pub fn instances(&self) -> Vec<Qid> {
        let mut instances = Vec::with_capacity(1);
        for (pid, claim) in &self.claims {
            if *pid == consts::INSTANCE_OF {
                if let ClaimValueData::Item(qid) = claim.data {
                    instances.push(qid);
                };
            };
        }
        instances.shrink_to_fit();
        instances
    }

    pub fn start_time(&self) -> Option<DateTime<chrono::offset::Utc>> {
        for (pid, claim) in &self.claims {
            if *pid == consts::DATE_OF_BIRTH {
                if let ClaimValueData::DateTime { date_time, .. } = claim.data {
                    return Some(date_time);
                };
            };
        }
        None
    }

    pub fn end_time(&self) -> Option<DateTime<chrono::offset::Utc>> {
        for (pid, claim) in &self.claims {
            if *pid == consts::DATE_OF_DEATH {
                if let ClaimValueData::DateTime { date_time, .. } = claim.data {
                    return Some(date_time);
                };
            };
        }
        None
    }
}

fn get_json_string(mut json: json::JsonValue) -> String {
    match json.take_string() {
        Some(x) => x,
        None => panic!("get_json_stringing called with a non-string JsonValue"),
    }
}

fn parse_wb_number(num: &json::JsonValue) -> Option<f64> {
    // could be a string repersenting a number, or a number
    if num.is_number() {
        Some(num.as_number()?.into())
    } else {
        let s = num.as_str()?;
        match s.parse() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}

fn try_get_as_qid(datavalue: &json::JsonValue) -> Option<Qid> {
    match datavalue
        .as_str()?
        .split("http://www.wikidata.org/entity/Q")
        .nth(1)?
        .parse()
    {
        Ok(x) => Some(Qid(x)),
        Err(_) => None,
    }
}

fn take_prop(key: &'static str, claim: &mut json::JsonValue) -> json::JsonValue {
    claim.remove(key)
}

fn parse_wb_time(time: &str) -> Option<chrono::DateTime<chrono::offset::Utc>> {
    if time.is_empty() {
        return None;
    }

    // "Negative years are allowed in formatting but not in parsing.", so we
    // set the era ourselves, after parsing
    let is_ce = time.chars().next()? == '+';
    let time = &time[1..];

    let time_parts: Vec<&str> = time.split('T').collect();
    let dash_parts: Vec<&str> = time_parts[0].split('-').collect();
    // could be wrong maybe if the percision is more than a year, meh
    let year: i32 = match dash_parts[0].parse() {
        Ok(x) => x,
        Err(_) => return None,
    };
    let year: i32 = year * (if is_ce { 1 } else { -1 });
    let month: Option<u32> = match dash_parts.get(1) {
        Some(month_str) => match month_str.parse() {
            Ok(0) | Err(_) => None,
            Ok(x) => Some(x),
        },
        None => None,
    };
    let day: Option<u32> = match dash_parts.get(2) {
        Some(day_str) => match day_str.parse() {
            Ok(0) | Err(_) => None,
            Ok(x) => Some(x),
        },
        None => None,
    };
    let maybe_date = Utc.ymd_opt(year, month.unwrap_or(1), day.unwrap_or(1));
    let date = match maybe_date {
        chrono::offset::LocalResult::Single(date) => date,
        _ => return None, //  matched zero dates or matched multiple
    };
    let (hour, min, sec) = if time_parts.len() == 2 {
        let colon_parts: Vec<&str> = time_parts[1].split(':').collect();
        let hour = match colon_parts.get(0)?.parse() {
            Ok(x) => x,
            Err(_) => return None,
        };
        let minute = match colon_parts.get(1)?.parse() {
            Ok(x) => x,
            Err(_) => return None,
        };
        let sec = match colon_parts.get(2)?[0..2].parse() {
            Ok(x) => x,
            Err(_) => return None,
        };
        (hour, minute, sec)
    } else {
        (0, 0, 0)
    };
    Some(date.and_hms(hour, min, sec))
}

impl ClaimValueData {
    #[must_use]
    /// Parses a snak, panics on failure.
    pub fn parse_snak(mut snak: json::JsonValue) -> Option<Self> {
        let mut datavalue: json::JsonValue = take_prop("datavalue", &mut snak);
        let datatype: &str = &get_json_string(take_prop("datatype", &mut snak));
        let snaktype: &str = &get_json_string(take_prop("snaktype", &mut snak));
        match snaktype {
            "value" => {}
            "somevalue" => return Some(ClaimValueData::UnknownValue),
            "novalue" => return Some(ClaimValueData::NoValue),
            x => panic!(
                "Expected snaktype to be value, somevalue, or novalue, but it was {}",
                x
            ),
        };
        let type_str = take_prop("type", &mut datavalue)
            .take_string()
            .expect("Invalid datavalue type. Perhaps a new data type has been added?");
        let mut value = take_prop("value", &mut datavalue);
        match &type_str[..] {
            "string" => {
                let s = value
                    .take_string()
                    .expect("expected string, didn't find one");
                match datatype {
                    "string" => Some(ClaimValueData::Stringg(s)),
                    "commonsMedia" => Some(ClaimValueData::CommonsMedia(s)),
                    "external-id" => Some(ClaimValueData::ExternalID(s)),
                    "math" => Some(ClaimValueData::MathExpr(s)),
                    "geo-shape" => Some(ClaimValueData::GeoShape(s)),
                    "musical-notation" => Some(ClaimValueData::MusicNotation(s)),
                    "tabular-data" => Some(ClaimValueData::TabularData(s)),
                    "url" => Some(ClaimValueData::Url(s)),
                    _ => {
                        eprintln!("Invalid datatype {}", datatype);
                        None
                    }
                }
            }
            "wikibase-entityid" => {
                // the ID could be a entity, lexeme, property, form, or sense
                let id = get_json_string(take_prop("id", &mut value));
                match id.chars().next().expect("Entity ID was empty string") {
                    'Q' => Some(ClaimValueData::Item(Qid(id[1..]
                        .parse()
                        .expect("Malformed entity ID")))),
                    'P' => Some(ClaimValueData::Property(Pid(id[1..]
                        .parse()
                        .expect("Malformed property ID")))),
                    'L' => {
                        // sense: "L1-S2", form: "L1-F2", lexeme: "L2"
                        let parts: Vec<&str> = id.split('-').collect();
                        match parts.len() {
                            1 => Some(ClaimValueData::Lexeme(Lid(id[1..]
                                .parse()
                                .expect("Malformed lexeme ID")))),
                            2 => {
                                match parts[1]
                                    .chars()
                                    .next()
                                    .expect("Nothing after dash in lexeme ID")
                                {
                                    'F' => Some(ClaimValueData::Form(Fid(
                                        Lid(parts[0][1..].parse().expect("Malformed lexeme ID")),
                                        parts[1][1..].parse().expect("Invalid form ID"),
                                    ))),
                                    'S' => Some(ClaimValueData::Sense(Sid(
                                        Lid(parts[0][1..].parse().expect("Malformed lexeme ID")),
                                        parts[1][1..].parse().expect("Invalid sense ID"),
                                    ))),
                                    _ => panic!("Invalid second part of lexeme ID"),
                                }
                            }
                            _ => panic!("Lexeme ID had more than 1 dash"),
                        }
                    }
                    _ => panic!("Couldn't parse entity ID"),
                }
            }
            "globecoordinate" => {
                Some(ClaimValueData::GlobeCoordinate {
                    lat: parse_wb_number(&take_prop("latitude", &mut value))?,
                    lon: parse_wb_number(&take_prop("longitude", &mut value))?,
                    // altitude field is deprecated and we ignore it
                    precision: parse_wb_number(&take_prop("precision", &mut value))?,
                    // globe *can* be any IRI, but it practice it's almost always an entity URI
                    // so we return None if it doesn't match our expectations
                    globe: try_get_as_qid(&take_prop("globe", &mut value))?,
                })
            }
            "quantity" => Some(ClaimValueData::Quantity {
                amount: parse_wb_number(&take_prop("amount", &mut value))?,
                upper_bound: parse_wb_number(&take_prop("upperBound", &mut value)),
                lower_bound: parse_wb_number(&take_prop("lowerBound", &mut value)),
                unit: try_get_as_qid(&take_prop("unit", &mut value)),
            }),
            "time" => Some(ClaimValueData::DateTime {
                // our time parsing code can't handle a few edge cases (really old years), so we
                // just give up on parsing the snak if parse_wb_time returns None
                date_time: parse_wb_time(&get_json_string(take_prop("time", &mut value)))?,
                precision: parse_wb_number(&take_prop("precision", &mut value))
                    .expect("Invalid precision {}") as u8,
            }),
            "monolingualtext" => Some(ClaimValueData::MonolingualText {
                text: get_json_string(take_prop("text", &mut value)),
                lang: get_json_string(take_prop("language", &mut value)),
            }),
            other => {
                eprintln!("Couldn't parse data type {}", other);
                None
            }
        }
    }
}

impl ClaimValue {
    #[must_use]
    pub fn get_prop_from_snak(mut claim: json::JsonValue, skip_id: bool) -> Option<ClaimValue> {
        let claim_str = take_prop("rank", &mut claim)
            .take_string()
            .expect("No rank");
        let rank = match &claim_str[..] {
            "deprecated" => {
                return None;
            }
            "normal" => Rank::Normal,
            "preferred" => Rank::Preferred,
            other => panic!("Invalid rank {}", other),
        };
        let mainsnak = take_prop("mainsnak", &mut claim);
        let data = match ClaimValueData::parse_snak(mainsnak) {
            Some(x) => x,
            None => {
                eprintln!("Failed to parse mainsnak");
                return None;
            }
        };
        let references_json = take_prop("references", &mut claim);
        let references = if references_json.is_array() {
            let mut v: Vec<ReferenceGroup> = Vec::with_capacity(references_json.len());
            let mut references_vec = if let json::JsonValue::Array(a) = references_json {
                a
            } else {
                panic!("references not an array");
            };
            for mut reference_group in references_vec.drain(..) {
                let mut claims = Vec::with_capacity(reference_group["snaks"].len());
                let snaks = take_prop("snaks", &mut reference_group);
                let mut entries: Vec<(&str, &json::JsonValue)> = snaks.entries().collect();
                for (pid, snak_group) in entries.drain(..) {
                    let mut members: Vec<&json::JsonValue> = snak_group.members().collect();
                    for snak in members.drain(..) {
                        // clone, meh
                        let owned_snak = snak.clone().take();
                        match ClaimValueData::parse_snak(owned_snak) {
                            Some(x) => claims
                                .push((Pid(pid[1..].parse().expect("Invalid property ID")), x)),
                            None => {
                                eprintln!("Failed to parse reference snak");
                            }
                        }
                    }
                }
                v.push(ReferenceGroup { claims });
            }
            v
        } else {
            vec![]
        };
        let qualifiers_json = take_prop("qualifiers", &mut claim);
        let qualifiers = if qualifiers_json.is_object() {
            let mut v: Vec<(Pid, ClaimValueData)> = vec![];
            let mut entries: Vec<(&str, &json::JsonValue)> = qualifiers_json.entries().collect();
            for (pid, claim_array_json) in entries.drain(..) {
                // yep it's a clone, meh
                let mut claim_array =
                    if let json::JsonValue::Array(x) = claim_array_json.clone().take() {
                        x
                    } else {
                        panic!("qualifiers doesn't have a claim array");
                    };
                for claim in claim_array.drain(..) {
                    match ClaimValueData::parse_snak(claim) {
                        Some(x) => v.push((Pid(pid[1..].parse().expect("Invalid property ID")), x)),
                        None => {
                            eprintln!("Failed to parse qualifier snak");
                        }
                    };
                }
            }
            v
        } else {
            vec![]
        };
        Some(ClaimValue {
            rank,
            id: if skip_id {
                String::new()
            } else {
                take_prop("id", &mut claim)
                    .take_string()
                    .expect("No id on snak")
            },
            data,
            references,
            qualifiers,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_parsing() {
        let valid_times = vec![
            "+2001-12-31T00:00:00Z",
            "+12346-12-31T00:00:00Z",
            "+311-12-31T00:00:00Z",
            "+1979-00-00T00:00:00Z",
            "-1979-00-00T00:00:00Z",
            "+2001-12-31T00:00:00Z",
            "+2001-12-31",
            "+2001-12",
            "-12561",
            "+311-12-31T12:34:56Z",
            "+311-12-31T23:45:42Z",
            // below are times that *should* work, but chrono doesn't accept
            // "-410000000-00-00T00:00:00Z",
        ];
        for time in valid_times {
            println!("Trying \"{}\"", time);
            assert!(match parse_wb_time(time) {
                Some(val) => {
                    println!("Got {:#?}", val);
                    true
                }
                None => false,
            });
        }
    }

    #[test]
    fn as_qid_test() {
        let qid =
            try_get_as_qid(&json::parse(r#""http://www.wikidata.org/entity/Q1234567""#).unwrap());
        assert_eq!(qid, Some(Qid(1234567)));
    }
}
