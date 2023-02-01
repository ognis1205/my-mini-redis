use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {}
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {}
}
