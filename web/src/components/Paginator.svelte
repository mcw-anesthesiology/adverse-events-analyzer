<div class="paginator">
	<button type="button" on:click={handleBackPage} disabled={pageNum === 0}>
		←
	</button>

	<span>
		Page

		<input type="number" aria-label="Page number" value={pageNum + 1} on:change={handlePageChange} min="1" max={maxPageNum + 1} />

		of

		<span>
			{maxPageNum + 1}
		</span>
	</span>

	<button type="button" on:click={handleForwardPage} disabled={pageNum >= maxPageNum}>
		→
	</button>
</div>

<script>
	export let pageNum, maxPageNum;

	function handlePageChange(event) {
		const input = Number(event.target.value);
		if (!Number.isNaN(input)) {
			const value = input - 1;
			if (value >= 0 && value <= maxPageNum) {
				pageNum = value;
			}
		}
	}

	function handleBackPage() {
		pageNum--;
	}

	function handleForwardPage() {
		pageNum++;
	}
</script>

<style>
	.paginator {
		margin: 1em;
		display: flex;
		justify-content: center;
	}

	.paginator > * {
		margin: 0.5em;
	}

	button {
		padding: 0.25em 1em;
	}

	input {
		width: 5em;
	}
</style>
