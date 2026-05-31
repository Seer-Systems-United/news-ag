use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ElPais,
    EndpointScope::World,
    "https://feeds.elpais.com/mrss-s/pages/ep/site/english.elpais.com/portada"
);
