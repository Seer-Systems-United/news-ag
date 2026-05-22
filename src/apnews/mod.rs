use crate::parse::{approach::ParseApproach, date::DataFormat, extract::ExtractMethod, rule::Rule};

pub struct ApNews;

impl crate::source::Source for ApNews {
    fn endpoints(&self) -> Vec<crate::source::endpoint::Endpoint> {
        vec![crate::source::endpoint::Endpoint {
            url: "https://apnews.com/world-news".parse().unwrap(),
            format: crate::parse::Format::HTML,
            rules: vec![
                Rule {
                    section: crate::parse::section::ParseSection::AreaOfInterest,
                    approach: ParseApproach::UseClass {
                        class_name: "PageList-items-item".to_string(),
                    },
                },
                Rule {
                    section: crate::parse::section::ParseSection::Title {
                        extract_method: ExtractMethod::Text,
                    },
                    approach: ParseApproach::UseClass {
                        class_name: "PagePromoContentIcons-text".to_string(),
                    },
                },
                Rule {
                    section: crate::parse::section::ParseSection::Link {
                        extract_method: ExtractMethod::Attribute {
                            name: "href".to_string(),
                        },
                    },
                    approach: ParseApproach::UseClass {
                        class_name: "Link".to_string(),
                    },
                },
                Rule {
                    section: crate::parse::section::ParseSection::Date {
                        extract_method: ExtractMethod::Attribute {
                            name: "data-posted-date-timestamp".to_string(),
                        },
                        date_format: DataFormat::UnixTimestamp,
                    },
                    approach: ParseApproach::UseClass {
                        class_name: "PagePromo".to_string(),
                    },
                },
            ],
            scope: crate::source::endpoint::EndpointScope::World,
        }]
    }
}
