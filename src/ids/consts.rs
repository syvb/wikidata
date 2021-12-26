//! Various IDs for commonly used entities/properties on Wikidata.

#![allow(clippy::unreadable_literal)]

use super::*;

macro_rules! qid_consts (
    { $($key:ident => $value:expr),+, } => {
        $(
            #[doc = concat!("Item [Q", $value, "](https://www.wikidata.org/wiki/Q", $value, ") on Wikidata")]
            pub const $key: crate::ids::Qid = crate::ids::Qid($value);
        )+
    };
);
macro_rules! pid_consts (
    { $($key:ident => $value:expr),+, } => {
        $(
            #[doc = concat!("Property [P", $value, "](https://www.wikidata.org/wiki/Property:P", $value, ") on Wikidata")]
            pub const $key: crate::ids::Pid = crate::ids::Pid($value);
        )+
    };
);

macro_rules! qid_unit_suffixes {
    { $($key:ident => $value:expr),+, } => {
        #[must_use]
        use super::*;
        pub(crate) const fn unit_suffix(qid: Qid) -> Option<&'static str> {
            $(
                if qid.0 == ($key).0 {
                    Some($value)
                } else
            )+
            {
                None
            }
        }
    };
}

impl Qid {
    /// If the Qid is a commonly used unit on Wikidata, get it as a unit suffix.
    #[must_use]
    pub const fn unit_suffix(self) -> Option<&'static str> {
        consts::unit_suffix(self)
    }
}

mod qid;
pub use qid::*;

mod qid_unit_suffixes;
pub(super) use qid_unit_suffixes::*;

mod pid;
pub use pid::*;
