use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FolhaDeSPaulo,
    705,
    EndpointScope::World,
    "https://feeds.folha.uol.com.br/emcimadahora/rss091.xml"
);
