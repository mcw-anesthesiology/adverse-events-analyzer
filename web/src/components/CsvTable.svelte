{#await rowsPromise}
	<p>Loading...</p>
{:then { data: rows }}
	<table>
		{#if headerRows}
			<thead>
				{#each rows.slice(0, headerRows) as cells}
					<tr>
						{#each cells as cell, i}
							<th class:isHeaderCol={i < headerCols}>{cell}</th>
						{/each}
					</tr>
				{/each}
			</thead>
		{/if}
		<tbody>
			{#each headerRows ? rows.slice(headerRows) : rows as cells}
				{#if cells.length > 0 && (cells.length > 1 || cells[0] != '')}
					<tr>
						{#each cells as cell, i}
							{#if i < headerCols}
								<th>{cell}</th>
							{:else}
								<td>{formatCell(cell)}</td>
							{/if}
						{/each}
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>
{:catch err}
	{@debug err}
	<p>Sorry, there was a problem.</p>
{/await}

<script>
	import { parse } from 'papaparse';

	import { number, percent } from '../formatters.js';

	export let csv, headerRows = 1, headerCols = 1;
	let rowsPromise;

	$: rowsPromise = parse(csv);

	function formatCell(cellText) {
		let formatter = number;
		if (cellText.length) {
			if (cellText.endsWith('%')) {
				cellText = cellText.substring(0, cellText.length - 1);
				formatter = percent;
			}

			let n = Number(cellText);
			if (!Number.isNaN(n)) {
				if (formatter === percent) {
					n /= 100;
				}
				return formatter(n);
			}
		}

		return cellText;
	}
</script>

<style>
	table {
		border-collapse: collapse;
		width: 100%;
	}

	th, td {
		border: 1px solid #ccc;
		padding: 0.25em 0.5em;
	}

	thead th,
	td {
		text-align: right;
	}

	tbody th,
	th.isHeaderCol {
		text-align: left;
	}
</style>
