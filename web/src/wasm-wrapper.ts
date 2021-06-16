// This is a somewhat gross way to make the wasm functions synchronous,
// but since they will be initialized in the root App component before
// any dependent children are rendered it should be okay.

// @ts-ignore
import initialize from '../../wasm_wrapper/Cargo.toml';

interface AdverseEventUtils {
	get_events: (zipData: Uint8Array) => number;
}

// @ts-ignore
let utils: AdverseEventUtils = {};

const init = initialize().then((initializedUtils: AdverseEventUtils) => {
	for (const [key, val] of Object.entries(initializedUtils)) {
		// @ts-ignore
		utils[key] = val;
	}

	return utils;
});

export { initialize, init };

export default utils;
