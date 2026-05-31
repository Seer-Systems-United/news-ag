use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NationalReview;

impl crate::source::Source for NationalReview {
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::Politics, "https://www.nationalreview.com")])
    }
}
