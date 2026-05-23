#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtractMethod {
    Text,
    Attribute { name: String },
}
