<script></script><script>

  import { onMount } from 'svelte';  import { onMount } from 'svelte';

  import { page } from '$app/stores';  import { page } from '$app/stores';

  import { authStore, authHelpers } from '$lib/stores.js';  import { authStore, authHelpers } from '$lib/stores.js';

    

  let isMenuOpen = false;  let isMenuOpen = false;

    

  function toggleMenu() {  function toggleMenu() {

    isMenuOpen = !isMenuOpen;    isMenuOpen = !isMenuOpen;

  }  }

    

  function handleLogout() {  function handleLogout() {

    authHelpers.logout();    authHelpers.logout();

    window.location.href = '/';    window.location.href = '/';

  }  }

</script></script>



<svelte:head><svelte:head>

  <meta name="viewport" content="width=device-width, initial-scale=1" />  <meta name="viewport" content="width=device-width, initial-scale=1" />

</svelte:head></svelte:head>



<div class="app"><div class="app">

  <!-- Navigation -->  <!-- Navigation -->

  <nav class="navbar">  <nav class="navbar">

    <div class="nav-container">    <div class="nav-container">

      <div class="nav-brand">      <div class="nav-brand">

        <a href="/">Berlin DanceMode</a>        <a href="/">Berlin DanceMode</a>

      </div>      </div>

            

      <button class="nav-toggle" on:click={toggleMenu}>      <button class="nav-toggle" on:click={toggleMenu}>

        ☰        ☰

      </button>      </button>

            

      <div class="nav-menu" class:active={isMenuOpen}>      <div class="nav-menu" class:active={isMenuOpen}>

        <a href="/" class="nav-link">Home</a>        <a href="/" class="nav-link">Home</a>

        <a href="/events" class="nav-link">Events</a>        <a href="/events" class="nav-link">Events</a>

        <a href="/packages" class="nav-link">Packages</a>        <a href="/packages" class="nav-link">Packages</a>

        <a href="/venues" class="nav-link">Venues</a>        <a href="/venues" class="nav-link">Venues</a>

        <a href="/about" class="nav-link">About</a>        <a href="/about" class="nav-link">About</a>

                

        {#if $authStore.isAuthenticated}        {#if $authStore.isAuthenticated}

          <a href="/dashboard" class="nav-link">Dashboard</a>          <a href="/dashboard" class="nav-link">Dashboard</a>

          <button on:click={handleLogout} class="nav-link logout-btn">Logout</button>          <button on:click={handleLogout} class="nav-link logout-btn">Logout</button>

        {:else}        {:else}

          <a href="/login" class="nav-link">Login</a>          <a href="/login" class="nav-link">Login</a>

          <a href="/register" class="nav-link register-btn">Register</a>          <a href="/register" class="nav-link register-btn">Register</a>

        {/if}        {/if}

      </div>      </div>

    </div>    </div>

  </nav>  </nav>



  <!-- Main Content -->  <!-- Main Content -->

  <main class="main-content">  <main class="main-content">

    <slot />    <slot />

  </main>  </main>



  <!-- Footer -->  <!-- Footer -->

  <footer class="footer">  <footer class="footer">

    <div class="container">    <div class="container">

      <div class="footer-content">      <div class="footer-content">

        <div class="footer-section">        <div class="footer-section\">

          <h3>Berlin DanceMode</h3>          <h3>Berlin DanceMode</h3>

          <p>Discover your passion for dance in the heart of Berlin</p>          <p>Discover your passion for dance in the heart of Berlin</p>

        </div>        </div>

                

        <div class="footer-section">        <div class="footer-section\">

          <h4>Quick Links</h4>          <h4>Quick Links</h4>

          <ul>          <ul>

            <li><a href="/events">Events</a></li>            <li><a href=\"/events\">Events</a></li>

            <li><a href="/packages">Packages</a></li>            <li><a href=\"/packages\">Packages</a></li>

            <li><a href="/venues">Venues</a></li>            <li><a href=\"/venues\">Venues</a></li>

            <li><a href="/about">About</a></li>            <li><a href=\"/about\">About</a></li>

          </ul>          </ul>

        </div>        </div>

                

        <div class="footer-section">        <div class="footer-section\">

          <h4>Contact</h4>          <h4>Contact</h4>

          <p>Email: info@berlindancemode.com</p>          <p>Email: info@berlindancemode.com</p>

          <p>Phone: +49 30 12345678</p>          <p>Phone: +49 30 12345678</p>

        </div>        </div>

      </div>      </div>

            

      <div class="footer-bottom">      <div class="footer-bottom\">

        <p>&copy; 2025 Berlin DanceMode. All rights reserved.</p>        <p>&copy; 2025 Berlin DanceMode. All rights reserved.</p>

      </div>      </div>

    </div>    </div>

  </footer>  </footer>

</div></div>



<style><style>

  :global(body) {  :global(body) {

    margin: 0;    margin: 0;

    padding: 0;    padding: 0;

    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;

    line-height: 1.6;    line-height: 1.6;

  }  }

    

  :global(*) {  :global(*) {

    box-sizing: border-box;    box-sizing: border-box;

  }  }

    

  .app {  .app {

    min-height: 100vh;    min-height: 100vh;

    display: flex;    display: flex;

    flex-direction: column;    flex-direction: column;

  }  }

    

  .navbar {  .navbar {

    background: #fff;    background: #fff;

    box-shadow: 0 2px 10px rgba(0,0,0,0.1);    box-shadow: 0 2px 10px rgba(0,0,0,0.1);

    position: sticky;    position: sticky;

    top: 0;    top: 0;

    z-index: 100;    z-index: 100;

  }  }

    

  .nav-container {  .nav-container {

    max-width: 1200px;    max-width: 1200px;

    margin: 0 auto;    margin: 0 auto;

    padding: 0 1rem;    padding: 0 1rem;

    display: flex;    display: flex;

    justify-content: space-between;    justify-content: space-between;

    align-items: center;    align-items: center;

    height: 70px;    height: 70px;

  }  }

    

  .nav-brand a {  .nav-brand a {

    font-size: 1.5rem;    font-size: 1.5rem;

    font-weight: bold;    font-weight: bold;

    color: #667eea;    color: #667eea;

    text-decoration: none;    text-decoration: none;

  }  }

    

  .nav-toggle {  .nav-toggle {

    display: none;    display: none;

    background: none;    background: none;

    border: none;    border: none;

    font-size: 1.5rem;    font-size: 1.5rem;

    cursor: pointer;    cursor: pointer;

  }  }

    

  .nav-menu {  .nav-menu {

    display: flex;    display: flex;

    gap: 2rem;    gap: 2rem;

    align-items: center;    align-items: center;

  }  }

    

  .nav-link {  .nav-link {

    color: #333;    color: #333;

    text-decoration: none;    text-decoration: none;

    font-weight: 500;    font-weight: 500;

    transition: color 0.3s ease;    transition: color 0.3s ease;

  }  }

    

  .nav-link:hover {  .nav-link:hover {

    color: #667eea;    color: #667eea;

  }  }

    

  .logout-btn {  .logout-btn {

    background: none;    background: none;

    border: none;    border: none;

    cursor: pointer;    cursor: pointer;

    font: inherit;    font: inherit;

  }  }

    

  .register-btn {  .register-btn {

    background: #667eea;    background: #667eea;

    color: white !important;    color: white !important;

    padding: 0.5rem 1rem;    padding: 0.5rem 1rem;

    border-radius: 6px;    border-radius: 6px;

  }  }

    

  .main-content {  .main-content {

    flex: 1;    flex: 1;

  }  }

    

  .footer {  .footer {

    background: #333;    background: #333;

    color: white;    color: white;

    padding: 3rem 0 1rem;    padding: 3rem 0 1rem;

  }  }

    

  .container {  .container {

    max-width: 1200px;    max-width: 1200px;

    margin: 0 auto;    margin: 0 auto;

    padding: 0 1rem;    padding: 0 1rem;

  }  }

    

  .footer-content {  .footer-content {

    display: grid;    display: grid;

    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));

    gap: 2rem;    gap: 2rem;

    margin-bottom: 2rem;    margin-bottom: 2rem;

  }  }

    

  .footer-section h3,  .footer-section h3,

  .footer-section h4 {  .footer-section h4 {

    margin-bottom: 1rem;    margin-bottom: 1rem;

  }  }

    

  .footer-section ul {  .footer-section ul {

    list-style: none;    list-style: none;

    padding: 0;    padding: 0;

  }  }

    

  .footer-section li {  .footer-section li {

    margin-bottom: 0.5rem;    margin-bottom: 0.5rem;

  }  }

    

  .footer-section a {  .footer-section a {

    color: #ccc;    color: #ccc;

    text-decoration: none;    text-decoration: none;

  }  }

    

  .footer-section a:hover {  .footer-section a:hover {

    color: white;    color: white;

  }  }

    

  .footer-bottom {  .footer-bottom {

    border-top: 1px solid #555;    border-top: 1px solid #555;

    padding-top: 1rem;    padding-top: 1rem;

    text-align: center;    text-align: center;

    color: #ccc;    color: #ccc;

  }  }

    

  @media (max-width: 768px) {  @media (max-width: 768px) {

    .nav-toggle {    .nav-toggle {

      display: block;      display: block;

    }    }

        

    .nav-menu {    .nav-menu {

      display: none;      display: none;

      position: absolute;      position: absolute;

      top: 100%;      top: 100%;

      left: 0;      left: 0;

      width: 100%;      width: 100%;

      background: white;      background: white;

      flex-direction: column;      flex-direction: column;

      padding: 1rem;      padding: 1rem;

      box-shadow: 0 2px 10px rgba(0,0,0,0.1);      box-shadow: 0 2px 10px rgba(0,0,0,0.1);

    }    }

        

    .nav-menu.active {    .nav-menu.active {

      display: flex;      display: flex;

    }    }

        

    .footer-content {    .footer-content {

      grid-template-columns: 1fr;      grid-template-columns: 1fr;

      text-align: center;      text-align: center;

    }    }

  }  }

</style></style>