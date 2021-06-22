<div>
	<div class="trend-controls">
		<fieldset>
			<legend>Chart view</legend>

			<div class="view-selection-container">
				<div class="view-selection-group">
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.EventCount} />
						# with events
					</label>
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.EventPercentage} />
						% with events
					</label>
				</div>
				<div class="view-selection-group">
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.ComplicationSpecifiedCount} />
						# with complications button pressed
					</label>
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.ComplicationSpecifiedPercentage} />
						% with complications button pressed
					</label>
				</div>
				<div class="view-selection-group">
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.ComplicationOccurredCount} />
						# with complications = "Yes"
					</label>
					<label>
						<input type="radio" bind:group={viewType} value={TimeseriesType.ComplicationOccurredPercentage} />
						% with complications = "Yes"
					</label>
				</div>
			</div>
		</fieldset>
	</div>

	<Chart type="line" {data} {axisOptions} {lineOptions} />
</div>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { formatNumber, formatShortDate } from '../formatters.js';
	import { getTimeseries, Period, TimeseriesType } from '../wasm-wrapper.js';
	import type { DatePeriodNumber } from '../wasm-wrapper.js';

	export let viewHandle: number;

	enum ViewType {
		Count = 'count',
		Percentage = 'percentage'
	}

	let viewType = TimeseriesType.EventCount;

	let counts: DatePeriodNumber[] = [];
	$: getPeriodCounts(viewHandle, viewType);

	async function getPeriodCounts(handle: number, timeseriesType: TimeseriesType) {
		counts = await getTimeseries(handle, timeseriesType, Period.Day);
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

	.view-selection-container {
		display: flex;
		flex-wrap: wrap;
	}

	label {
		display: block;
		margin: 0.5em;
		white-space: nowrap;
	}
</style>
