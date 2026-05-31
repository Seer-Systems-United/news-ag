use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NationalReview,
    EndpointScope::Politics,
    "https://www.nationalreview.com/feed/"
);
