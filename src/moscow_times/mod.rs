use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MoscowTimes,
    EndpointScope::World,
    "https://www.themoscowtimes.com/rss/news"
);
