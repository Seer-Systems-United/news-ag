use crate::parse::{approach::ParseApproach, section::ParseSection};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub section: ParseSection,
    pub approach: ParseApproach,
}
