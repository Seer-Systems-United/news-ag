use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FinancialTimes,
    103,
    EndpointScope::Business,
    "https://www.ft.com/rss/home"
);
