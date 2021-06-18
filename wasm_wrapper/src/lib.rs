use chrono::{Local, NaiveDate, TimeZone};
use lazy_static::lazy_static;
use serde_json;
use wasm_bindgen::prelude::*;

use adverse_events::{
    sort_map, AdverseEvents, AdverseEventsView, Error as AdverseEventsError, Period,
};

use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    convert::{From, TryInto},
    io::{BufReader, Cursor},
    str::FromStr,
    sync::Mutex,
};

type ViewHandle = u8;

lazy_static! {
    static ref RECORDS: Mutex<UnsafeCell<AdverseEvents>> =
        Mutex::new(UnsafeCell::new(AdverseEvents::new()));
    static ref VIEW_MAP: Mutex<UnsafeCell<HashMap<ViewHandle, AdverseEventsView<'static>>>> =
        Mutex::new(UnsafeCell::new(HashMap::new()));
    static ref NEXT_HANDLE: Mutex<Cell<ViewHandle>> = Mutex::new(Cell::new(0));
}

#[wasm_bindgen]
pub fn get_events(zip_data: &[ViewHandle]) -> Result<ViewHandle, JsValue> {
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
pub fn len(handle: ViewHandle) -> Result<u32, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;
    Ok(view.records.len() as u32)
}

#[wasm_bindgen]
pub fn event_counts(handle: ViewHandle) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;
    let counts = sort_map(view.event_counts());
    serde_json::to_string(&counts).map_err(|_| JsValue::from_str("failed serializing counts"))
}

#[wasm_bindgen]
pub fn with_event(handle: ViewHandle, event: &str) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not aquire views"))?;

    let map = map_cell.get_mut();
    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let new_view = view.with_event(event);
    let mut next_handle_lock = NEXT_HANDLE
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire next handle"))?;
    let next_handle = next_handle_lock.get_mut();
    let handle: ViewHandle = *next_handle;
    map.insert(handle, new_view);
    *next_handle += 1;

    Ok(handle)
}

#[wasm_bindgen]
pub fn between(
    handle: ViewHandle,
    start_timestamp: i32,
    end_timestamp: i32,
) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;
    let map = map_cell.get_mut();

    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let start = Local
        .timestamp(start_timestamp.into(), 0)
        .naive_local()
        .date();
    let end = Local
        .timestamp(end_timestamp.into(), 0)
        .naive_local()
        .date();

    let new_view = view.between(start, end);
    let mut next_handle_lock = NEXT_HANDLE
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire next handle"))?;
    let next_handle = next_handle_lock.get_mut();
    let handle: ViewHandle = *next_handle;
    map.insert(handle, new_view);
    *next_handle += 1;

    Ok(handle)
}

#[wasm_bindgen]
pub fn release_view(handle: ViewHandle) -> Result<ViewHandle, JsValue> {
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
pub fn get_records(handle: ViewHandle) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    serde_json::to_string(&view.records)
        .map_err(|_| JsValue::from_str("failed serializing records"))
}

#[wasm_bindgen]
pub fn date_range(handle: ViewHandle) -> Result<Box<[i32]>, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let (start, end) = view.date_range().ok_or(JsValue::from_str("no dates"))?;

    Ok(Box::new([to_timestamp(start)?, to_timestamp(end)?]))
}

pub fn period_counts(handle: ViewHandle, period: &str) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let period = Period::from_str(period).map_err(|_| JsValue::from_str("invalid period"))?;

    let view_counts = view.by_period(period);
    let period_counts: Vec<_> = view_counts.into_iter().map(|vc| vc.to_count()).collect();
    serde_json::to_string(&period_counts)
        .map_err(|_| JsValue::from_str("failed serializing view counts"))
}

fn to_timestamp(date: NaiveDate) -> Result<i32, JsValue> {
    date.and_hms(0, 0, 0)
        .timestamp()
        .try_into()
        .map_err(|_| JsValue::from_str("out of range timestamp"))
}

#[derive(Debug)]
pub enum Error {
    AdverseEventsError(AdverseEventsError),
}

impl From<AdverseEventsError> for Error {
    fn from(e: AdverseEventsError) -> Self {
        Error::AdverseEventsError(e)
    }
}

impl From<Error> for JsValue {
    fn from(e: Error) -> JsValue {
        // FIXME: Improve this
        JsValue::from_str(&format!("{:?}", e))
    }
}
