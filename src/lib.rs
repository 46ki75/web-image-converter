pub mod args;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_png(image_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
    let img = image::load_from_memory(image_bytes).map_err(|e| JsError::from(e))?;

    let mut output = std::io::Cursor::new(Vec::new());

    img.write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| JsError::from(e))?;

    Ok(output.into_inner())
}
