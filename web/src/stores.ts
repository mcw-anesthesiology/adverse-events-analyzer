import { writable } from 'svelte/store';
import * as localforage from 'localforage';

const INPUT_FILE_KEY = 'inputFile';

export const inputFile = writable(undefined);

localforage.getItem(INPUT_FILE_KEY).then(blob => {
	inputFile.set(blob);
});

inputFile.subscribe(blob => {
	if (blob) {
		localforage.setItem(INPUT_FILE_KEY, blob);
	} else {
		localforage.removeItem(INPUT_FILE_KEY);
	}
});
