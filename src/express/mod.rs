use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Express;

impl crate::source::Source for Express {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.express.co.uk/posts/rss/1/news",
        )])
    }
}
