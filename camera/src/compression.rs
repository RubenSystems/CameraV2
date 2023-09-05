use image::{Rgb, RgbImage};

pub enum CompressionError {
    CompressionIssue,
}

pub fn compress(data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, CompressionError> {
    let image = convert_to_image(data, width, height);
    compress_image(image)
}

fn convert_to_image(data: &[u8], width: u32, height: u32) -> RgbImage {
    let image = image::RgbImage::from_fn(width, height, |x, y| {
        let base_offset: usize = ((width * y) + x) as usize;
        let r = data[base_offset];
        let g = data[base_offset + 1];
        let b = data[base_offset + 2];
        Rgb([r, g, b])
    });

    image
}

fn compress_image(image: RgbImage) -> Result<Vec<u8>, CompressionError> {
    match turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::Sub2x2) {
        Ok(data) => Ok(data.to_vec()),
        Err(_) => Err(CompressionError::CompressionIssue),
    }
}
