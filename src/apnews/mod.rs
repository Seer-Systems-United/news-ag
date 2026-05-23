use crate::parse::{approach::ParseApproach, date::DataFormat, extract::ExtractMethod, rule::Rule};

pub struct ApNews;

fn common_rules() -> Vec<Rule> {
    vec![
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
    ]
}

impl crate::source::Source for ApNews {
    fn endpoints() -> Vec<crate::source::endpoint::Endpoint> {
        vec![
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/world-news".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::World,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/us-news".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::US,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/politics".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Politics,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/business".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Business,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/technology".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Technology,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/entertainment".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Entertainment,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/sports".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Sports,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/science".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Science,
            },
            crate::source::endpoint::Endpoint {
                url: "https://apnews.com/health".parse().unwrap(),
                format: crate::parse::Format::HTML,
                rules: common_rules(),
                scope: crate::source::endpoint::EndpointScope::Health,
            },
        ]
    }
}
