<section>
	<div>
		<Chart title="With events" type="pie" data={withEventsData} />
	</div>
</section>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { BreakdownType, getBreakdown } from '../wasm-wrapper.js';
	import type { LabeledCount } from '../wasm-wrapper.js';

	export let viewHandle: number;

	$: loadBreakdown(viewHandle, BreakdownType.WithEvent);

	let withEvents: LabeledCount[] = [];

	async function loadBreakdown(handle: number, breakdownType: BreakdownType) {
		withEvents = await getBreakdown(handle, breakdownType);
	}

	$: withEventsData = {
		labels: withEvents.map(b => b.label),
		datasets: [
			{ values: withEvents.map(b => b.value) }
		]
	};
	$: console.log(withEventsData);

</script>

<style>
	section {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-evenly;
	}

	section > div {
		width: 200px;
	}
</style>
