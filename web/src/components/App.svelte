<header>
	<h1>Adverse events analyzer</h1>

</header>

<main>
	{#if dataLoaded}
		<div>
			{#if rootHandle != null}
				<ViewFilter {rootHandle} />
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
	import ViewFilter from './ViewFilter.svelte';

	import { init } from '../wasm-wrapper.js';

	import '../global.css';

	let dataLoaded = false;

	let rootHandle;

	async function handleSubmit(event) {
		event.preventDefault();

		const archive = event.target.elements.archive.files[0];
		const [utils, archiveBuf] = await Promise.all([
			init,
			archive.arrayBuffer()
		]);

		rootHandle = utils.get_events(new Uint8Array(archiveBuf));
		dataLoaded = true;
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
