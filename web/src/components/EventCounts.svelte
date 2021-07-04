<div class="event-counts">
	<Chart type="percentage" {data} {colors} {maxSlices} height={200} />

	<div class="controls">
		<label>
			Number of columns
			<input type="number" bind:value={numChunks} min="1" max="5" />
		</label>

		<PrintButton target={chunksContainer} filename="Event counts.pdf" options={printOptions} />
	</div>

	<div bind:this={chunksContainer} class="chunks-container" class:small={numChunks > 2}>
		{#each chunks as counts}
			<EventCountTable {counts} />
		{/each}
	</div>
</div>

<script lang="ts">
	import { schemeSet1, schemeSet2, schemeSet3, schemeTableau10 } from 'd3-scale-chromatic';

	import Chart from './FrappeChart.svelte';
	import EventCountTable from './EventCountTable.svelte';
	import PrintButton from './PrintButton.svelte';

	import { eventCounts } from '../wasm-wrapper.js';

	let chunksContainer: HTMLDivElement;
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

	const printOptions = {
		printBackground: true
	};
</script>

<style>
	.controls {
		display: flex;
		justify-content: flex-end;
		align-items: flex-end;
		margin-top: 1em;
	}

	.controls > :global(:not(:first-child)) {
		margin-left: 1em;
	}

	.controls > :global(button) {
		display: block;
	}

	.chunks-container {
		display: flex;
	}

	.chunks-container.small {
		font-size: 0.75em;
	}

	input[type="number"] {
		width: 100%;
	}

	@media print {
		.chunks-container {
			flex-wrap: wrap;
		}

		.chunks-container :global(table ~ table thead) {
			display: none;
		}
	}
</style>
