import { getDate } from './date-utils.js';

export const dateTimeFormatter = new Intl.DateTimeFormat('en-US', {
	year: 'numeric',
	month: 'short',
	day: 'numeric',
	hour: 'numeric',
	minute: 'numeric',
});

export const dateFormatter = new Intl.DateTimeFormat('en-US', {
	year: 'numeric',
	month: 'short',
	day: 'numeric',
});

export function formatDate(date: Date | string): string {
	return applyDateFormatter(date, dateFormatter);
}

export function applyDateFormatter(
	date: Date | string,
	formatter: Intl.DateTimeFormat
): string {
	try {
		if (!(date instanceof Date)) date = getDate(date) as Date;

		return formatter.format(date);
	} catch (e) {
		console.error(e);
		return 'Invalid date';
	}
}

export const numFormatter = new Intl.NumberFormat('en-US', {
	minimumFractionDigits: 0,
	maximumFractionDigits: 3
});

export const percentFormatter = new Intl.NumberFormat('en-US', {
	style: 'percent',
	maximumFractionDigits: 3,
});

export function number(s: number) {
	return numFormatter.format(s);
}

export function percent(s: number) {
	return percentFormatter.format(s);
}
