<div>
	<Chart type="line" {data} {axisOptions} />
</div>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { formatShortDate } from '../formatters.js';
	import { periodCounts, Period } from '../wasm-wrapper.js';
	import type { DatePeriodCount } from '../wasm-wrapper.js';

	export let viewHandle: number;

	let counts: DatePeriodCount[] = [];
	$: getPeriodCounts(viewHandle);

	async function getPeriodCounts(handle: number) {
		counts = await periodCounts(handle, Period.Day);
	}

	let labels: string[] = [];
	let values: number[] = [];

	$: labels = counts.map(count => formatShortDate(count.start));
	$: values = counts.map(count => count.value);

	let data = {};
	$: data = {
		labels,
		datasets: [
			{ values }
		],
	};

	const axisOptions = {
		xAxisMode: 'tick',
		yAxisMode: 'tick',
		xIsSeries: true,
	};

</script>
