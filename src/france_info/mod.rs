use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(FranceInfo, 945, EndpointScope::World, "https://www.francetvinfo.fr/titres.rss");
