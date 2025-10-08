<script>
  let isMenuOpen = false;
  
  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }
</script>

<nav class="navbar">
  <div class="nav-container">
    <a href="#/" class="nav-brand">
      <span class="brand-text">Berlin DanceMode</span>
    </a>
    
    <button class="nav-toggle" on:click={toggleMenu} class:active={isMenuOpen} aria-label="Toggle menu">
      <span></span>
      <span></span>
      <span></span>
    </button>
    
    <div class="nav-menu" class:active={isMenuOpen}>
      <a href="#/" class="nav-link">Home</a>
      <a href="#/events" class="nav-link">Events</a>
      <a href="#/venues" class="nav-link">Venues</a>
      <a href="#/packages" class="nav-link">Packages</a>
      <a href="#/about" class="nav-link">About</a>
      <a href="#/login" class="nav-link nav-cta">Login</a>
      <a href="#/register" class="nav-link nav-cta-primary">Register</a>
    </div>
  </div>
</nav>

<style>
  .navbar {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid rgba(226, 232, 240, 0.8);
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
    padding: 1rem 0;
  }

  .nav-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .nav-brand {
    text-decoration: none;
    color: #1a202c;
    font-weight: 700;
    font-size: 1.25rem;
  }

  .brand-text {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .nav-toggle {
    display: none;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    flex-direction: column;
    gap: 0.25rem;
  }

  .nav-toggle span {
    width: 25px;
    height: 3px;
    background: #1a202c;
    transition: 0.3s;
  }

  .nav-toggle.active span:nth-child(1) {
    transform: rotate(-45deg) translate(-5px, 6px);
  }

  .nav-toggle.active span:nth-child(2) {
    opacity: 0;
  }

  .nav-toggle.active span:nth-child(3) {
    transform: rotate(45deg) translate(-5px, -6px);
  }

  .nav-menu {
    display: flex;
    align-items: center;
    gap: 2rem;
  }

  .nav-link {
    text-decoration: none;
    color: #4a5568;
    font-weight: 500;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    transition: all 0.3s ease;
  }

  .nav-link:hover {
    color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }

  .nav-cta {
    border: 1px solid #667eea;
    color: #667eea;
  }

  .nav-cta:hover {
    background: #667eea;
    color: white;
  }

  .nav-cta-primary {
    background: #667eea;
    color: white;
  }

  .nav-cta-primary:hover {
    background: #5a67d8;
  }

  @media (max-width: 768px) {
    .nav-toggle {
      display: flex;
    }

    .nav-menu {
      position: fixed;
      top: 100%;
      left: 0;
      right: 0;
      background: white;
      flex-direction: column;
      gap: 0;
      padding: 2rem;
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      transform: translateY(-100%);
      opacity: 0;
      visibility: hidden;
      transition: all 0.3s ease;
    }

    .nav-menu.active {
      transform: translateY(0);
      opacity: 1;
      visibility: visible;
    }

    .nav-link {
      width: 100%;
      text-align: center;
      padding: 1rem;
      margin: 0.25rem 0;
    }
  }
</style>