pub mod endpoint;

pub trait Source {
    fn endpoints() -> Vec<endpoint::Endpoint>;
    fn get_endpoint(scope: endpoint::EndpointScope) -> Option<endpoint::Endpoint> {
        Self::endpoints()
            .into_iter()
            .find(|endpoint| endpoint.scope == scope)
    }
}
