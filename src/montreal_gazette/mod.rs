use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct MontrealGazette;

impl crate::source::Source for MontrealGazette {
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::World, "https://montrealgazette.com")])
    }
}
