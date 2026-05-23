use crate::{
    parse::Format,
    source::endpoint::{Endpoint, EndpointScope},
};

pub(crate) fn rss_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| rss_endpoint(scope.clone(), url))
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
