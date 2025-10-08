<script>
  import { authStore } from '../stores/auth.js'
  import { url } from '@roxi/routify'

  function handleLogout() {
    authStore.logout()
    window.location.href = '/'
  }

  function safeUrl(path) {
    try {
      return $url(path)
    } catch (e) {
      return path
    }
  }
</script>

<nav class="navbar">
  <div class="nav-container">
    <a href={safeUrl('/')} class="nav-brand">Berlin DanceMode</a>
    
    <div class="nav-menu">
      <a href={safeUrl('/')}>Home</a>
      <a href={safeUrl('/events')}>Events</a>
      <a href={safeUrl('/packages')}>Packages</a>
      <a href={safeUrl('/venues')}>Venues</a>
      <a href={safeUrl('/about')}>About</a>
      
      {#if $authStore.isAuthenticated}
        {#if $authStore.isCreator}
          <a href={safeUrl('/creator')} class="creator-link">Creator Area</a>
        {/if}
        <a href={safeUrl('/dashboard')}>Dashboard</a>
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
        <a href={safeUrl('/login')}>Login</a>
        <a href={safeUrl('/register')}>Register</a>
      {/if}
    </div>
  </div>
</nav>

<style>
  .navbar {
    background: white;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .nav-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .nav-brand {
    font-size: 1.5rem;
    font-weight: bold;
    color: #667eea;
    text-decoration: none;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .nav-menu {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .nav-menu a {
    color: #4a5568;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    transition: all 0.2s ease;
    font-weight: 500;
  }

  .nav-menu a:hover {
    color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }

  .creator-link {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%) !important;
    color: white !important;
    font-weight: 600;
  }

  .creator-link:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
  }

  .user-profile {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 25px;
    color: white;
  }

  .user-avatar {
    position: relative;
  }

  .avatar-circle {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 1rem;
    color: white;
    border: 2px solid rgba(255, 255, 255, 0.3);
  }

  .user-name {
    font-weight: 500;
    font-size: 0.9rem;
    color: white;
  }

  .logout-btn {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
    padding: 0.4rem 0.8rem;
    border-radius: 15px;
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .logout-btn:hover {
    background: rgba(255, 255, 255, 0.3);
    transform: translateY(-1px);
  }

  @media (max-width: 768px) {
    .nav-container {
      padding: 1rem;
      flex-wrap: wrap;
    }

    .nav-menu {
      gap: 0.5rem;
      flex-wrap: wrap;
    }

    .user-profile {
      order: 3;
      margin-top: 0.5rem;
    }
  }
</style>