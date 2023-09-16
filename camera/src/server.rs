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

    pub async fn listen(&mut self) -> (Option<Client>, Vec<u8>) {
        self.server.recieve_once().await
    }

    pub async fn send(&self, data: &[u8], to: &Client, runtime: &tokio::runtime::Runtime) {
        self.server.transmit_concurrently(data, to, runtime).await;
    }
}
