<script>
	import Router from 'svelte-spa-router';
	import Navigation from './components/Navigation.svelte';
	
	import Home from './pages/Home.svelte';
	import Events from './pages/Events.svelte';
	import Venues from './pages/Venues.svelte';
	import Packages from './pages/Packages.svelte';
	import About from './pages/About.svelte';
	import Login from './pages/Login.svelte';
	import Register from './pages/Register.svelte';
	
	const routes = {
		'/': Home,
		'/events': Events,
		'/venues': Venues,
		'/packages': Packages,
		'/about': About,
		'/login': Login,
		'/register': Register
	};
</script>

<Navigation />
<main>
	<Router {routes} />
</main>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		background: #f8fafc;
	}
	
	:global(*) {
		box-sizing: border-box;
	}
	
	main {
		min-height: 100vh;
	}
</style>