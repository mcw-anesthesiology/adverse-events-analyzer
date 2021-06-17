<fieldset class="date-fieldset">
	<legend>{label}</legend>
	<label>
		Range
		<select bind:this={dateRangeSelect} bind:value={selectedDateRange} on:change={handleDateRangeChange}>
			{#each Array.from(options.entries()) as [name, dates]}
				<option value={JSON.stringify(dates)} data-date-range-option>
					{name}
				</option>
			{/each}
			<option value={JSON.stringify({ startDate: undefined, endDate: undefined })}>
				All time
			</option>
			<option value="">Custom</option>
		</select>
	</label>
	<div>
		<input type="date" bind:value={startDate} readonly={Boolean(selectedDateRange)} />
		–
		<input type="date" bind:value={endDate} readonly={Boolean(selectedDateRange)} />
	</div>
</fieldset>

<script>
	let selectedDateRange, dateRangeSelect;
	export let options = new Map(), startDate, endDate, label = 'Date range';

	$: if (options && (!startDate && !endDate)) {
		if (dateRangeSelect) {
			dateRangeSelect.dispatchEvent(new Event('change'));
		}
	}

	function handleDateRangeChange(event) {
		if (event.target.value) {
			const deserialized = JSON.parse(event.target.value);
			startDate = deserialized.startDate;
			endDate = deserialized.endDate;
		}
	}

	$: if (startDate === '') {
		startDate = undefined;
	}
	$: if (endDate === '') {
		endDate = undefined;
	}
</script>

<style>
	fieldset {
		text-align: left;
		padding: 1em;
		margin-bottom: 1em;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
		align-items: center;
	}

	label {
		margin: 1em;
	}

	label select {
		display: block;
	}

	fieldset > div {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
		align-items: center;
	}

	input {
		display: inline-block;
		margin: 0.5em;
	}
</style>
