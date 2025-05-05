use image::ImageReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn webp() -> WebpOption {
    WebpOption::new()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct WebpOption {}

#[wasm_bindgen]
impl WebpOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert(&self, image_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
        let img = ImageReader::new(std::io::Cursor::new(image_bytes))
            .with_guessed_format()?
            .decode()?;

        let mut bytes = Vec::new();

        let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut bytes);

        img.write_with_encoder(encoder)?;

        Ok(bytes)
    }
}
