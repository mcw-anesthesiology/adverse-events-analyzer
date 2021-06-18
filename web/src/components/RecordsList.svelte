<div>

	<Paginator bind:pageNum {maxPageNum} />

	<table>
		<thead>
			<tr>
				<th>Date</th>
				<th>MRN</th>
				<th>Diagnosis</th>
				<th>Procedure</th>
				<th>Anesthesiologist</th>
				<th>Anesthesia staff</th>
				<th>Location</th>
				<th>ASA</th>
				<th>Start</th>
				<th>Stop</th>
				<th>Smoker</th>
				<th>Age</th>
				<th>BMI</th>
				<th>Adverse events</th>
			</tr>
		</thead>
		<tbody>
			{#each pageRecords as record}
				<tr>
					<td>
						<RichDate date={record.date} />
					</td>
					<td>{record.mrn}</td>
					<td>{record.diagnosis}</td>
					<td>{record.procedure}</td>
					<td>{record.anesthesiologist}</td>
					<td>
						<ul>
							{#each record.anesthesiaStaff as staffMember}
								<li>{staffMember}</li>
							{/each}
						</ul>
					</td>
					<td>{record.location}</td>
					<td>{record.asa}</td>
					<td>{record.anStart}</td>
					<td>{record.anStop}</td>
					<td>
						{#if record.smoker}
							✓
						{/if}
					</td>
					<td>{record.age}</td>
					<td>{record.bmi}</td>
					<td>
						<ul>
							{#each record.adverseEvents as event}
								<li>{event}</li>
							{/each}
						</ul>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<Paginator bind:pageNum {maxPageNum} />

	<div>
		<label>
			Page size
			<input type="number" bind:value={pageSize} step="5" />
		</label>
	</div>
</div>

<script>
	import RichDate from './RichDate.svelte';
	import Paginator from './Paginator.svelte';

	import { getRecords } from '../wasm-wrapper.js';

	export let viewHandle;

	let records = [];
	$: fetchRecords(viewHandle);

	async function fetchRecords(handle) {
		records = await getRecords(handle);
	}

	let pageNum = 0;
	let pageSize = 20;
	let pageRecords = [];
	let maxPageNum;
	$: maxPageNum = Math.ceil(records.length / pageSize) - 1;

	$: pageRecords = records.slice(pageNum * pageSize, (pageNum + 1) * pageSize);

</script>

<style>
	ul {
		padding-left: 1em;
	}
</style>
