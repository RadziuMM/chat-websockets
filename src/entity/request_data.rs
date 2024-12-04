use tokio::net::TcpStream;

pub struct RequestData {
    pub(crate) stream: TcpStream,
    pub(crate) buffer: [u8; 1024],
    pub(crate) method: String,
    pub(crate) path: String,
}