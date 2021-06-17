<time datetime={dateObj.toISOString()}>
	{applyDateFormatter(dateObj, formatter)}
</time>

<script>
	import { parseDate } from '../date-utils.js';
	import { applyDateFormatter, dateFormatter, dateTimeFormatter } from '../formatters.js';

	export let date;
	export let showTime = false;
	let formatter;

	let dateObj;
	$: dateObj = date instanceof Date ? date : parseDate(date);

	$: formatter = showTime ? dateTimeFormatter : dateFormatter;
</script>
