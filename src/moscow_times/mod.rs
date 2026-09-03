use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MoscowTimes,
    704,
    EndpointScope::World,
    "https://www.themoscowtimes.com/rss/news"
);
