
#[derive(Debug)]
pub enum CompressionError {
    CompressionIssue,
}

pub struct JPEGCompressor {
    compressor: turbojpeg::Compressor,
}

#[derive(Clone, Copy)]
pub struct ImageData {
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
}

impl JPEGCompressor {
    pub fn new() -> Self {
        let mut compressor = turbojpeg::Compressor::new().unwrap();
        compressor.set_quality(75);
        compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);

        JPEGCompressor { compressor }
    }

    pub fn compress(
        &mut self,
        data: &[u8],
        metadata: ImageData,
    ) -> Result<Vec<u8>, CompressionError> {
        let image = convert_to_image(data, metadata.width, metadata.height, metadata.pitch);

        match self.compressor.compress_to_owned(image) {
            Ok(compressed) => Ok(compressed.to_vec()),
            Err(_) => Err(CompressionError::CompressionIssue),
        }
    }
}

fn convert_to_image(data: &[u8], width: u64, height: u64, pitch: u64) -> turbojpeg::Image<&[u8]> {
    let image = turbojpeg::Image::<&[u8]> {
        pixels: data,
        width: width as usize,
        height: height as usize,
        pitch: pitch as usize,
        format: turbojpeg::PixelFormat::RGB,
    };

    image
}
