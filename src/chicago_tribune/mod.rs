use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ChicagoTribune;

impl crate::source::Source for ChicagoTribune {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(302)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(302)
    }
    fn logo_url() -> &'static str {
        "https://www.theverge.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.chicagotribune.com")])
    }
}
