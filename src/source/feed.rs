use crate::{
    models::Article,
    parse::{Format, approach::ParseApproach, rule::Rule, section::ParseSection},
    source::endpoint::{Endpoint, EndpointScope},
};

const USER_AGENT: &str = "Mozilla/5.0 (compatible; news-sources/0.1)";

macro_rules! rss_source {
    ($name:ident, $scope:expr, $url:expr) => {
        pub struct $name;

        impl $crate::source::Source for $name {
            fn endpoints() -> Vec<$crate::source::endpoint::Endpoint> {
                $crate::source::feed::rss_endpoints(&[($scope, $url)])
            }
        }
    };
}

macro_rules! news_sitemap_source {
    ($name:ident, $scope:expr, $url:expr) => {
        pub struct $name;

        impl $crate::source::Source for $name {
            fn endpoints() -> Vec<$crate::source::endpoint::Endpoint> {
                $crate::source::feed::news_sitemap_endpoints(&[($scope, $url)])
            }
        }
    };
}

pub(crate) use {news_sitemap_source, rss_source};

pub(crate) fn rss_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| rss_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn news_sitemap_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| news_sitemap_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn wordpress_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| wordpress_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn ssr_json_endpoints(
    feeds: &[(EndpointScope, &str)],
    function: fn(&str) -> Vec<Article>,
) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| ssr_json_endpoint(scope.clone(), url, function))
        .collect()
}

fn rss_endpoint(scope: EndpointScope, url: &str) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::RSS,
        scope,
        rules: Vec::new(),
    }
}

fn news_sitemap_endpoint(scope: EndpointScope, url: &str) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::GoogleNewsSitemap,
        scope,
        rules: Vec::new(),
    }
}

fn wordpress_endpoint(scope: EndpointScope, base_url: &str) -> Endpoint {
    Endpoint {
        url: wordpress_posts_url(base_url),
        format: Format::JSON,
        scope,
        rules: vec![Rule {
            section: ParseSection::AreaOfInterest,
            approach: ParseApproach::UseJSONParser {
                function: crate::parse::json::wordpress::parse_posts,
                headers: vec![
                    ("accept".to_string(), "application/json".to_string()),
                    ("user-agent".to_string(), USER_AGENT.to_string()),
                ],
                http1_only: false,
            },
        }],
    }
}

fn wordpress_posts_url(base_url: &str) -> reqwest::Url {
    let base_url = base_url.trim_end_matches('/');
    format!(
        "{base_url}/wp-json/wp/v2/posts?per_page=20&_fields=date,date_gmt,link,title,yoast_head_json.author"
    )
    .parse()
    .unwrap()
}

fn ssr_json_endpoint(
    scope: EndpointScope,
    url: &str,
    function: fn(&str) -> Vec<Article>,
) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::JSON,
        scope,
        rules: vec![Rule {
            section: ParseSection::AreaOfInterest,
            approach: ParseApproach::UseJSONParser {
                function,
                headers: vec![
                    (
                        "accept".to_string(),
                        "text/html,application/xhtml+xml,application/json".to_string(),
                    ),
                    ("user-agent".to_string(), USER_AGENT.to_string()),
                ],
                http1_only: false,
            },
        }],
    }
}
