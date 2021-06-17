<div class="view-filter">
	<p>
		Length: {length}
	</p>

	{#if start && end}
		<p>
			<DateRange {start} {end} />
		</p>
	{/if}

	<div class="filter-breadcrumbs">
		{#each filterStack as filter}
			{#if isEventFilter(filter)}
				<span class="event-filter">
					Name: {filter.eventName}
				</span>
			{/if}
		{/each}
	</div>

	<form>
		<label>
			<input type="checkbox" bind:checked={viewRecords} />
			View records
		</label>

		<button type="button" on:click={() => {
			addEventFilter('Corneal abrasion');
		}}>
			Filter
		</button>

		<button type="button" on:click={popFilter}>
			Remove filter
		</button>
	</form>

	{#if viewRecords}
		<RecordsList viewHandle={currentHandle} />
	{/if}

	<EventCounts viewHandle={currentHandle} />
</div>

<script lang="typescript">
	import DateRange from './DateRange.svelte';
	import EventCounts from './EventCounts.svelte';
	import RecordsList from './RecordsList.svelte';

	import { dateRange, len, withEvent, releaseView } from '../wasm-wrapper.js';

	export let rootHandle: number;
	let viewRecords: boolean;


	let length: number = 0;
	let start: Date;
	let end: Date;

	$: updateLength(currentHandle);
	$: updateDates(currentHandle);

	async function updateLength(handle: number) {
		length = await len(handle);
	}

	async function updateDates(handle: number) {
		const [s, e] = await dateRange(handle);
		start = s;
		end = e;
	}

	function isEventFilter(filter: Filter): filter is EventFilter {
		return filter.type === FilterType.Event;
	}

	enum FilterType {
		Event = 'event'
	}

	interface Filter {
		type: FilterType;
		handle: number;
	}

	interface EventFilter extends Filter {
		eventName: string;
	}

	let filterStack: Filter[] = [];
	let currentHandle: number = rootHandle;

	function addFilter(filterObj: Filter) {
		filterStack.push(filterObj);
		filterStack = filterStack;
		currentHandle = filterObj.handle;
	}

	async function addEventFilter(eventName: string) {
		try {
			const handle = await withEvent(currentHandle, eventName)
			const filterObj: EventFilter = {
				type: FilterType.Event,
				handle,
				eventName
			}

			addFilter(filterObj)
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
</script>
