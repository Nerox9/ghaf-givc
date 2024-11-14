use crate::pb;
use givc_client::endpoint::EndpointConfig;
use tonic::transport::Channel;

type Client = pb::wifi_service_client::WifiServiceClient<Channel>;

#[derive(Debug)]
pub struct WifiClient {
    endpoint: EndpointConfig,
}

impl WifiClient {
    async fn connect_to(&self) -> anyhow::Result<WifiGRPCClient> {
        let channel = self.endpoint.connect().await?;
        Ok(WifiGRPCClient::new(channel))
    }

    // New style api, not yet implemented, stub atm to make current code happy
    // FIXME: Still doubt if constructor should be sync or async
    pub fn new(addr: String, port: u16, tls_info: Option<(String, TlsConfig)>) -> Self {
        Self::from_endpoint_address(EndpointAddress::Tcp { addr, port }, tls_info)
    }

    pub fn from_endpoint_address(
        address: EndpointAddress,
        tls_info: Option<(String, TlsConfig)>,
    ) -> Self {
        let (tls_name, tls) = match tls_info {
            Some((name, tls)) => (name, Some(tls)),
            None => (String::from("bogus(no tls)"), None),
        };
        Self {
            endpoint: EndpointConfig {
                transport: TransportConfig { address, tls_name },
                tls,
            },
        }
    }

    pub async fn get_mac_address(&self) -> anyhow::Result<WifiResult> {
        let request = pb::wifi::EmptyRequest {};
        let _response = self.connect_to().await?.GetMACAddress(request).await?;
        let result = WifiResult {
            response: _response,
        };
        Ok(result)
    }
}