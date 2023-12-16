pub mod camera_bindings;
pub mod client_store;
pub mod compression;
pub mod server;
pub mod thread_pool;

use camera_bindings::{camera_init, camera_setup, CameraDimensionConfig, HiResCameraCapture};

use crate::thread_pool::CompressionThreadPool;
use rsct::client::Client;
use std::sync::Arc;

// const HCAMERA_WIDTH: u64 = 2328;
// const HCAMERA_HEIGHT: u64 = 1748;
// const CAMERA_FPS: u64 = 25;
const HCAMERA_WIDTH: u64 = 1920;
const HCAMERA_HEIGHT: u64 = 1080;
const CAMERA_FPS: u64 = 30;

const LFRAME_DIV_FACT: u64 = 10;
const LCAMERA_WIDTH: u64 = HCAMERA_WIDTH / LFRAME_DIV_FACT;
const LCAMERA_HEIGHT: u64 = HCAMERA_HEIGHT / LFRAME_DIV_FACT;

const IMAGE_METADATA: compression::ImageData = compression::ImageData {
    width: HCAMERA_WIDTH,
    height: HCAMERA_HEIGHT,
};

lazy_static::lazy_static! {
    pub static ref ASYNC_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_io()
    .enable_time()
    .build()
    .unwrap();
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
                15,
                LCAMERA_HEIGHT,
                LCAMERA_WIDTH,
                0,
                CAMERA_FPS,
            ),
        )
    };

    let state = Arc::new(server::CameraServer::new("5254").await);

    let l_state_ref = Arc::clone(&state);
    let sync_runtime: CompressionThreadPool =
        CompressionThreadPool::new(IMAGE_METADATA, l_state_ref, camera, |size, data, srv| {
            ASYNC_RUNTIME.spawn(async move {
                srv.send(
                    &data[..(size as usize)],
                    &Client::from_string("192.168.86.250:5000".to_string()),
                )
                .await;
            });
        });

    loop {
        let res = HiResCameraCapture::new(camera).await;
        sync_runtime.dispatch(res);
    }
}
