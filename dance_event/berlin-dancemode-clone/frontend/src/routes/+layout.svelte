<script>
  import '../lib/styles/global.css';
  import { navigationStore, authStore } from '$lib/stores.js';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  
  let isMenuOpen = false;
  let user = null;
  let isAuthenticated = false;
  
  // Subscribe to stores
  navigationStore.subscribe(nav => {
    isMenuOpen = nav.isMenuOpen;
  });
  
  authStore.subscribe(auth => {
    user = auth.user;
    isAuthenticated = auth.isAuthenticated;
  });
  
  onMount(() => {
    // Update current page in navigation store
    const unsubscribe = page.subscribe(($page) => {
      navigationStore.update(nav => ({
        ...nav,
        currentPage: $page.route?.id || 'home'
      }));
    });
    
    return unsubscribe;
  });
  
  function toggleMenu() {
    navigationStore.update(nav => ({
      ...nav,
      isMenuOpen: !nav.isMenuOpen
    }));
  }
  
  function closeMenu() {
    navigationStore.update(nav => ({
      ...nav,
      isMenuOpen: false
    }));
  }
</script>

<div class="app">
  <!-- Navigation Header -->
  <nav class="navbar">
    <div class="container">
      <div class="nav-container">
        <!-- Logo -->
        <a href="/" class="logo" on:click={closeMenu}>
          <span class="logo-text">Berlin DanceMode</span>
        </a>
        
        <!-- Desktop Navigation -->
        <div class="nav-links desktop-only">
          <a href="/" class="nav-link">Home</a>
          <a href="/events" class="nav-link">Events</a>
          <a href="/packages" class="nav-link">Packages</a>
          <a href="/venues" class="nav-link">Venues</a>
          <a href="/about" class="nav-link">About</a>
        </div>
        
        <!-- Auth Links -->
        <div class="auth-links desktop-only">
          {#if isAuthenticated && user}
            <a href="/profile" class="nav-link">Hello, {user.first_name}</a>
            <a href="/dashboard" class="btn btn-outline btn-sm">Dashboard</a>
          {:else}
            <a href="/login" class="nav-link">Login</a>
            <a href="/register" class="btn btn-primary btn-sm">Sign Up</a>
          {/if}
        </div>
        
        <!-- Mobile Menu Button -->
        <button class="mobile-menu-btn mobile-only" on:click={toggleMenu}>
          <span class="hamburger" class:active={isMenuOpen}></span>
        </button>
      </div>
    </div>
    
    <!-- Mobile Navigation Menu -->
    {#if isMenuOpen}
      <div class="mobile-menu">
        <div class="mobile-menu-content">
          <a href="/" class="mobile-nav-link" on:click={closeMenu}>Home</a>
          <a href="/events" class="mobile-nav-link" on:click={closeMenu}>Events</a>
          <a href="/packages" class="mobile-nav-link" on:click={closeMenu}>Packages</a>
          <a href="/venues" class="mobile-nav-link" on:click={closeMenu}>Venues</a>
          <a href="/about" class="mobile-nav-link" on:click={closeMenu}>About</a>
          
          <div class="mobile-auth-links">
            {#if isAuthenticated && user}
              <a href="/profile" class="mobile-nav-link" on:click={closeMenu}>Profile</a>
              <a href="/dashboard" class="mobile-nav-link" on:click={closeMenu}>Dashboard</a>
            {:else}
              <a href="/login" class="mobile-nav-link" on:click={closeMenu}>Login</a>
              <a href="/register" class="mobile-nav-link" on:click={closeMenu}>Sign Up</a>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </nav>
  
  <!-- Main Content -->
  <main class="main-content">
    <slot />
  </main>
  
  <!-- Footer -->
  <footer class="footer">
    <div class="container">
      <div class="footer-content">
        <div class="footer-section">
          <h3>Berlin DanceMode</h3>
          <p>Experience Blues & Fusion dancing, learning, community and selfcare in Berlin.</p>
        </div>
        
        <div class="footer-section">
          <h4>Quick Links</h4>
          <ul class="footer-links">
            <li><a href="/events">Events</a></li>
            <li><a href="/packages">Packages</a></li>
            <li><a href="/venues">Venues</a></li>
            <li><a href="/about">About</a></li>
          </ul>
        </div>
        
        <div class="footer-section">
          <h4>Contact</h4>
          <ul class="footer-links">
            <li><a href="/contact">Contact Us</a></li>
            <li><a href="/terms">Terms & Conditions</a></li>
            <li><a href="/privacy">Privacy Policy</a></li>
          </ul>
        </div>
        
        <div class="footer-section">
          <h4>Follow Us</h4>
          <div class="social-links">
            <a href="#" class="social-link">Facebook</a>
            <a href="#" class="social-link">Instagram</a>
            <a href="#" class="social-link">Twitter</a>
          </div>
        </div>
      </div>
      
      <div class="footer-bottom">
        <p>&copy; 2025 Berlin DanceMode Clone - Educational Project</p>
      </div>
    </div>
  </footer>
</div>

<style>
  /* Navigation Styles */
  .navbar {
    background-color: var(--bg-primary);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 1000;
    box-shadow: var(--shadow-sm);
  }
  
  .nav-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md) 0;
  }
  
  .logo {
    font-family: 'Poppins', sans-serif;
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--primary-color);
    text-decoration: none;
  }
  
  .logo:hover {
    color: var(--primary-dark);
  }
  
  .nav-links {
    display: flex;
    gap: var(--spacing-xl);
    align-items: center;
  }
  
  .nav-link {
    color: var(--text-secondary);
    font-weight: 500;
    transition: color 0.2s ease;
  }
  
  .nav-link:hover {
    color: var(--primary-color);
  }
  
  .auth-links {
    display: flex;
    gap: var(--spacing-md);
    align-items: center;
  }
  
  /* Mobile Menu */
  .mobile-menu-btn {
    display: none;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 40px;
    height: 40px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }
  
  .hamburger {
    width: 24px;
    height: 2px;
    background-color: var(--text-primary);
    transition: all 0.3s ease;
    position: relative;
  }
  
  .hamburger::before,
  .hamburger::after {
    content: '';
    position: absolute;
    width: 24px;
    height: 2px;
    background-color: var(--text-primary);
    transition: all 0.3s ease;
  }
  
  .hamburger::before {
    top: -8px;
  }
  
  .hamburger::after {
    bottom: -8px;
  }
  
  .hamburger.active {
    background-color: transparent;
  }
  
  .hamburger.active::before {
    transform: rotate(45deg);
    top: 0;
  }
  
  .hamburger.active::after {
    transform: rotate(-45deg);
    bottom: 0;
  }
  
  .mobile-menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background-color: var(--bg-primary);
    border-bottom: 1px solid var(--border-color);
    box-shadow: var(--shadow-lg);
  }
  
  .mobile-menu-content {
    padding: var(--spacing-lg) var(--spacing-md);
  }
  
  .mobile-nav-link {
    display: block;
    padding: var(--spacing-sm) 0;
    color: var(--text-secondary);
    font-weight: 500;
    border-bottom: 1px solid var(--border-light);
  }
  
  .mobile-nav-link:hover {
    color: var(--primary-color);
  }
  
  .mobile-auth-links {
    margin-top: var(--spacing-lg);
    padding-top: var(--spacing-lg);
    border-top: 1px solid var(--border-color);
  }
  
  /* Main Content */
  .main-content {
    min-height: calc(100vh - 200px);
  }
  
  /* Footer */
  .footer {
    background-color: var(--bg-dark);
    color: white;
    margin-top: var(--spacing-3xl);
  }
  
  .footer-content {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: var(--spacing-xl);
    padding: var(--spacing-3xl) 0 var(--spacing-xl) 0;
  }
  
  .footer-section h3 {
    color: white;
    margin-bottom: var(--spacing-md);
  }
  
  .footer-section h4 {
    color: white;
    font-size: 1.125rem;
    margin-bottom: var(--spacing-md);
  }
  
  .footer-section p {
    color: #d1d5db;
    line-height: 1.6;
  }
  
  .footer-links {
    list-style: none;
  }
  
  .footer-links li {
    margin-bottom: var(--spacing-sm);
  }
  
  .footer-links a {
    color: #d1d5db;
    transition: color 0.2s ease;
  }
  
  .footer-links a:hover {
    color: white;
  }
  
  .social-links {
    display: flex;
    gap: var(--spacing-md);
  }
  
  .social-link {
    color: #d1d5db;
    transition: color 0.2s ease;
  }
  
  .social-link:hover {
    color: white;
  }
  
  .footer-bottom {
    border-top: 1px solid #374151;
    padding: var(--spacing-lg) 0;
    text-align: center;
  }
  
  .footer-bottom p {
    color: #9ca3af;
    margin: 0;
  }
  
  /* Responsive Design */
  .desktop-only {
    display: flex;
  }
  
  .mobile-only {
    display: none;
  }
  
  @media (max-width: 768px) {
    .desktop-only {
      display: none;
    }
    
    .mobile-only {
      display: flex;
    }
    
    .footer-content {
      grid-template-columns: 1fr;
      gap: var(--spacing-lg);
    }
  }
</style>