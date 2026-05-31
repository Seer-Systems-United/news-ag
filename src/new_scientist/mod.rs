use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewScientist,
    EndpointScope::Science,
    "https://www.newscientist.com/feed/home/"
);
