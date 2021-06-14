use wasm_bindgen::prelude::*;

use adverse_events::AdverseEvents;

use std::io::{BufReader, Cursor};

#[wasm_bindgen]
pub fn analyze_events(zip_data: &[u8]) -> Result<f64, JsValue> {
    let cursor = Cursor::new(zip_data);
    let buf = BufReader::new(cursor);
    let adverse_events =
        AdverseEvents::from_zip(buf).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    Ok(adverse_events.records.len() as f64)
}
