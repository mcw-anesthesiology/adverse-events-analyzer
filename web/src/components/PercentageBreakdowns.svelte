<section>
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
</section>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { BreakdownType, getBreakdown } from '../wasm-wrapper.js';
	import type { LabeledCount } from '../wasm-wrapper.js';

	const height = 200;
	const maxSlices = 10;

	export let viewHandle: number;

	let withEvents: LabeledCount[] = [];
	let patientAge: LabeledCount[] = [];
	let patientBmi: LabeledCount[] = [];
	let patientSmoker: LabeledCount[] = [];

	$: loadCounts(viewHandle);


	async function loadCounts(handle: number) {
		withEvents = await getBreakdown(handle, BreakdownType.WithEvent);
		patientAge = await getBreakdown(handle, BreakdownType.PatientAge);
		patientBmi = await getBreakdown(handle, BreakdownType.PatientBmi);
		patientSmoker = await getBreakdown(handle, BreakdownType.PatientSmoker);
	}

	let withEventsData = {};
	let patientAgeData = {};
	let patientBmiData = {};
	let patientSmokerData = {};

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
	section {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-evenly;
	}

	section > div {
		margin: 1em;
		box-sizing: border-box;
		width: calc(50% - 2em);
		min-width: 500px;
	}
</style>
