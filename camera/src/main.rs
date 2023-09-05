pub mod compression;
pub mod camera_bindings;
pub mod client_store;
pub mod server;

use camera_bindings::{camera_init, camera_setup, CameraCapture};
use std::sync::{Arc};

const CAMERA_WIDTH: u64 = 1920;
const CAMERA_HEIGHT: u64 = 1080;
const CAMERA_FPS: u64 = 30;

lazy_static::lazy_static! {
    pub static ref ASYNC_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .build()
    .unwrap();
}


#[tokio::main]
async fn main() {


    let camera = unsafe {camera_init()};
    unsafe {camera_setup(camera, CAMERA_HEIGHT, CAMERA_WIDTH, CAMERA_FPS)};
    let camera_server = server::CameraServer::new().await;
    let mut camera_buffer = vec![0_u8; (CAMERA_WIDTH * CAMERA_HEIGHT * 3) as usize];

    loop {
        let _ = CameraCapture::new (
            camera, 
            &mut camera_buffer,
            CAMERA_WIDTH * CAMERA_HEIGHT * 3
        ).await;

        let client = rsct::client::Client::from_string("0.0.0.0:5253".to_string());
        camera_server.send(&camera_buffer, &client).await;
        println!("HI!");
    }
}
