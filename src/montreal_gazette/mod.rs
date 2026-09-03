use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct MontrealGazette;

impl crate::source::Source for MontrealGazette {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(406)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(406)
    }
    fn logo_url() -> &'static str {
        "https://montrealgazette.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::World, "https://montrealgazette.com")])
    }
}
