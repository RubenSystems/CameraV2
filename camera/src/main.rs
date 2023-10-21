pub mod camera_bindings;
pub mod client_store;
pub mod compression;
pub mod server;

use camera_bindings::{
    camera_init, camera_next_frame, camera_setup, CameraDimensionConfig, HiResCameraCapture,
    LoResCameraCapture,
};

use rsct::client::Client;
use std::sync::Arc;
use std::thread;

// const CAMERA_WIDTH: u64 = 2328;
// const CAMERA_HEIGHT: u64 = 1748;
// const CAMERA_FPS: u64 = 25;

const HCAMERA_WIDTH: u64 = 1920;
const HCAMERA_HEIGHT: u64 = 1080;
const LCAMERA_WIDTH: u64 = 1920 / 6;
const LCAMERA_HEIGHT: u64 = 1080 / 6;
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
    client: Client,
}

#[tokio::main]
async fn main() {
    let camera = unsafe { camera_init() };
    unsafe {
        camera_setup(
            camera,
            CameraDimensionConfig::new(
                HCAMERA_HEIGHT,
                HCAMERA_WIDTH,
                8,
                LCAMERA_HEIGHT,
                LCAMERA_WIDTH,
                8,
                CAMERA_FPS,
            ),
        )
    };

    let state = Arc::new(SystemState {
        server: server::CameraServer::new().await,
        client: Client::from_string("192.168.86.250:5254".to_string()),
    });

    let h_image_metadata = compression::ImageData {
        width: HCAMERA_WIDTH,
        height: HCAMERA_HEIGHT,
    };

    let l_image_metadata = compression::ImageData {
        width: LCAMERA_WIDTH,
        height: LCAMERA_HEIGHT,
    };

    loop {
        let res = HiResCameraCapture::new(camera).await;

        let state_ref = Arc::clone(&state);
        SYNC_RUNTIME.spawn(move || {
            let compresser = compression::Compresser::new();
            let (buf_size, buf_data) = compresser.compress_image(l_image_metadata, &res.data);

            unsafe { camera_next_frame(camera, res.request) };
            ASYNC_RUNTIME.spawn(async move {
                state_ref
                    .server
                    .send(&buf_data[..(buf_size as usize)], &state_ref.client)
                    .await;
            });
        });
    }
}
