import download from 'downloadjs';

export async function printElement(
	target,
	filename = 'download.pdf',
	options = {}
) {
	const body = `<html id="printer"><body><main>${target.outerHTML}</main></body></html>`;

	const styles = Array.from(document.styleSheets).map(styleSheet => {
		if (styleSheet.href) {
			return { url: styleSheet.href };
		}

		try {
			return {
				content: Array.from(styleSheet.cssRules)
					.map(rule => rule.cssText)
					.join(' '),
			};
		} catch (err) {
			console.error(err);
		}
	});

	styles.push({
		content: `html#printer, html#printer body, html#printer main { margin: 0; padding: 0; width: unset; height: unset; min-width: unset; min-height: unset; }`,
	});
	styles.push({ content: `html#printer main { padding: 0 1em; }` });

	return fetch(import.meta.env.PRINTER_ENDPOINT, {
		method: 'POST',
		body: JSON.stringify({
			body,
			styles,
			options,
		}),
	})
		.then(response => {
			if (response.ok) {
				return response.blob();
			}

			throw new Error(response.status);
		})
		.then(blob => {
			download(blob, filename, 'application/pdf');
		})
		.catch(err => {
			console.error(err);
		});
}
