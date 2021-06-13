export function toISODate(d) {
	const s = d.toISOString();
	return s.substring(0, s.indexOf('T'));
}
export function yearBlockStarts(date = new Date()) {
	const blocks = [];

	let year = date.getMonth() >= JULY
		? date.getFullYear()
		: date.getFullYear() - 1;

	let d = new Date(year, JULY, 1);
	blocks.push(new Date(d));
	d.setDate(d.getDate() + 28);

	// Set next block start to Monday, first block is <= 4 weeks
	d.setDate((d.getDate() - d.getDay()) + 1);
	blocks.push(new Date(d));

	for (let i = 0; i < 11; i++) {
		d.setDate(d.getDate() + 28);
		blocks.push(new Date(d));
	}

	return blocks;
}

const JULY = 6;
