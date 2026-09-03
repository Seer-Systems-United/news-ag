use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    ColumbusDispatch,
    322,
    EndpointScope::US,
    "https://www.dispatch.com/news-sitemap.xml"
);
