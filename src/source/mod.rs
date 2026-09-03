pub mod endpoint;
pub(crate) mod feed;

pub trait Source {
    fn id() -> uuid::Uuid;
    fn endpoints() -> Vec<endpoint::Endpoint>;
    fn get_endpoint(scope: endpoint::EndpointScope) -> Option<endpoint::Endpoint> {
        Self::endpoints()
            .into_iter()
            .find(|endpoint| endpoint.scope == scope)
    }
}
