use crate::{
    models::Article,
    parse::{approach::ParseApproach, rule::Rule, section::ParseSection},
    source::endpoint::{Endpoint, EndpointScope},
};

mod config;
mod helpers;
mod json_feed;

pub struct Reuters;

impl crate::source::Source for Reuters {
    fn endpoints() -> Vec<Endpoint> {
        vec![
            endpoint(EndpointScope::World, config::WORLD_PATH),
            endpoint(EndpointScope::US, config::US_PATH),
            endpoint(EndpointScope::Politics, config::POLITICS_PATH),
            endpoint(EndpointScope::Business, config::BUSINESS_PATH),
            endpoint(EndpointScope::Technology, config::TECHNOLOGY_PATH),
            endpoint(EndpointScope::Entertainment, config::ENTERTAINMENT_PATH),
            endpoint(EndpointScope::Sports, config::SPORTS_PATH),
            endpoint(EndpointScope::Science, config::SCIENCE_PATH),
            endpoint(EndpointScope::Health, config::HEALTH_PATH),
        ]
    }
}

fn endpoint(scope: EndpointScope, section_path: &str) -> Endpoint {
    Endpoint {
        url: config::section_url(section_path),
        format: crate::parse::Format::JSON,
        rules: common_rules(section_path),
        scope,
    }
}

fn common_rules(section_path: &str) -> Vec<Rule> {
    vec![Rule {
        section: ParseSection::AreaOfInterest,
        approach: ParseApproach::UseJSONParser {
            function: parse_articles,
            headers: config::json_headers(section_path),
            http1_only: true,
        },
    }]
}

fn parse_articles(body: &str) -> Vec<Article> {
    json_feed::parse(body).unwrap_or_default()
}
