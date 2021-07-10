use chrono::NaiveDate;
use lazy_static::lazy_static;
use serde_json;
use wasm_bindgen::prelude::*;

use adverse_events::{
    sort_map, AdverseEventRecord, AdverseEvents, AdverseEventsView, BreakdownType,
    DatePeriodPercentage, DatePeriodView, Error as AdverseEventsError, Period, TimeseriesType,
};

use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    convert::From,
    io::{BufReader, Cursor},
    str::FromStr,
    sync::Mutex,
};

type ViewHandle = u8;

const ISO_DATE_FORMAT: &str = "%Y-%m-%d";

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
pub fn with_complications_specified(handle: ViewHandle) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not aquire views"))?;

    let map = map_cell.get_mut();
    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let new_view = view.with_filter(|r| r.complications.is_some());
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
pub fn with_complications_occurred(handle: ViewHandle) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not aquire views"))?;

    let map = map_cell.get_mut();
    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let new_view = view.with_filter(|r| r.complications == Some(true));
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
pub fn with_any_event(handle: ViewHandle) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not aquire views"))?;

    let map = map_cell.get_mut();
    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let new_view = view.with_any_event();
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
pub fn between(handle: ViewHandle, start: &str, end: &str) -> Result<ViewHandle, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;
    let map = map_cell.get_mut();

    let view = map
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let start = NaiveDate::parse_from_str(start, ISO_DATE_FORMAT)
        .map_err(|_| JsValue::from_str("failed parsing start date"))?;
    let end = NaiveDate::parse_from_str(end, ISO_DATE_FORMAT)
        .map_err(|_| JsValue::from_str("failed parsing end date"))?;

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
pub fn get_records(
    handle: ViewHandle,
    start: Option<usize>,
    length: Option<usize>,
) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let records: Vec<&AdverseEventRecord>;

    let records_ref: &Vec<&AdverseEventRecord> = if start.is_some() || length.is_some() {
        let mut iter: Box<dyn Iterator<Item = &AdverseEventRecord>> =
            Box::new(view.records.iter().copied());

        if let Some(start) = start {
            iter = Box::new(iter.skip(start));
        }

        if let Some(length) = length {
            iter = Box::new(iter.take(length));
        }

        records = iter.collect();

        &records
    } else {
        &view.records
    };

    serde_json::to_string(records_ref).map_err(|_| JsValue::from_str("failed serializing records"))
}

#[wasm_bindgen]
pub fn date_range(handle: ViewHandle) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let (start, end) = view.date_range().ok_or(JsValue::from_str("no dates"))?;

    serde_json::to_string(&[start.to_string(), end.to_string()])
        .map_err(|_| JsValue::from_str("failed serializing dates"))
}

#[wasm_bindgen]
pub fn get_timeseries(
    handle: ViewHandle,
    timeseries_type: &str,
    period: &str,
) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let timeseries_type = TimeseriesType::from_str(timeseries_type)
        .map_err(|_| JsValue::from_str("invalid timeseries type"))?;

    let period = Period::from_str(period).map_err(|_| JsValue::from_str("invalid period"))?;

    match timeseries_type {
        TimeseriesType::EventCount => {
            let view_counts = view.by_period(period, |record| !record.adverse_events.is_empty());
            let period_counts: Vec<_> = view_counts.into_iter().map(|vc| vc.to_count()).collect();
            serde_json::to_string(&period_counts)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
        TimeseriesType::EventPercentage => {
            let mut with_events: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |record| !record.adverse_events.is_empty())
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |_| true)
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records = sort_map(all_records);

            let period_percentages: Vec<_> = all_records
                .into_iter()
                .map(|(dates, dpv)| {
                    let with_event_count: usize = with_events
                        .remove(&dates)
                        .map(|dpv| dpv.value.records.len())
                        .unwrap_or_default();

                    let total_count = dpv.value.records.len();

                    DatePeriodPercentage {
                        period: dpv.period,
                        start: dpv.start,
                        end: dpv.end,
                        value: if total_count == 0 {
                            0.0
                        } else {
                            with_event_count as f64 / total_count as f64 * 100.0
                        },
                    }
                })
                .collect();

            serde_json::to_string(&period_percentages)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
        TimeseriesType::ComplicationSpecifiedCount => {
            let view_counts = view.by_period(period, |record| record.complications.is_some());
            let period_counts: Vec<_> = view_counts.into_iter().map(|vc| vc.to_count()).collect();
            serde_json::to_string(&period_counts)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
        TimeseriesType::ComplicationSpecifiedPercentage => {
            let mut with_complications: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |record| record.complications.is_some())
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |_| true)
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records = sort_map(all_records);

            let period_percentages: Vec<_> = all_records
                .into_iter()
                .map(|(dates, dpv)| {
                    let with_complications_count: usize = with_complications
                        .remove(&dates)
                        .map(|dpv| dpv.value.records.len())
                        .unwrap_or_default();

                    let total_count = dpv.value.records.len();

                    DatePeriodPercentage {
                        period: dpv.period,
                        start: dpv.start,
                        end: dpv.end,
                        value: if total_count == 0 {
                            0.0
                        } else {
                            with_complications_count as f64 / total_count as f64 * 100.0
                        },
                    }
                })
                .collect();

            serde_json::to_string(&period_percentages)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
        TimeseriesType::ComplicationOccurredCount => {
            let view_counts = view.by_period(period, |record| record.complications == Some(true));
            let period_counts: Vec<_> = view_counts.into_iter().map(|vc| vc.to_count()).collect();
            serde_json::to_string(&period_counts)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
        TimeseriesType::ComplicationOccurredPercentage => {
            let mut with_complications: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |record| record.complications == Some(true))
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records: HashMap<(NaiveDate, NaiveDate), DatePeriodView> = view
                .by_period(period, |_| true)
                .into_iter()
                .map(|dpv| ((dpv.start, dpv.end), dpv))
                .collect();

            let all_records = sort_map(all_records);

            let period_percentages: Vec<_> = all_records
                .into_iter()
                .map(|(dates, dpv)| {
                    let with_complications_count: usize = with_complications
                        .remove(&dates)
                        .map(|dpv| dpv.value.records.len())
                        .unwrap_or_default();

                    let total_count = dpv.value.records.len();

                    DatePeriodPercentage {
                        period: dpv.period,
                        start: dpv.start,
                        end: dpv.end,
                        value: if total_count == 0 {
                            0.0
                        } else {
                            with_complications_count as f64 / total_count as f64 * 100.0
                        },
                    }
                })
                .collect();

            serde_json::to_string(&period_percentages)
                .map_err(|_| JsValue::from_str("failed serializing view counts"))
        }
    }
}

#[wasm_bindgen]
pub fn get_breakdown(handle: ViewHandle, breakdown_type: &str) -> Result<String, JsValue> {
    let mut map_cell = VIEW_MAP
        .lock()
        .map_err(|_| JsValue::from_str("could not acquire views"))?;

    let view = map_cell
        .get_mut()
        .get(&handle)
        .ok_or(JsValue::from_str("no view found for handle"))?;

    let breakdown_type = BreakdownType::from_str(breakdown_type)
        .map_err(|_| JsValue::from_str("invalid breakdown type"))?;

    serde_json::to_string(&view.get_breakdown(breakdown_type))
        .map_err(|_| JsValue::from_str("failed serializing view counts"))
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
