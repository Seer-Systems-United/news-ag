use crate::parse::{approach::ParseApproach, section::ParseSection};

#[derive(Debug, Clone)]
pub struct Rule {
    pub section: ParseSection,
    pub approach: ParseApproach,
}
