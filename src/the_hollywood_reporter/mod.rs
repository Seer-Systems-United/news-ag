use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheHollywoodReporter,
    901,
    EndpointScope::Entertainment,
    "https://www.hollywoodreporter.com/feed/"
);
