import { createApp } from './app.js';
import { registerSW } from 'virtual:pwa-register';

createApp(document.getElementById('app'));
registerSW();
