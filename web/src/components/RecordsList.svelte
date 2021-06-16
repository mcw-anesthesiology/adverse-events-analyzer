<ol>
	{#each records as record}
		<li><pre>{JSON.stringify(record)}</pre></li>
	{/each}
</ol>

<script>
	import { getRecords } from '../wasm-wrapper.js';

	export let viewHandle;
	let records = [];

	$: fetchRecords(viewHandle);

	async function fetchRecords(handle) {
		records = await getRecords(handle);
	}
</script>
