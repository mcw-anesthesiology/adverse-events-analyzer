<div>
	<div class="trend-controls">
		<fieldset>
			<legend>Chart view</legend>

			<label>
				<input type="radio" bind:group={viewType} value={ViewType.Count} />
				# cases with events
			</label>
			<label>
				<input type="radio" bind:group={viewType} value={ViewType.Percentage} />
				% cases with events
			</label>
		</fieldset>
	</div>

	<Chart type="line" {data} {axisOptions} {lineOptions} />
</div>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { formatShortDate } from '../formatters.js';
	import { periodPercentages, periodCounts, Period } from '../wasm-wrapper.js';
	import type { DatePeriodCount } from '../wasm-wrapper.js';

	export let viewHandle: number;

	enum ViewType {
		Count = 'count',
		Percentage = 'percentage'
	}

	let viewType = ViewType.Count;

	let counts: DatePeriodCount[] = [];
	$: getPeriodCounts(viewHandle, viewType);

	async function getPeriodCounts(handle: number, viewType: ViewType) {
		switch (viewType) {
			case ViewType.Count:
				counts = await periodCounts(handle, Period.Day);
				break;
			case ViewType.Percentage:
				counts = await periodPercentages(handle, Period.Day);
				break;
		}
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

	const lineOptions = {
		hideDots: true
	};
</script>

<style>
	.trend-controls {
		display: flex;
		justify-content: flex-end;
	}

	label {
		margin: 0.5em;
	}
</style>
