use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(BangkokPostBusiness, 923, EndpointScope::Business, "https://www.bangkokpost.com/rss/data/business.xml");
