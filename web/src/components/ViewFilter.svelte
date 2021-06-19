<div class="view-filter">

	<section class="filters">
		{#if filterStack.length > 0}
			<h4>Filters</h4>

			<div class="filter-breadcrumbs">
				{#each filterStack as filter}
					{#if isEventFilter(filter)}
						<span class="event-filter">
							Event: {filter.eventName}
						</span>
					{:else if isDateFilter(filter)}
						<span class="event-filter">
							Between:
							<DateRange start={filter.startDate} end={filter.endDate} />
						</span>
					{/if}
				{/each}
			</div>

			<div class="back-container">
				<a href="#" class:disabled={!filterStack.length} on:click|preventDefault={popFilter}>
					← Remove filter
				</a>
			</div>
		{/if}
	</section>


	<aside class="meta">
		<span>
			{length} records
		</span>

		<span>
			{#if earliest && latest}
				<DateRange start={earliest} end={latest} />
			{/if}
		</span>
	</aside>


	<form>
		<DateRangePicker bind:startDate bind:endDate />

		<label>
			<input type="checkbox" bind:checked={viewRecords} />
			View records
			{#if length > 1000}
				<small>
					<b>Warning:</b> viewing many records at once may take a
					long time to load or freeze the window.
				</small>
			{/if}
		</label>
	</form>

	{#if viewRecords}
		<RecordsList viewHandle={currentHandle} />
	{/if}

	<div>
		<EventTrend viewHandle={currentHandle} />
		<EventCounts viewHandle={currentHandle} />
	</div>

</div>

<script lang="typescript">
	import { setContext } from 'svelte';

	import DateRange from './DateRange.svelte';
	import DateRangePicker from './DateRangePicker.svelte';
	import EventCounts from './EventCounts.svelte';
	import EventTrend from './EventTrend.svelte';
	import RecordsList from './RecordsList.svelte';

	import { between, dateRange, len, withEvent, releaseView } from '../wasm-wrapper.js';
	import { getDate } from '../date-utils.js';

	export let rootHandle: number;
	let viewRecords: boolean;

	let startDate: Date;
	let endDate: Date;


	let length: number = 0;
	let earliest: Date;
	let latest: Date;

	$: updateLength(currentHandle);
	$: updateDates(currentHandle);
	$: if (startDate && endDate) {
		updateDateFilter(
			getDate(startDate),
			getDate(endDate)
		);
	}

	async function updateLength(handle: number) {
		length = await len(handle);
	}

	async function updateDates(handle: number) {
		const [s, e] = await dateRange(handle);
		earliest = s;
		latest = e;
	}

	async function updateDateFilter(start: Date, end: Date) {
		return addDateFilter(start, end);
	}

	enum FilterType {
		Event = 'event',
		Date = 'date',
	}

	interface Filter {
		type: FilterType;
		handle: number;
	}

	interface EventFilter extends Filter {
		eventName: string;
	}

	interface DateFilter extends Filter {
		startDate: Date;
		endDate: Date;
	}

	function isEventFilter(filter: Filter): filter is EventFilter {
		return filter.type === FilterType.Event;
	}

	function isDateFilter(filter: Filter): filter is DateFilter {
		return filter.type === FilterType.Date;
	}


	let filterStack: Filter[] = [];
	let currentHandle: number = rootHandle;

	function addFilter(filter: Filter) {
		filterStack.push(filter);
		filterStack = filterStack;
		currentHandle = filter.handle;
	}

	async function addEventFilter(eventName: string) {
		try {
			const handle = await withEvent(currentHandle, eventName);
			const filter: EventFilter = {
				type: FilterType.Event,
				handle,
				eventName
			};

			addFilter(filter);
		} catch (err) {
			console.error(err);
		}
	}

	async function addDateFilter(startDate: Date, endDate: Date) {
		try {
			const handle = await between(currentHandle, startDate, endDate);
			const filter: DateFilter = {
				type: FilterType.Date,
				handle,
				startDate,
				endDate
			};

			addFilter(filter);
		} catch (err) {
			console.error(err);
		}
	}

	async function popFilter() {
		if (!filterStack.length) return;

		const removed = filterStack.pop();
		filterStack = filterStack;
		releaseView(removed.handle);

		const lastIndex = filterStack.length - 1;
		if (lastIndex < 0) {
			currentHandle = rootHandle;
		} else {
			currentHandle = filterStack[lastIndex].handle;
		}
	}

	setContext('filter', {
		addEventFilter,
		addDateFilter,
		popFilter,
	});
</script>

<style>
	.filters {
		min-height: 6em;
		margin-bottom: 1em;
	}

	h4 {
		margin: 0;
	}

	.filter-breadcrumbs {
		font-size: 0.9em;
		display: flex;
		min-height: 2em;
	}

	.filter-breadcrumbs > span {
		margin: 0.5em;
		padding: 0.25em 0.5em;
		border: 1px solid var(--border-color);
		border-radius: 2px;
		background-color: #fafafa;
	}
</style>
