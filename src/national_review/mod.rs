use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NationalReview;

impl crate::source::Source for NationalReview {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(829)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(829)
    }
    fn logo_url() -> &'static str {
        "https://www.nationalreview.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::Politics, "https://www.nationalreview.com")])
    }
}
