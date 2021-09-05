//! Various ID types used by Wikidata.

use serde::{Deserialize, Serialize};
use std::{fmt, num::ParseIntError, str::FromStr};

pub mod consts;

/// Three main types of IDs entities can have.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WikiId {
    /// A Qid, representing an entity.
    EntityId(Qid),
    /// A Pid, representing a property.
    PropertyId(Pid),
    /// An Lid, representing a lexeme.
    LexemeId(Lid),
}

/// An error parsing an ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdParseError {
    /// The number couldn't be parsed.
    UnparseableNumber(ParseIntError),
    /// The ID had an invalid prefix letter.
    InvalidPrefix,
    /// The ID had too many parts seperated by dashes
    ///
    /// ## Example
    /// ```
    /// use std::str::FromStr;
    /// use wikidata::{Fid, IdParseError};
    /// assert_eq!(Fid::from_str("L3-F2-S6"), Err(IdParseError::TooManyParts));
    /// ```
    TooManyParts,
    /// The ID had too few parts seperated by dashes
    ///
    /// ## Example
    /// ```
    /// use std::str::FromStr;
    /// use wikidata::{Fid, IdParseError};
    /// assert_eq!(Fid::from_str("L3"), Err(IdParseError::TooFewParts));
    /// ```
    TooFewParts,
}

macro_rules! id_def {
    ($name:ident, $full_name:expr, $letter:expr, $khar:expr) => {
        #[derive(
            Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[doc = "A Wikidata"]
        #[doc = $full_name]
        pub struct $name(pub u64);

        impl $name {
            /// Get the URL to access data about the claim on Wikidata.
            #[must_use]
            pub fn json_url(&self) -> String {
                format!(
                    concat!(
                        "https://www.wikidata.org/wiki/Special:EntityData/",
                        $letter,
                        "{}.json"
                    ),
                    self.0
                )
            }
        }
        impl FromStr for $name {
            type Err = IdParseError;

            /// Parse the identifier from a string.
            fn from_str(x: &str) -> Result<Self, Self::Err> {
                if x.chars().next() != Some($khar) {
                    return Err(IdParseError::InvalidPrefix);
                }
                let num_str = &x[1..];
                match num_str.parse() {
                    Ok(num) => Ok(Self(num)),
                    Err(e) => Err(IdParseError::UnparseableNumber(e)),
                }
            }
        }
        impl fmt::Display for $name {
            /// Display the ID as it would be in a URI.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($letter, "{}"), self.0)
            }
        }
    };
}

id_def!(Qid, "entity ID", "Q", 'Q');
id_def!(Pid, "property ID", "P", 'P');
id_def!(Lid, "lexeme ID", "L", 'L');

macro_rules! lexeme_subid_def {
    ($name:ident, $full_name:expr, $letter:expr, $khar:expr) => {
        /// A lexeme ID and associated
        #[doc = $full_name]
        #[derive(
            Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        pub struct $name(pub Lid, pub u16);

        impl fmt::Display for $name {
            /// Display the ID as it would be in a URI.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}-{}{}", self.0, $khar, self.1)
            }
        }

        impl FromStr for $name {
            type Err = IdParseError;

            /// Parse the identifier from a string.
            fn from_str(x: &str) -> Result<Self, Self::Err> {
                if x.chars().next() != Some('L') {
                    return Err(IdParseError::InvalidPrefix);
                }
                let mut parts = x[1..].split('-');
                let lid = parts
                    .next()
                    .ok_or(IdParseError::TooFewParts)?
                    .parse()
                    .map_err(IdParseError::UnparseableNumber)?;
                let part2 = parts.next().ok_or(IdParseError::TooFewParts)?;
                if part2.chars().next() != Some($khar) {
                    return Err(IdParseError::InvalidPrefix);
                }
                let id = part2[1..]
                    .parse()
                    .map_err(IdParseError::UnparseableNumber)?;
                if parts.next().is_some() {
                    Err(IdParseError::TooManyParts)
                } else {
                    Ok(Self(Lid(lid), id))
                }
            }
        }
    };
}

lexeme_subid_def!(Fid, "form ID", "F", 'F');
lexeme_subid_def!(Sid, "sense ID", "S", 'S');

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn json_url() {
        assert_eq!(
            Qid(42).json_url(),
            "https://www.wikidata.org/wiki/Special:EntityData/Q42.json"
        );
        assert_eq!(
            Pid(31).json_url(),
            "https://www.wikidata.org/wiki/Special:EntityData/P31.json"
        );
        assert_eq!(
            Lid(1).json_url(),
            "https://www.wikidata.org/wiki/Special:EntityData/L1.json"
        )
    }

    #[test]
    fn to_string() {
        let entity = Qid(42);
        assert_eq!(format!("{}", entity), "Q42");

        let prop = Pid(6);
        assert_eq!(format!("{}", prop), "P6");

        let lexeme = Lid(2);
        assert_eq!(format!("{}", lexeme), "L2");

        let sense = Sid(Lid(5), 9);
        assert_eq!(format!("{}", sense), "L5-S9");

        let form = Fid(Lid(3), 11);
        assert_eq!(format!("{}", form), "L3-F11");
    }

    #[test]
    fn from_str() {
        assert_eq!(Qid::from_str("Q42").unwrap(), Qid(42));
        assert_eq!(Lid::from_str("L944114").unwrap(), Lid(944114));
        assert_eq!(Pid::from_str("P1341").unwrap(), Pid(1341));
        assert_eq!(Pid::from_str("Q1341"), Err(IdParseError::InvalidPrefix));
        assert_eq!(Pid::from_str("1341"), Err(IdParseError::InvalidPrefix));
        assert!(Qid::from_str("Q").is_err());
        assert_eq!(Sid::from_str("S1341"), Err(IdParseError::InvalidPrefix));
        assert_eq!(Sid::from_str("L1341"), Err(IdParseError::TooFewParts));
        assert_eq!(
            Sid::from_str("L1341-A123"),
            Err(IdParseError::InvalidPrefix)
        );
        assert_eq!(Sid::from_str("L1341-S123").unwrap(), Sid(Lid(1341), 123));
        assert!(Lid::from_str("L1341-S123").is_err());
        assert!(Lid::from_str("L1341-F123").is_err());
    }

    #[test]
    fn unit_suffix() {
        assert_eq!(consts::unit_suffix(consts::METRE).unwrap(), " m");
        assert_eq!(consts::unit_suffix(consts::DEGREE).unwrap(), "°");
    }
}
