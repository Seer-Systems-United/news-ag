use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    ColumbusDispatch,
    EndpointScope::US,
    "https://www.dispatch.com/news-sitemap.xml"
);
