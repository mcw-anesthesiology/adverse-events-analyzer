<div bind:this={container}>
	<Chart bind:this={chart} {...$$props} />
	<div class="export-button-container">
		<button type="button" on:click={handleExportChart}>
			Save chart image
		</button>
	</div>
</div>

<script>
	import Chart from 'svelte-frappe-charts';

	export let title;
	let container, chart;

	$: updateTitle(title);

	function updateTitle(title) {
		if (!container) return;

		const titleEl = container.querySelector('svg text.title');
		if (titleEl) {
			titleEl.textContent = title;
		}
	}

	function handleExportChart() {
		chart.exportChart();
	}
</script>

<style>
	.export-button-container {
		text-align: right;
	}
</style>
