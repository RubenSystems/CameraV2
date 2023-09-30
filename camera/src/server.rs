use rsct::allocators::basic_allocator::BasicAllocator;
use rsct::{client::Client, server::Server};
pub struct CameraServer {
    pub server: Server<BasicAllocator>,
}

impl CameraServer {
    pub async fn new() -> CameraServer {
        CameraServer {
            server: Server::<BasicAllocator>::new("0.0.0.0", "5253", BasicAllocator).await,
        }
    }

    pub async fn listen(&mut self) -> (Option<Client>, Vec<u8>) {
        self.server.recieve_once().await
    }

    pub async fn send(&self, data: &[u8], to: &Client) {
        self.server.transmit(data, to).await;
    }
}
