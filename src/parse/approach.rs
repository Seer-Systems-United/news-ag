#[derive(Debug, Clone)]
pub enum ParseApproach {
    UseClass {
        class_name: String,
    },
    UseJSONParser {
        function: fn(body: &str) -> Vec<crate::models::Article>,
        headers: Vec<(String, String)>,
        http1_only: bool,
    },
}
