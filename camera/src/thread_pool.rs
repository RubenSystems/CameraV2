use std::sync::{mpsc, Arc, Mutex};

use core_affinity::CoreId;

use crate::{
    camera_bindings::{camera_next_frame, FrameData, VoidPointer},
    compression::{Compresser, ImageData},
    server::CameraServer,
};

pub struct CompressionThreadPool {
    workers: Vec<CompressionWorker>,
    sender: mpsc::Sender<FrameData>,
}

impl CompressionThreadPool {
    pub fn new(
        data: ImageData,
        server: Arc<CameraServer>,
        camera: VoidPointer,
        on_complete: fn(u64, Vec<u8>, Arc<CameraServer>),
    ) -> Self {
		let core_ids = core_affinity::get_core_ids().unwrap();
        let (sender, receiver) = mpsc::channel();
        let recv_mtx = Arc::new(Mutex::new(receiver));

        let workers = core_ids.into_iter().map(|id| {
            let srv_ref = Arc::clone(&server);
            let mtx = Arc::clone(&recv_mtx);
            CompressionWorker::new(
				id,
                mtx,
                data,
                srv_ref,
                camera,
                on_complete,
            )
        }).collect();

        Self {
            workers,
            sender,
        }
    }

    pub fn dispatch(&self, image: FrameData) {
        let _ = self.sender.send(image);
    }

    pub fn stop(self) {
        for worker in self.workers.into_iter() {
            worker.stop();
        }
    }
}

struct CompressionWorker {
    pub worker: std::thread::JoinHandle<()>,
}

impl CompressionWorker {
    pub fn new(
		thread_id: CoreId,
        receiver: Arc<Mutex<mpsc::Receiver<FrameData>>>,
        data: ImageData,
        server: Arc<CameraServer>,
        camera: VoidPointer,
        on_complete: fn(u64, Vec<u8>, Arc<CameraServer>) -> (),
    ) -> Self {
        let worker = std::thread::spawn(move || {
			let res = core_affinity::set_for_current(thread_id);
            let compresser = Compresser::new();
            loop {
                let isrv_ref = Arc::clone(&server);
                let uncompressed_image = receiver.lock().unwrap().recv().unwrap();
                let (size, data) = compresser.compress_image(data, &uncompressed_image.data);
                unsafe { camera_next_frame(camera, uncompressed_image.request) };
                on_complete(size, data, isrv_ref);
            }
        });

        Self { worker }
    }

    fn stop(self) {
        _ = self.worker.join();
    }
}
