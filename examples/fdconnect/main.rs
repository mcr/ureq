use std::net::{SocketAddr};
use std::net::IpAddr;
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;
use ureq::{Error, TlsConnector};

mod mbedtls_connector;

pub fn main() -> Result<(), Error> {
    let connector = Arc::new(mbedtls_connector::MbedTlsConnector::new(mbedtls::ssl::config::AuthMode::None));

    let agent = ureq::builder()
        .tls_connector(connector.clone())
        .timeout_connect(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build();
    let httpbinip = "::2".parse::<IpAddr>().unwrap();
    let httpbinaddr = SocketAddr::new(
        httpbinip, 8443 as u16
    );
    let content = vec![0x41,0x42,0x43];

    /* establish the connection */
    let conn = TcpStream::connect(httpbinaddr).unwrap();
    let connbox = Box::new(conn);

    let https_stream = connector.connect("example.com", connbox)?;

    let request = agent.request(&"POST".to_string(),
                                "http://example.com/.well-known/brski/requestvoucher");

    let response = request.send_bytes1(&content, https_stream, httpbinaddr).unwrap();

    println!("status {}", response.status());

    Ok(())
}
