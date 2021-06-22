<button type="button" on:click={handleClick}>
	<slot>Save to PDF</slot>
</button>

<script>
	import { printElement } from '../utils.js';

	export let target, filename = 'download.pdf', landscape = false, options = {};

	function handleClick() {
		const element = typeof target === 'string' ? document.querySelector(target) : target;

		printElement(element, filename, {
			landscape,
			...options
		});
	}
</script>
