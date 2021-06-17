// This is a somewhat gross way to make the wasm functions synchronous,
// but since they will be initialized in the root App component before
// any dependent children are rendered it should be okay.

// @ts-ignore
import initialize from '../../wasm_wrapper/Cargo.toml';

import { fromLocalSecondsTimestamp } from './date-utils.js';

interface AdverseEventRecord {
	date: string;
	mrn: string;
	episodeId: string;
	patientName: string;
	diagnosis: string;
	procedure: string;
	anesthesiologist: string;
	anesthesiaStaff: string[];
	location: string;
	adverseEvents: string;
	asa: number;
	anStart: string;
	anStop: string;
	smoker: boolean;
	age: number;
	bmi: number;
}

interface AdverseEventUtils {
	get_events: (zipData: Uint8Array) => number;
	len: (handle: number) => number;
	event_counts: (handle: number) => string;
	with_event: (handle: number, event: string) => number;
	release_view: (handle: number) => number;
	get_records: (handle: number) => string;
	date_range: (handle: number) => Int32Array;
}

// @ts-ignore
let utils: AdverseEventUtils = ({} as AdverseEventUtils);

const init: Promise<AdverseEventUtils> = initialize().then((initializedUtils: AdverseEventUtils) => {
	for (const [key, val] of Object.entries(initializedUtils)) {
		// @ts-ignore
		utils[key] = val;
	}

	return utils;
});

export { initialize, init };

export async function getEvents(zipData: Uint8Array): Promise<number> {
	const utils = await init;
	return utils.get_events(zipData);
}

export async function len(handle: number): Promise<number> {
	const utils = await init;
	return utils.len(handle);
}

export async function eventCounts(handle: number): Promise<Map<string, number>> {
	const utils = await init;
	return new Map(JSON.parse(utils.event_counts(handle)));
}

export async function withEvent(handle: number, event: string): Promise<number> {
	const utils = await init;
	return utils.with_event(handle, event);
}

export async function releaseView(handle: number): Promise<number> {
	const utils = await init;
	return utils.release_view(handle);
}

export async function getRecords(handle: number): Promise<AdverseEventRecord[]> {
	const utils = await init;
	return JSON.parse(utils.get_records(handle));
}

export async function dateRange(handle: number): Promise<[Date, Date]> {
	const utils = await init;
	const [startTimestamp, endTimestamp] = utils.date_range(handle);

	return [
		fromLocalSecondsTimestamp(startTimestamp),
		fromLocalSecondsTimestamp(endTimestamp),
	];
}

export default utils;
