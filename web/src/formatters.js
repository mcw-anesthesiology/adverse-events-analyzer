export const numFormatter = new Intl.NumberFormat('en-US', {
	minimumFractionDigits: 0,
	maximumFractionDigits: 3
});

export const percentFormatter = new Intl.NumberFormat('en-US', {
	style: 'percent',
	maximumFractionDigits: 3,
});

export function number(s) {
	return numFormatter.format(s);
}

export function percent(s) {
	return percentFormatter.format(s);
}
