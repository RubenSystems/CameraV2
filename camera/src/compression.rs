pub struct Compresser {
    handle: turbojpeg_sys::tjhandle,
}

#[derive(Clone, Copy)]
pub struct ImageData {
    pub width: u64,
    pub height: u64,
}

impl Compresser {
    pub fn new() -> Self {
        Self {
            handle: unsafe { turbojpeg_sys::tjInitCompress() },
        }
    }

    pub fn compress_image(&self, metadata: ImageData, buffer: &[u8]) -> (u64, Vec<u8>) {
        let mut buf_size = (metadata.width * metadata.height * 3) as u64;

        let mut jpeg_buf = vec![0_u8; buf_size as usize];
        let buf_ref = &mut jpeg_buf.as_mut_ptr() as *mut *mut u8;

        let compression = unsafe {
            turbojpeg_sys::tjCompressFromYUV(
                self.handle,
                buffer.as_ptr(),
                metadata.width as i32,
                1,
                metadata.height as i32,
                2,
                buf_ref,
                &mut buf_size as *mut u64,
                75,
                0,
            )
        };

        (buf_size, jpeg_buf)
    }
}

// #[derive(Debug)]
// pub enum CompressionError {
//     CompressionIssue,
// }

// pub struct JPEGCompressor {
//     compressor: turbojpeg::Compressor,
// }

// #[derive(Clone, Copy)]
// pub struct ImageData {
//     pub width: u64,
//     pub height: u64,
//     pub pitch: u64,
// }

// impl JPEGCompressor {
//     pub fn new() -> Self {
//         let mut compressor = turbojpeg::Compressor::new().unwrap();
//         compressor.set_quality(75);
//         compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);

//         JPEGCompressor { compressor }
//     }

//     pub fn compress_rgb(
//         &mut self,
//         data: &[u8],
//         metadata: ImageData,
//     ) -> Result<Vec<u8>, CompressionError> {
//         let image = convert_to_rgb_image(data, metadata.width, metadata.height, metadata.pitch);

//         match self.compressor.compress_to_owned(image) {
//             Ok(compressed) => Ok(compressed.to_vec()),
//             Err(_) => Err(CompressionError::CompressionIssue),
//         }
//     }

// }

// impl Default for JPEGCompressor {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// fn convert_to_rgb_image(data: &[u8], width: u64, height: u64, pitch: u64) -> turbojpeg::Image<&[u8]> {
//     turbojpeg::Image::<&[u8]> {
//         pixels: data,
//         width: width as usize,
//         height: height as usize,
//         pitch: pitch as usize,
//         format: turbojpeg::PixelFormat::RGB,
//     }
// }

// fn convert_to_yuv_image(data: &[u8], width: u64, height: u64) -> turbojpeg::YuvImage::<&[u8]> {
//     turbojpeg::YuvImage::<&[u8]> {
//         pixels: data,
//         width: width as usize,
//         align: 1,
//         height: height as usize,
//         subsamp: turbojpeg::Subsamp::Sub2x2,
//     }
// }
