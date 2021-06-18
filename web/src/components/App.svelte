<header>
	<h1>Adverse events</h1>

</header>

<main>
	{#if dataLoaded}
		<div>
			{#if rootHandle != null}
				<ViewFilter {rootHandle} />
			{/if}
		</div>
	{:else}
		<form on:submit={handleSubmit} class:loading>
			<label>
				{#if loading}
					Loading...
				{:else}
					<span>
						Drop file
						<small>
							or click to select
						</small>
					</span>
				{/if}

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

<script>
	import ViewFilter from './ViewFilter.svelte';

	import { init } from '../wasm-wrapper.js';

	import '../anet.css';
	import '../global.css';

	let loading = false;
	let dataLoaded = false;
	let loadingError;

	let rootHandle;

	async function handleArchiveChange(event) {
		const input = event.target;
		if (input.files.length === 0) return;

		const archive = input.files[0];
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
</script>

<style>
	h1 {
		font-size: 3em;
		margin: 0;
		font-weight: 100;
	}

	header {
		padding: 1em;
	}

	main {
		padding: 1em;
		margin: 0 auto;
	}

	footer {
		color: #555;
		padding: 1em;
	}

	form {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
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
