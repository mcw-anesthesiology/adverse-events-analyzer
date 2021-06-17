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
					<th>
						<a href="#" on:click={handleEventClick}>
							{event}
						</a>
					</th>
					<td>{count}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<script lang="typescript">
	import { getContext } from 'svelte';
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

	const { addEventFilter } = getContext('filter');

	async function handleEventClick(event: Event) {
		event.preventDefault();

		const th = event.target as HTMLTableHeaderCellElement;
		const eventName = th.textContent.trim();

		return addEventFilter(eventName);
	}
</script>
