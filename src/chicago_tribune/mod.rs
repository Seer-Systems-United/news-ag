use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ChicagoTribune;

impl crate::source::Source for ChicagoTribune {
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.chicagotribune.com")])
    }
}
