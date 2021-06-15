<ol>
	{#each records as record}
		<li><pre>{JSON.stringify(record)}</pre></li>
	{/each}
</ol>

<script>
	import utils from '../wasm-wrapper.js';

	export let viewHandle;
	let records = [];

	$: getRecords(viewHandle);

	async function getRecords(handle) {
		records = JSON.parse(utils.get_records(handle));
	}
</script>
