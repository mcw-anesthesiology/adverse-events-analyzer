<fieldset class="date-fieldset">
	<legend>{label}</legend>
	<label>
		Range
		<select bind:this={dateRangeSelect} bind:value={selectedDateRange} on:change={handleDateRangeChange}>
			<option value="">Custom</option>
			{#each Array.from(options.entries()) as [name, dates]}
				<option value={JSON.stringify(dates)} data-date-range-option>
					{name}
				</option>
			{/each}
		</select>
	</label>
	<div>
		<input type="date" bind:value={startDate} readonly={Boolean(selectedDateRange)} {min} max={endDate || max} />
		–
		<input type="date" bind:value={endDate} readonly={Boolean(selectedDateRange)} min={startDate || min} {max} />
	</div>
</fieldset>

<script>
	import { getDate, getAcademicYear, toISODate } from '../date-utils.js';

	let dateRangeSelect;
	export let startDate, endDate, label = 'Date range';
	export let selectedDateRange;
	export let inputOptions = undefined;
	export let min = undefined, max = undefined;
	export { inputOptions as options };

	// FIXME: For some reason this reruns whenever an option is selected
	let options;
	$: if (inputOptions) {
		options = inputOptions;
	} else if (min && max) {
		const minDate = getDate(min);
		const maxDate = getDate(max);

		const [maxStart, maxEnd] = getAcademicYear(maxDate);

		if (maxStart <= minDate && maxEnd >= maxDate) {
			options = new Map();
		} else {
			let date = new Date(minDate);
			const newOptions = new Map();
			while (date <= maxEnd) {
				const [yearStart, yearEnd] = getAcademicYear(date);
				newOptions.set(
					`${yearStart.getFullYear()} - ${yearEnd.getFullYear()}`,
					{
						startDate: toISODate(yearStart),
						endDate: toISODate(yearEnd),
					}
				);
				date.setFullYear(date.getFullYear() + 1);
			}

			options = newOptions;
		}
	} else {
		options = new Map();
	}

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
		padding: 0.5em;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
		align-items: center;
		border: none;
	}

	label {
		margin: 0 1em;
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
