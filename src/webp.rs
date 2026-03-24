use image::ImageReader;
use wasm_bindgen::prelude::*;
use webp::{Encoder, WebPMemory};

#[wasm_bindgen]
pub fn webp() -> WebpOption {
    WebpOption::new()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct WebpOption {
    quality: Option<f32>,
}

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

        if let Some(quality) = self.quality {
            if quality < 1f32 || quality > 100f32 {
                return Err(JsError::new("quality must be between 1 and 100"));
            }

            let encoder: Encoder = Encoder::from_image(&img).unwrap();

            let webp: WebPMemory = encoder.encode(quality);

            Ok(webp.to_vec())
        } else {
            let mut bytes = Vec::new();

            let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut bytes);

            img.write_with_encoder(encoder)?;

            Ok(bytes)
        }
    }
}
