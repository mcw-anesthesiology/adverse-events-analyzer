<div class="view-filter">

	<div class="filters-container">
		{#if filterStack.length > 0}
			<button type="button" class="pop-filter-button" disabled={!filterStack.length} on:click|preventDefault={popFilter}>
				← Remove filter
			</button>

			<section class="filters">
				<h4>Filters</h4>

				<div class="filter-breadcrumbs">
					{#each filterStack as filter}
						{#if filter.type === FilterType.WithComplicationsSpecified}
							<span>
								Complications specified
							</span>
						{:else if filter.type === FilterType.WithComplicationsOccurred}
							<span>
								Complications occurred
							</span>
						{:else if filter.type === FilterType.HasEvent}
							<span>
								Has events
							</span>
						{:else if isEventFilter(filter)}
							<span>
								Event: {filter.eventName}
							</span>
						{:else if isDateFilter(filter)}
							<span>
								Between:
								<DateRange start={filter.startDate} end={filter.endDate} />
							</span>
						{/if}
					{/each}
				</div>
			</section>
		{/if}
	</div>


	<section class="view-header">
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

		<div class="filter-controls">
			<div class="filter-buttons">
				<button type="button" on:click={addHasEventFilter} disabled={hasHasEventFilter}>
					Add has events filter
				</button>

				<button type="button" on:click={addWithComplicationsSpecifiedFilter} disabled={hasWithComplicationsSpecifiedFilter}>
					Add complications specified filter
				</button>

				<button type="button" on:click={addWithComplicationsOccurredFilter} disabled={hasWithComplicationsOccurredFilter}>
					Add complications occurred filter
				</button>
			</div>

			<form class="date-filter-form" on:submit={handleAddDateFilter}>
				{#if earliest && latest}
					<DateRangePicker label="Date filter" bind:selectedDateRange bind:startDate bind:endDate min={earliest} max={latest} />

					<button type="submit" disabled={!startDate || !endDate}>
						Add date filter
					</button>
				{/if}
			</form>
		</div>
	</section>

	<Tabs>
		<TabList>
			<Tab>Breakdowns</Tab>
			<Tab>Event counts</Tab>
			<Tab>Records</Tab>
		</TabList>

		<TabPanel>
			<EventTrend viewHandle={currentHandle} />
			<PercentageBreakdowns viewHandle={currentHandle} />
		</TabPanel>

		<TabPanel>
			<EventCounts viewHandle={currentHandle} />
		</TabPanel>

		<TabPanel>
			<RecordsList viewHandle={currentHandle} />
		</TabPanel>
	</Tabs>
</div>

<script lang="ts">
	import { setContext } from 'svelte';
	import { Tabs, Tab, TabList, TabPanel } from 'svelte-tabs';

	import DateRange from './DateRange.svelte';
	import DateRangePicker from './DateRangePicker.svelte';
	import EventCounts from './EventCounts.svelte';
	import EventTrend from './EventTrend.svelte';
	import PercentageBreakdowns from './PercentageBreakdowns.svelte';
	import RecordsList from './RecordsList.svelte';

	import { between, dateRange, len, withAnyEvent, withEvent, releaseView, withComplicationsSpecified, withComplicationsOccurred } from '../wasm-wrapper.js';
	import { getDate } from '../date-utils.js';

	export let rootHandle: number;

	let selectedDateRange: string;
	let startDate: Date;
	let endDate: Date;

	let length: number = 0;
	let earliest: Date;
	let latest: Date;

	$: updateLength(currentHandle);
	$: updateDates(currentHandle);

	let hasWithComplicationsSpecifiedFilter: boolean;
	$: hasWithComplicationsSpecifiedFilter = filterStack.some(event => event.type === FilterType.WithComplicationsSpecified || event.type === FilterType.WithComplicationsOccurred);

	let hasWithComplicationsOccurredFilter: boolean;
	$: hasWithComplicationsOccurredFilter = filterStack.some(event => event.type === FilterType.WithComplicationsOccurred);

	let hasHasEventFilter: boolean;
	$: hasHasEventFilter = filterStack.some(event => event.type === FilterType.HasEvent);

	function handleAddDateFilter(event: Event) {
		event.preventDefault();

		if (startDate && endDate) {
			addDateFilter(
				getDate(startDate),
				getDate(endDate)
			);
			selectedDateRange = undefined;
		}
	};

	async function updateLength(handle: number) {
		length = await len(handle);
	}

	async function updateDates(handle: number) {
		const [s, e] = await dateRange(handle);
		earliest = s;
		latest = e;
	}

	enum FilterType {
		WithComplicationsSpecified = 'complicationsSpecified',
		WithComplicationsOccurred = 'complicationsOccurred',
		HasEvent = 'hasEvent',
		Event = 'event',
		Date = 'date',
	}

	interface Filter {
		type: FilterType;
		handle: number;
	}

	interface HasEventFilter extends Filter {
		type: FilterType.HasEvent;
	}

	interface EventFilter extends Filter {
		type: FilterType.Event;
		eventName: string;
	}

	interface DateFilter extends Filter {
		type: FilterType.Date;
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

	async function addWithComplicationsSpecifiedFilter() {
		try {
			const handle = await withComplicationsSpecified(currentHandle);
			const filter: HasEventFilter = {
				type: FilterType.WithComplicationsSpecified,
				handle,
			};

			addFilter(filter);
		} catch (err) {
			console.error(err);
		}
	}

	async function addWithComplicationsOccurredFilter() {
		try {
			const handle = await withComplicationsOccurred(currentHandle);
			const filter: HasEventFilter = {
				type: FilterType.WithComplicationsOccurred,
				handle,
			};

			addFilter(filter);
		} catch (err) {
			console.error(err);
		}
	}

	async function addHasEventFilter() {
		try {
			const handle = await withAnyEvent(currentHandle);
			const filter: HasEventFilter = {
				type: FilterType.HasEvent,
				handle,
			};

			addFilter(filter);
		} catch (err) {
			console.error(err);
		}
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
	.filters-container {
		min-height: 4em;
		display: flex;
		margin-bottom: 1em;
		align-items: flex-start;
	}

	.filters {
		margin: 0 1em;
	}

	h4 {
		margin: 0;
	}

	.filter-breadcrumbs {
		font-size: 0.9em;
		display: flex;
		flex-wrap: wrap;
	}

	.filter-breadcrumbs > span {
		margin: 0.5em;
		padding: 0.25em 0.5em;
		border: 1px solid var(--border-color);
		border-radius: 2px;
		background-color: #fafafa;
	}

	.pop-filter-button {
		display: block;
		padding: 0.5em 1em;
		margin: 2em 1em 1em;
	}

	.view-header,
	.meta,
	.filter-controls {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
	}

	.view-header {
		justify-content: space-between;
		margin-bottom: 1em;
	}

	.meta {
		flex-grow: 2;
		justify-content: space-evenly;
		font-size: 1.25em;
	}

	.filter-controls {
		flex-grow: 1;
		justify-content: space-between;
	}

	.filter-buttons {
		display: flex;
		flex-direction: column;
	}

	.filter-buttons button {
		margin: 0.5em 0;
	}

	.date-filter-form {
		border: 1px solid var(--border-color);
		border-radius: 1px;
		padding: 1em;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
