use rsct::{client::Client, server::Server};

pub struct CameraServer {
    pub server: Server,
}

impl CameraServer {
    pub async fn new() -> CameraServer {
        CameraServer {
            server: Server::new("0.0.0.0", "5253").await,
        }
    }

    pub async fn send(&self, data: &[u8], to: &Client) {
        self.server.transmit(data, to).await;
    }
}
