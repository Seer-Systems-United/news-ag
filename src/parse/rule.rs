use crate::parse::{approach::ParseApproach, section::ParseSection};

pub struct Rule {
    pub section: ParseSection,
    pub approach: ParseApproach,
}
