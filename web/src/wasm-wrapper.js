// This is a somewhat gross way to make the wasm functions synchronous,
// but since they will be initialized in the root App component before
// any dependent children are rendered it should be okay.

import initialize from '../../wasm_wrapper/Cargo.toml';

let utils = {};

const init = initialize().then(initializedUtils => {
	for (const [key, val] of Object.entries(initializedUtils)) {
		utils[key] = val;
	}
});

export { initialize, init };

export default utils;
