use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewScientist,
    209,
    EndpointScope::Science,
    "https://www.newscientist.com/feed/home/"
);
