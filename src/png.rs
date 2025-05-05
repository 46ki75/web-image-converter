use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn png() -> PngOption {
    PngOption::new()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct PngOption {
    pub quality: u8,
}

#[wasm_bindgen]
impl PngOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = quality;
        self
    }

    pub fn convert(&self, image_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
        let img = image::ImageReader::new(std::io::Cursor::new(image_bytes))
            .with_guessed_format()?
            .decode()?;

        let mut bytes = Vec::new();

        let encoder = image::codecs::png::PngEncoder::new(&mut bytes);

        img.write_with_encoder(encoder)?;

        Ok(bytes)
    }
}
