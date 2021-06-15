<header>
	<h1>Adverse events analyzer</h1>

</header>

<main>
	{#if dataLoaded}
		<div>
			{#if viewHandle != null}
				{#if length != null}
					<p>
						Length: {length}
					</p>
				{/if}

				<label>
					<input type="checkbox" bind:checked={viewRecords} />
					View records
				</label>

				<button type="button" on:click={() => { viewHandle = filter(viewHandle); }}>
					Filter
				</button>

				<button type="button" on:click={() => { resetFilter(viewHandle); }}>
					Reset filter
				</button>

				{#if viewRecords}
					<RecordsList {viewHandle} />
				{/if}
			{/if}
		</div>
	{:else}
		<form on:submit={handleSubmit}>
			<label>
				Data
				<input type="file" name="archive" accept=".zip" />
			</label>

			<button type="submit">
				Submit
			</button>
		</form>
	{/if}
</main>

<footer>
	<p>
		This site requires modern browser features.
		Please use an updated version of Firefox, Chrome, Edge, or Safari.
	</p>
</footer>

<script>
	import RecordsList from './RecordsList.svelte';

	import utils, { init } from '../wasm-wrapper.js';

	import '../global.css';

	let dataLoaded = false;
	let viewRecords;

	let viewHandle;
	let length;

	$: if (viewHandle != null) {
		updateLength(viewHandle);
	}

	async function updateLength(handle) {
		length = utils.len(handle);
	}

	async function handleSubmit(event) {
		event.preventDefault();

		const archive = event.target.elements.archive.files[0];
		const [_, archiveBuf] = await Promise.all([
			init,
			archive.arrayBuffer()
		]);

		viewHandle = utils.get_events(new Uint8Array(archiveBuf));
		dataLoaded = true;
	}

	function filter(handle) {
		const newHandle = utils.with_event(handle, "Corneal abrasion");
		return newHandle;
	}

	async function resetFilter(handle) {
		viewHandle = utils.release_view(handle);
	}
</script>

<style>
	h1 {
		font-size: 3em;
		margin: 0;
		font-weight: 100;
	}

	main {
		padding: 1em;
		margin: 0 auto;
	}

	footer {
		color: #555;
		padding: 1em;
	}
</style>
