<script>
  import { Router } from '@roxi/routify'
  import { routes } from '../.routify/routes'
  import NavigationBar from './components/NavigationBar.svelte'
  import { authStore } from './stores/auth.js'
</script>

<div class="app">
  <main>
    <Router {routes} />
  </main>

  <footer>
    <p>&copy; 2025 Berlin DanceMode - Enhanced with Creator Area</p>
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    min-height: 100vh;
  }

  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  main {
    flex: 1;
  }

  footer {
    background: #2d3748;
    color: white;
    padding: 2rem;
    text-align: center;
    margin-top: auto;
  }

  footer p {
    margin: 0;
    font-size: 0.9rem;
  }
</style>