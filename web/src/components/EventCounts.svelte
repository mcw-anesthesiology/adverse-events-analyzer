<div class="event-counts">
	<div>
		<Chart type="percentage" {data} {colors} {maxSlices} height={200} />
	</div>

	<div class="controls">
		<label>
			Number of columns
			<input type="number" bind:value={numChunks} min="1" max="5" />
		</label>
	</div>


	<div class="chunks-container" class:small={numChunks > 2}>
		{#each chunks as counts}
			<EventCountTable {counts} />
		{/each}
	</div>
</div>

<script lang="typescript">
	import Chart from 'svelte-frappe-charts';
	import { schemeSet1, schemeSet2, schemeSet3, schemeTableau10 } from 'd3-scale-chromatic';

	import EventCountTable from './EventCountTable.svelte';

	import { eventCounts } from '../wasm-wrapper.js';

	export let viewHandle: number;

	const colors: string[] = [
		...schemeSet1,
		...schemeSet2,
		...schemeSet3,
		...schemeTableau10
	];
	const maxSlices = colors.length;


	let counts: Map<string, number> = new Map();
	$: getCounts(viewHandle);

	let labels: string[];
	let values: number[];
	$: labels = Array.from(counts.keys());
	$: values = Array.from(counts.values());

	let data = {};
	$: data = {
		labels,
		datasets: [
			{ values }
		]
	};

	let numChunks = 2;
	let chunks: Map<string, number>[];
	$: if (numChunks === 1) {
		chunks = [counts];
	} else {
		const newChunks = [];
		const chunkSize = Math.ceil(counts.size / numChunks);
		const entries = Array.from(counts.entries());
		while (entries.length > 0) {
			const chunk = entries.splice(0, chunkSize);
			newChunks.push(new Map(chunk));
		}

		chunks = newChunks;
	}

	async function getCounts(handle: number) {
		counts = await eventCounts(handle);
	}
</script>

<style>
	.chunks-container {
		display: flex;
	}

	.chunks-container.small {
		font-size: 0.75em;
	}

	.controls {
		display: flex;
		justify-content: flex-end;
	}

	input[type="number"] {
		width: 100%;
	}
</style>
