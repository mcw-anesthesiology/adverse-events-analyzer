<div class="view-filter">
	<p>
		Length: {length}
	</p>

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
</div>

<script lang="ts">
	import RecordsList from './RecordsList.svelte';

	// @ts-ignore
	import utils from '../wasm-wrapper.ts';

	export let rootHandle: number;
	let viewRecords: boolean;


	let length: number = 0;

	$: updateLength(currentHandle);

	async function updateLength(handle: number) {
		length = utils.len(handle);
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

	function addEventFilter(eventName: string) {
		try {
			const handle = utils.with_event(currentHandle, eventName)
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

	function popFilter() {
		if (!filterStack.length) return;

		const removed = filterStack.pop();
		filterStack = filterStack;
		utils.release_view(removed.handle);

		const lastIndex = filterStack.length - 1;
		if (lastIndex < 0) {
			currentHandle = rootHandle;
		} else {
			currentHandle = filterStack[lastIndex].handle;
		}
	}
</script>
