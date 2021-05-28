use serde::{Deserialize, Serialize};

/// A language, as used in the Wikibase data model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Lang(pub String);

/// Text that is in a certain language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Text {
    /// The raw text.
    pub text: String,
    /// The language of the text.
    pub lang: Lang,
}
