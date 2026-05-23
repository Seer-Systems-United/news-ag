#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseApproach {
    UseClass { class_name: String },
}
