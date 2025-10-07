import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 5173,
		host: '127.0.0.1'
	},
	define: {
		// API base URL for backend communication
		API_BASE_URL: JSON.stringify(process.env.API_BASE_URL || 'http://127.0.0.1:8080')
	}
});
