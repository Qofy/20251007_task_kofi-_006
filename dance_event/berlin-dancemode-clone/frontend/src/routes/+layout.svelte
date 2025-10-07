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
          <div class="user-profile">
            <div class="user-avatar">
              <div class="avatar-circle">
                {$authStore.user?.username?.charAt(0).toUpperCase() || 'U'}
              </div>
            </div>
            <span class="user-name">{$authStore.user?.username || 'User'}</span>
            <button class="logout-btn" on:click={handleLogout}>Logout</button>
          </div>
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
  
  .user-profile {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 25px;
    color: white;
  }
  
  .user-avatar {
    position: relative;
  }
  
  .avatar-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 1.1rem;
    color: white;
    border: 2px solid rgba(255, 255, 255, 0.3);
  }
  
  .user-name {
    font-weight: 500;
    font-size: 0.95rem;
    color: white;
  }
  
  .logout-btn {
    background: rgba(255, 255, 255, 0.2) !important;
    color: white !important;
    border: 1px solid rgba(255, 255, 255, 0.3);
    padding: 0.4rem 0.8rem;
    border-radius: 15px;
    font-size: 0.85rem;
    transition: all 0.2s ease;
  }
  
  .logout-btn:hover {
    background: rgba(255, 255, 255, 0.3) !important;
    transform: translateY(-1px);
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
