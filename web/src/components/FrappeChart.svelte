<div>
	<Chart bind:this={chart} {...$$props} />
	<div class="export-button-container">
		<button type="button" on:click={handleExportChart}>
			Save chart image
		</button>
	</div>
</div>

<script>
	import Chart from 'svelte-frappe-charts';

	let chart;

	function handleExportChart() {
		chart.exportChart();
	}
</script>

<style>
	.export-button-container {
		text-align: right;
	}
</style>
