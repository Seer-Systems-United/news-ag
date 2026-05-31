use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TimesOfIndia,
    EndpointScope::World,
    "https://timesofindia.indiatimes.com/rssfeedstopstories.cms"
);
