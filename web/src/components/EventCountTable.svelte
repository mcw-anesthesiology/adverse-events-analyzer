<table>
	<thead>
		<tr>
			<th>Event</th>
			<th class="numeric">Number</th>
			<td class="bar-col"></td>
		</tr>
	</thead>
	<tbody>
		{#each Array.from(counts.entries()) as [event, count]}
			<tr>
				<th>
					<a href="#" on:click={handleEventClick}>
						{event}
					</a>
				</th>
				<td class="numeric">{count}</td>
				<td class="bar-col">
					<div class="bar" style={`width: ${getPercentage(count)}%`}></div>
				</td>
			</tr>
		{/each}
	</tbody>
</table>

<script lang="typescript">
	import { getContext } from 'svelte';

	export let counts: Map<string, number>;

	let values: number[] = [];
	$: values = Array.from(counts.values());

	let maxValue = 0;
	$: maxValue = values.reduce((acc, value) => value > acc ? value : acc, 0);

	let getPercentage: (value: number) => number;
	$: getPercentage = value => maxValue === 0 ? 0 : value / maxValue * 100;

	const { addEventFilter } = getContext('filter');

	async function handleEventClick(event: Event) {
		event.preventDefault();

		const th = event.target as HTMLTableHeaderCellElement;
		const eventName = th.textContent.trim();

		return addEventFilter(eventName);
	}
</script>

<style>
	table {
		width: 100%;
	}

	.bar-col {
		width: 50%;
		border: none;
	}


	.bar {
		transition: width;
		background: var(--bar-color, aqua);
		height: 1em;
	}
</style>
