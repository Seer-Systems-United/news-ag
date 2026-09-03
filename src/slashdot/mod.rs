use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Slashdot, 929, EndpointScope::Technology, "https://rss.slashdot.org/Slashdot/slashdotMain");
