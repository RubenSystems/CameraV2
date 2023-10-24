use rsct::allocators::basic_allocator::BasicAllocator;
use rsct::{
    client::Client, reassembler::Reassembler, reassembler::ReassemblerResult, server::Server,
};

pub struct CameraServer {
    pub server: Server,
}

impl CameraServer {
    pub async fn new(port_string: &str) -> CameraServer {
        CameraServer {
            server: Server::new("0.0.0.0", port_string).await,
        }
    }

    pub async fn listen(
        &self,
        reassembler: &mut Reassembler<BasicAllocator>,
    ) -> (Option<Client>, Vec<u8>) {
        loop {
            let packet = match self.server.recieve_once().await {
                Ok(dat) => dat,
                _ => continue,
            };

            match reassembler.add(packet) {
                ReassemblerResult::Complete(cli, dat) => return (cli, dat),
                _ => continue,
            }
        }
    }

    pub async fn send(&self, data: &[u8], to: &Client) {
        self.server.transmit(data, to).await;
    }
}
