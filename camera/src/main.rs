pub mod camera_bindings;
pub mod client_store;
pub mod compression;
pub mod server;

use camera_bindings::{
    camera_get_stride, camera_init, camera_next_frame, camera_setup, CameraCapture,
};

use rsct::{allocators::basic_allocator::BasicAllocator, reassembler::Reassembler};
use std::sync::Arc;
use tokio::sync::Mutex;

// const CAMERA_WIDTH: u64 = 2328;
// const CAMERA_HEIGHT: u64 = 1748;
const CAMERA_WIDTH: u64 = 1920;
const CAMERA_HEIGHT: u64 = 1080;
const CAMERA_FPS: u64 = 30;

lazy_static::lazy_static! {
    pub static ref ASYNC_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_io()
    .enable_time()
    .build()
    .unwrap();
}

lazy_static::lazy_static! {
    pub static ref SYNC_RUNTIME: rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
}

struct SystemState {
    server: server::CameraServer,
    clients: Mutex<client_store::ClientManager>,
}

#[tokio::main]
async fn main() {
    let camera = unsafe { camera_init() };
    unsafe { camera_setup(camera, CAMERA_WIDTH, CAMERA_HEIGHT, CAMERA_FPS) };
    // let camera_server = Arc::new(server::CameraServer::new().await);
    // let clients = Arc::new(Mutex::new(client_store::ClientManager::new()));
    let system_state = Arc::new(SystemState {
        server: server::CameraServer::new().await,
        clients: Mutex::new(client_store::ClientManager::new()),
    });

    // system_state
    //     .clients
    //     .lock()
    //     .await
    //     .add_client(rsct::client::Client::from_string(
    //         "192.168.86.42:5254".to_string(),
    //     ));

    let image_metadata = compression::ImageData {
        width: CAMERA_WIDTH,
        height: CAMERA_HEIGHT,
        pitch: unsafe { camera_get_stride(camera) } as u64,
    };

    let state_ref = Arc::clone(&system_state);
    // let clients_ingress_ref = Arc::clone(&clients);
    ASYNC_RUNTIME.spawn(async move {
        let mut reassembler = Reassembler::<BasicAllocator>::new(BasicAllocator);
        loop {
            let (client, msg_type) = state_ref.server.listen(&mut reassembler).await;
            println!("GOT");
            let message = msg_type[0];
            match (client, message) {
                (Some(c), 0) => state_ref.clients.lock().await.add_client(c),
                _ => continue,
            };
        }
    });

    loop {
        let state_ref = Arc::clone(&system_state);

        let res = CameraCapture::new(camera).await;

        SYNC_RUNTIME.spawn(move || {
            let mut compresser = compression::JPEGCompressor::new();
            let buffer = Arc::new(compresser.compress(&res.data, image_metadata).unwrap());

            unsafe { camera_next_frame(camera, res.request) };
            

            ASYNC_RUNTIME.spawn(async move {
                for (_, client_store) in state_ref.clients.lock().await.clients.iter() {
                    let state_ref_ref = Arc::clone(&state_ref);
                    let client_ref = Arc::clone(&client_store.client);
                    let buffer_ref = Arc::clone(&buffer);
                    ASYNC_RUNTIME.spawn(async move {
                        state_ref_ref.server.send(&buffer_ref, &client_ref).await;

                    });
                }
            });
        });
    }
}
