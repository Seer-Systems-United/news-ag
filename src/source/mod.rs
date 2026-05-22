pub mod endpoint;

pub trait Source {
    fn endpoints(&self) -> Vec<endpoint::Endpoint>;
}
