use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FinancialTimes,
    EndpointScope::Business,
    "https://www.ft.com/rss/home"
);
