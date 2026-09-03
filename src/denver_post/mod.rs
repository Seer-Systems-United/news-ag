use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DenverPost;

impl crate::source::Source for DenverPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(309)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(309)
    }
    fn logo_url() -> &'static str {
        "https://www.pcmag.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.denverpost.com")])
    }
}
