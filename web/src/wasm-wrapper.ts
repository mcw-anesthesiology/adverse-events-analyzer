// This is a somewhat gross way to make the wasm functions synchronous,
// but since they will be initialized in the root App component before
// any dependent children are rendered it should be okay.

// @ts-ignore
import initialize from '../../wasm_wrapper/Cargo.toml';

import { parseDate, toISODate } from './date-utils.js';

export interface AdverseEventRecord {
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

export interface AdverseEventUtils {
	get_events: (zipData: Uint8Array) => number;
	len: (handle: number) => number;
	event_counts: (handle: number) => string;
	with_any_event: (handle: number) => number;
	with_event: (handle: number, event: string) => number;
	between: (handle: number, start: string, end: string) => number;
	release_view: (handle: number) => number;
	get_records: (handle: number, start?: number, length?: number) => string;
	date_range: (handle: number) => string;
	get_timeseries: (
		handle: number,
		timeseriesType: string,
		period: string
	) => string;
	get_breakdown: (handle: number, breakdownType: string) => string;
}

// @ts-ignore
let utils: AdverseEventUtils = {} as AdverseEventUtils;

const init: Promise<AdverseEventUtils> = initialize().then(
	(initializedUtils: AdverseEventUtils) => {
		for (const [key, val] of Object.entries(initializedUtils)) {
			// @ts-ignore
			utils[key] = val;
		}

		return utils;
	}
);

export { initialize, init };

export async function getEvents(zipData: Uint8Array): Promise<number> {
	const utils = await init;
	return utils.get_events(zipData);
}

export async function len(handle: number): Promise<number> {
	const utils = await init;
	return utils.len(handle);
}

export async function eventCounts(
	handle: number
): Promise<Map<string, number>> {
	const utils = await init;
	return new Map(JSON.parse(utils.event_counts(handle)));
}

export async function withAnyEvent(handle: number): Promise<number> {
	const utils = await init;
	return utils.with_any_event(handle);
}

export async function withEvent(
	handle: number,
	event: string
): Promise<number> {
	const utils = await init;
	return utils.with_event(handle, event);
}

export async function between(
	handle: number,
	start: Date,
	end: Date
): Promise<number> {
	const utils = await init;

	return utils.between(handle, toISODate(start), toISODate(end));
}

export async function releaseView(handle: number): Promise<number> {
	const utils = await init;
	return utils.release_view(handle);
}

export async function getRecords(
	handle: number,
	start?: number,
	length?: number
): Promise<AdverseEventRecord[]> {
	const utils = await init;
	return JSON.parse(utils.get_records(handle, start, length));
}

export async function dateRange(handle: number): Promise<[Date, Date]> {
	const utils = await init;
	const [start, end] = JSON.parse(utils.date_range(handle));

	return [parseDate(start), parseDate(end)];
}

export enum Period {
	Day = 'day',
	Week = 'week',
	Month = 'month',
	Year = 'year',
}

export enum TimeseriesType {
	EventCount = 'event',
	EventPercentage = 'eventPercentage',
	ComplicationSpecifiedCount = 'complicationSpecified',
	ComplicationSpecifiedPercentage = 'complicationSpecifiedPercentage',
	ComplicationOccurredCount = 'complicationOccurred',
	ComplicationOccurredPercentage = 'complicationOccurredPercentage',
}

interface StringDatePeriodNumber {
	start: string;
	end: string;
	value: number;
}

export interface DatePeriodNumber {
	start: Date;
	end: Date;
	value: number;
}

export async function getTimeseries(
	handle: number,
	timeseriesType: TimeseriesType,
	period: Period
): Promise<DatePeriodNumber[]> {
	const utils = await init;
	const counts = utils.get_timeseries(
		handle,
		timeseriesType.toString(),
		period.toString()
	);

	return JSON.parse(counts).map((count: StringDatePeriodNumber) => {
		return {
			start: parseDate(count.start),
			end: parseDate(count.end),
			value: count.value,
		};
	});
}

export interface LabeledCount {
	label: string;
	value: number;
}

export enum BreakdownType {
	WithComplications = 'complications',
	WithEvent = 'event',
	PatientAge = 'age',
	PatientBmi = 'bmi',
	PatientSmoker = 'smoker',
}

export async function getBreakdown(
	handle: number,
	breakdownType: BreakdownType
): Promise<LabeledCount[]> {
	const utils = await init;
	return JSON.parse(utils.get_breakdown(handle, breakdownType.toString()));
}

export default utils;
