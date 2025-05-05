use image::ImageReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn jpeg() -> JpegOption {
    JpegOption::new()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct JpegOption {
    quality: u8,
}

#[wasm_bindgen]
impl JpegOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = quality;
        self
    }

    pub fn convert(&self, image_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
        if self.quality < 1 || self.quality > 100 {
            return Err(JsError::new("quality must be between 1 and 100"));
        }

        let img = ImageReader::new(std::io::Cursor::new(image_bytes))
            .with_guessed_format()?
            .decode()?;

        let mut bytes = Vec::new();

        let mut encoder =
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, self.quality);

        encoder.encode_image(&img)?;

        Ok(bytes)
    }
}
