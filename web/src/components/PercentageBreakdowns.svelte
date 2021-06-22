<section>
	<Chart title="Complications" type="pie" data={withComplicationsData} {maxSlices} height={pieHeight} />

	<div class="percentages-container">
		<div>
			<Chart title="With events" type="percentage" data={withEventsData} {maxSlices} {height} />
		</div>
		<div>
			<Chart title="Patient age" type="percentage" data={patientAgeData} {maxSlices} {height} />
		</div>
		<div>
			<Chart title="Patient BMI" type="percentage" data={patientBmiData} {maxSlices} {height} />
		</div>
		<div>
			<Chart title="Smoking status" type="percentage" data={patientSmokerData} {maxSlices} {height} />
		</div>
	</div>
</section>

<script lang="typescript">
	import Chart from './FrappeChart.svelte';

	import { BreakdownType, getBreakdown } from '../wasm-wrapper.js';
	import type { LabeledCount } from '../wasm-wrapper.js';

	const pieHeight = 400;
	const height = 200;
	const maxSlices = 10;

	export let viewHandle: number;

	let withComplications: LabeledCount[] = [];
	let withEvents: LabeledCount[] = [];
	let patientAge: LabeledCount[] = [];
	let patientBmi: LabeledCount[] = [];
	let patientSmoker: LabeledCount[] = [];

	$: loadCounts(viewHandle);


	async function loadCounts(handle: number) {
		withComplications = await getBreakdown(handle, BreakdownType.WithComplications);
		withEvents = await getBreakdown(handle, BreakdownType.WithEvent);
		patientAge = await getBreakdown(handle, BreakdownType.PatientAge);
		patientBmi = await getBreakdown(handle, BreakdownType.PatientBmi);
		patientSmoker = await getBreakdown(handle, BreakdownType.PatientSmoker);
	}

	let withComplicationsData = {};
	let withEventsData = {};
	let patientAgeData = {};
	let patientBmiData = {};
	let patientSmokerData = {};

	$: withComplicationsData = getData(withComplications);
	$: withEventsData = getData(withEvents);
	$: patientAgeData = getData(patientAge);
	$: patientBmiData = getData(patientBmi);
	$: patientSmokerData = getData(patientSmoker);

	function getData(counts: LabeledCount[]): object {
		return {
			labels: counts.map(b => b.label),
			datasets: [
				{ values: counts.map(b => b.value) }
			]
		};
	}
</script>

<style>
	.percentages-container {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-evenly;
	}

	.percentages-container > div {
		margin: 1em;
		box-sizing: border-box;
		width: calc(50% - 2em);
		min-width: 500px;
	}
</style>
