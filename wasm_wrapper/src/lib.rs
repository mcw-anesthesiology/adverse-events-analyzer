use lazy_static::lazy_static;
use serde_json;
use wasm_bindgen::prelude::*;

use adverse_events::{AdverseEvents, AdverseEventsView};

use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    io::{BufReader, Cursor},
    sync::Mutex,
};

lazy_static! {
    static ref RECORDS: Mutex<UnsafeCell<AdverseEvents>> =
        Mutex::new(UnsafeCell::new(AdverseEvents::new()));
    static ref VIEW_MAP: Mutex<UnsafeCell<HashMap<u8, AdverseEventsView<'static>>>> =
        Mutex::new(UnsafeCell::new(HashMap::new()));
    static ref NEXT_HANDLE: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));
}

#[wasm_bindgen]
pub fn get_events(zip_data: &[u8]) -> Result<u8, JsValue> {
    let cursor = Cursor::new(zip_data);
    let buf = BufReader::new(cursor);
    let adverse_events =
        AdverseEvents::from_zip(buf).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not update views"))?;
    map_cell.get_mut().clear();
    NEXT_HANDLE
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire next handle"))?
        .set(1);

    let records_cell = RECORDS
        .lock()
        .map_err(|_| JsValue::from_str("could not update records"))?;
    unsafe {
        *records_cell.get() = adverse_events;

        let records = records_cell.get();

        let view = (*records).view();

        let map = map_cell.get();
        (*map).insert(0, view);
    }

    Ok(0)
}

#[wasm_bindgen]
pub fn len(handle: u8) -> Result<u32, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&(handle as u8))
        .ok_or(JsValue::from_str("no view found for handle"))?;
    Ok(view.records.len() as u32)
}

#[wasm_bindgen]
pub fn event_counts(handle: u8) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&(handle as u8))
        .ok_or(JsValue::from_str("no view found for handle"))?;
    let counts = view.event_counts();
    serde_json::to_string(&counts).map_err(|_| JsValue::from_str("failed serializing counts"))
}

#[wasm_bindgen]
pub fn with_event(handle: u8, event: &str) -> Result<u8, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not aquire views"))?;

    let map = map_cell.get_mut();
    let view = map
        .get(&(handle as u8))
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let new_view = view.with_event(event);
    let mut next_handle_lock = NEXT_HANDLE
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire next handle"))?;
    let next_handle = next_handle_lock.get_mut();
    let handle: u8 = *next_handle;
    map.insert(handle, new_view);
    *next_handle += 1;

    Ok(handle)
}

#[wasm_bindgen]
pub fn release_view(handle: u8) -> Result<u8, JsValue> {
    if handle == 0 {
        Err(JsValue::from_str("cannot release base view"))
    } else {
        let mut map_cell = VIEW_MAP
            .lock()
            .map_err(|_| JsValue::from_str("could not acquire views"))?;

        map_cell.get_mut().remove(&handle);

        Ok(0)
    }
}

#[wasm_bindgen]
pub fn get_records(handle: u8) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&(handle as u8))
        .ok_or(JsValue::from_str("no view found for handle"))?;
    serde_json::to_string(&view.records)
        .map_err(|_| JsValue::from_str("failed serializing records"))
}
