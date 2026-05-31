use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheHollywoodReporter,
    EndpointScope::Entertainment,
    "https://www.hollywoodreporter.com/feed/"
);
