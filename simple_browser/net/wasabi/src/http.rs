extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use noli::net::{SocketAddr, TcpStream, lookup_host};
use sb_core::error::Error;
use sb_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(_) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    host
                )));
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network(
                "Failed to find any IP addresses".to_string(),
            ));
        };

        let socket_addr: SocketAddr = (ips[0], port).into();

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ));
            }
        };

        let mut request = self.create_request("GET".to_string(), host, path);

        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(b) => b,
            Err(_) => {
                return Err(Error::Network("Failed to write to TCP stream".to_string()));
            }
        };

        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(b) => b,
                Err(_) => {
                    return Err(Error::Network("Failed to read from TCP stream".to_string()));
                }
            };

            if bytes_read == 0 {
                break;
            }

            received.extend_from_slice(&buf[..bytes_read])
        }

        // HTTPのレスポンスはデフォルトUTF-8
        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(_) => Err(Error::Network("Failed to parse HTTP response".to_string())),
        }
    }

    fn create_request(&self, method: String, host: String, path: String) -> String {
        let mut request = self.create_request_startline(method, path);
        request.push_str(&self.create_request_headers(host));
        request
    }

    fn create_request_startline(&self, method: String, path: String) -> String {
        format!("{} /{} HTTP/1.1\r\n", method, path)
    }

    fn create_request_headers(&self, host: String) -> String {
        let mut headers = String::new();
        headers.push_str("Host: ");
        headers.push_str(&host);
        headers.push('\n');
        headers.push_str("Accept: text/html\n");
        headers.push_str("Connection: close\n");
        headers.push('\n');

        headers
    }
}
