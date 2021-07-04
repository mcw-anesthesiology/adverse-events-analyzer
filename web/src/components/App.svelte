<section class="adverse-events-app">
	<header>
		<h1>Adverse events</h1>

		{#if $inputFile}
			<button type="button" class="reset-button" on:click={handleClearArchive}>
				Reset
			</button>
		{/if}
	</header>

	<main>
		{#if dataLoaded}
			<div>
				{#if rootHandle != null}
					<ViewFilter {rootHandle} />
				{/if}
			</div>
		{:else if loading}
			<div class="loading-container">
				<p>Loading...</p>
			</div>
		{:else}
			<form on:submit={handleSubmit} class:loading>
				<label>
					<span>
						Drop file
						<small>
							or click to select
						</small>
					</span>

					<input type="file" name="archive" accept=".zip" on:input={handleArchiveChange} disabled={loading} />
				</label>
			</form>

			{#if loadingError}
				<p>Sorry, there was a problem loading the data.</p>

				<p>Are you sure the file you selected is valid?</p>
			{/if}
		{/if}
	</main>

	<footer>
		<p>
			This site requires modern browser features.
			Please use an updated version of Firefox, Chrome, Edge, or Safari.
		</p>
	</footer>
</section>

<script>
	import ViewFilter from './ViewFilter.svelte';

	import { inputFile } from '../stores.js';
	import { init } from '../wasm-wrapper.js';

	let loading = false;
	let dataLoaded = false;
	let loadingError;

	let rootHandle;

	async function handleArchiveChange(event) {
		const input = event.target;
		if (input.files.length === 0) return;

		const archive = input.files[0];
		inputFile.set(archive);
	}

	$: getEvents($inputFile);

	async function getEvents(archive) {
		if (!archive) return;

		loading = true;
		loadingError = null;
		try {
			const [utils, archiveBuf] = await Promise.all([
				init,
				archive.arrayBuffer()
			]);

			rootHandle = utils.get_events(new Uint8Array(archiveBuf));
		} catch (err) {
			console.error(err);
			loadingError = err;
		}

		loading = false;
		dataLoaded = true;
	}

	async function handleSubmit(event) {
		event.preventDefault();
	}

	function handleClearArchive() {
		$inputFile = null;
		location.reload();
	}
</script>

<style>
	.adverse-events-app {
		display: flex;
		flex-direction: column;
		align-items: stretch;
		min-height: 100vh;
	}

	header {
		padding: 0.25em 1em;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	h1 {
		font-size: 3em;
		margin: 0;
		font-weight: 100;
	}

	.reset-button {
		padding: 0.5em 1em;
	}

	main {
		flex-grow: 1;
		padding: 1em;
	}

	footer {
		color: #555;
		padding: 1em;
	}

	form,
	.loading-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
	}

	.loading-container {
		height: 70vh;
		font-size: 2em;
		color: #666;
	}

	form.loading {
		opacity: 0.75;
	}

	form label {
		font-size: 2em;
		box-sizing: border-box;
		display: flex;
		flex-direction: column;
		align-items: space-between;
		justify-content: space-between;
		padding: 3em;
		width: 60%;
		height: 400px;
		max-height: 80vh;
		margin: 1em;
		border: 1px solid #ccc;
		border-radius: 3px;
		background-color: #fefefe;
		cursor: pointer;
	}

	label:hover {
		background: #fafafa;
	}

	form.loading label {
		cursor: wait;
	}

	small {
		opacity: 0.5;
	}
</style>
