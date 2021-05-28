use serde::{Deserialize, Serialize};

/// A language used in the Wikibase data model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Lang(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub lang: Lang,
}
