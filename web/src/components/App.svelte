<svelte:options tag="mcw-anesth-adverse-events-analyzer" />

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

	async function handleSubmit(event) {
		event.preventDefault();

		const archive = event.target.elements.archive.files[0];
		const [{ analyze_events }, archiveBuf] = await Promise.all([
			adverseEvents(),
			archive.arrayBuffer()
		]);

		alert(analyze_events(new Uint8Array(archiveBuf)));
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
