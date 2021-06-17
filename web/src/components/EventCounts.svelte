<div>
	{#if data}

		<Chart {data} type="bar" />
	{/if}

	<table>
		<thead>
			<th>Event</th>
			<td>Number</td>
		</thead>
		<tbody>
			{#each Array.from(counts.entries()) as [event, count]}
				<tr>
					<th>{event}</th>
					<td>{count}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';

	import { eventCounts } from '../wasm-wrapper.js';

	export let viewHandle: number;

	let counts: Map<string, number> = new Map();
	let data: {};
	let labels: string[] = [];
	let values: number[] = [];

	$: labels = Array.from(counts.keys());
	$: values = Array.from(counts.values());
	$: data = {
		labels,
		datasets: [
			{
				values
			}
		]
	}

	$: getCounts(viewHandle);

	async function getCounts(handle: number) {
		counts = await eventCounts(handle);
	}

</script>
