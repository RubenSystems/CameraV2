pub mod camera_bindings;
pub mod client_store;
pub mod compression;
pub mod server;

use camera_bindings::{camera_get_stride, camera_init, camera_setup, CameraCapture};

use std::sync::Arc;

// const CAMERA_WIDTH: u64 = 2328;
// const CAMERA_HEIGHT: u64 = 1748;
const CAMERA_WIDTH: u64 = 1920;
const CAMERA_HEIGHT: u64 = 1080;
const CAMERA_FPS: u64 = 30;

lazy_static::lazy_static! {
    pub static ref ASYNC_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_io()
    .build()
    .unwrap();
}

lazy_static::lazy_static! {
    pub static ref SYNC_RUNTIME: rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
}

#[tokio::main]
async fn main() {
    let camera = unsafe { camera_init() };
    unsafe { camera_setup(camera, CAMERA_WIDTH, CAMERA_HEIGHT, CAMERA_FPS) };
    let camera_server = Arc::new(server::CameraServer::new().await);
    let mut frame_id: u8 = 0;

    let image_metadata = compression::ImageData {
        width: CAMERA_WIDTH,
        height: CAMERA_HEIGHT,
        pitch: unsafe { camera_get_stride(camera) } as u64,
    };

    loop {
        let mut camera_buffer = vec![0_u8; (CAMERA_WIDTH * CAMERA_HEIGHT * 4) as usize];
        let _ = CameraCapture::new(
            camera,
            frame_id,
            &mut camera_buffer
        )
        .await;

        frame_id = if frame_id >= u8::MAX { frame_id + 1 } else { 0 };

        let client = rsct::client::Client::from_string("192.168.86.67:5254".to_string());

        let server_ref = Arc::clone(&camera_server);

        SYNC_RUNTIME.spawn(move || {
            let mut compresser = compression::JPEGCompressor::new();
            let compressed_data = compresser.compress(&camera_buffer, image_metadata).unwrap();
            ASYNC_RUNTIME.spawn(async move {
                server_ref
                    .send(&compressed_data, &client, &ASYNC_RUNTIME)
                    .await;
            });
        });
    }
}
