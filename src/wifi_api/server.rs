use crate::pb;
use std::pin::Pin;
use tonic::{Request, Response, Status};

pub use pb::wifi_service_server::WifiServiceServer;

#[derive(Debug, Clone, Default)]
pub struct WifiService {}

impl WifiService {
    pub fn new() -> Self {
        Default::default()
    }
}

type Stream<T> = Pin<Box<dyn tokio_stream::Stream<Item = Result<T, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl pb::wifi_service_server::WifiService for WifiService {
    async fn get_mac_address(
        &self,
        _request: Request<pb::wifi::EmptyRequest>,
    ) -> Result<Response<pb::wifi::WifiMACResponse>, Status> {
        unimplemented!()
    }
}
