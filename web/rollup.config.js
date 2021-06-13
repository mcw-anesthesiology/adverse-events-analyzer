/* eslint-env node */

import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import css from 'rollup-plugin-css-only';
import livereload from 'rollup-plugin-livereload';
import rust from '@wasm-tool/rollup-plugin-rust';
import { terser } from 'rollup-plugin-terser';

const production = !process.env.ROLLUP_WATCH;

function serve() {
	let server;

	function toExit() {
		if (server) server.kill(0);
	}

	return {
		writeBundle() {
			if (server) return;
			server = require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
				stdio: ['ignore', 'inherit', 'inherit'],
				shell: true
			});

			process.on('SIGTERM', toExit);
			process.on('exit', toExit);
		}
	};
}

export default {
	input: 'src/app.js',
	output: [
		{
			sourcemap: true,
			format: 'es',
			name: 'app',
			file: 'public/build/app.mjs'
		},
		{
			sourcemap: true,
			format: 'iife',
			name: 'MCWAnesthAdverseEvents',
			file: 'public/build/bundle.js'
		}
	],
	plugins: [
		svelte({
			emitCss: true,
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
				customElement: true
			}
		}),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: ['svelte']
		}),
		commonjs(),
		css({
			output: 'bundle.css'
		}),

		rust({
			debug: false,
			serverPath: '/build/',
			watchPatterns: ['../../**/*.rs']
		}),

		// In dev mode, call `npm run start` once
		// the bundle has been generated
		!production && serve(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser()
	],
	watch: {
		clearScreen: false
	}
};
