<script>
  import { authStore, authHelpers } from '$lib/stores.js';
  
  function handleLogout() {
    authHelpers.logout();
    window.location.href = '/';
  }
</script>

<div class="app">
  <nav class="navbar">
    <div class="nav-container">
      <a href="/" class="nav-brand">Berlin DanceMode</a>
      
      <div class="nav-menu">
        <a href="/">Home</a>
        <a href="/events">Events</a>
        <a href="/packages">Packages</a>
        <a href="/venues">Venues</a>
        <a href="/about">About</a>
        
        {#if $authStore.isAuthenticated}
          <a href="/dashboard">Dashboard</a>
          <button on:click={handleLogout}>Logout</button>
        {:else}
          <a href="/login">Login</a>
          <a href="/register">Register</a>
        {/if}
      </div>
    </div>
  </nav>

  <main>
    <slot />
  </main>

  <footer>
    <p>&copy; 2025 Berlin DanceMode</p>
  </footer>
</div>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
  
  .navbar {
    background: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .nav-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .nav-brand {
    font-size: 1.5rem;
    font-weight: bold;
    color: #667eea;
    text-decoration: none;
  }
  
  .nav-menu {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  
  .nav-menu a, .nav-menu button {
    color: #333;
    text-decoration: none;
    padding: 0.5rem 1rem;
    background: none;
    border: none;
    cursor: pointer;
  }
  
  .nav-menu a:hover, .nav-menu button:hover {
    color: #667eea;
  }
  
  main {
    flex: 1;
  }
  
  footer {
    background: #333;
    color: white;
    padding: 2rem;
    text-align: center;
  }
</style>
