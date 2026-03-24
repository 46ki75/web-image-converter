use image::ImageReader;
use image::codecs::avif::AvifEncoder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn avif() -> AvifOption {
    AvifOption::new()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct AvifOption {
    quality: u8,
}

#[wasm_bindgen]
impl AvifOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { quality: 80 }
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
        let encoder = AvifEncoder::new_with_speed_quality(&mut bytes, 6, self.quality);
        img.write_with_encoder(encoder)?;

        Ok(bytes)
    }
}
