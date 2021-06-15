<header>
	<h1>Adverse events analyzer</h1>

</header>

<main>
	<form on:submit={handleSubmit}>
		<label>
			Data
			<input type="file" name="archive" accept=".zip" />
		</label>

		<button type="submit">
			Submit
		</button>

		{#if viewHandle != null}
			{#if length != null}
				<p>
					Length: {length}
				</p>
			{/if}

			<button type="button" on:click={() => { viewHandle = filter(viewHandle); }}>
				Filter
			</button>

			<button type="button" on:click={() => { getRecords(viewHandle); }}>
				Get records
			</button>

			<button type="button" on:click={() => { resetFilter(viewHandle); }}>
				Reset filter
			</button>
		{/if}


		{#if records}
			<ul>
				{#each records as record}
					<li><pre>{JSON.stringify(record)}</pre></li>
				{/each}
			</ul>
		{/if}
	</form>
</main>

<footer>
	<p>
		This site requires modern browser features.
		Please use an updated version of Firefox, Chrome, Edge, or Safari.
	</p>
</footer>

<script>
	import { adverseEvents } from '../wasm-wrappers.js';

	import '../global.css';

	let viewHandle;
	let records;
	let length;

	let utils;
	const init = adverseEvents().then(
		newUtils => {
			utils = newUtils;
		}
	);

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
	}

	async function getRecords(handle) {
		records = JSON.parse(utils.get_records(handle));
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
