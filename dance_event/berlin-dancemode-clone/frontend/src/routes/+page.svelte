<script>
  import { onMount } from 'svelte';
  import { eventsAPI, packagesAPI, venuesAPI } from '$lib/api.js';
  
  let events = [];
  let packages = [];
  let venues = [];
  let isLoading = true;
  
  onMount(async () => {
    try {
      const [eventsRes, packagesRes, venuesRes] = await Promise.all([
        eventsAPI.getAll(),
        packagesAPI.getAll(), 
        venuesAPI.getAll()
      ]);
      
      if (eventsRes.success) events = eventsRes.data.slice(0, 3);
      if (packagesRes.success) packages = packagesRes.data.slice(0, 3);
      if (venuesRes.success) venues = venuesRes.data.slice(0, 2);
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      isLoading = false;
    }
  });
</script>

<svelte:head>
  <title>Berlin DanceMode - Discover Your Dance Journey</title>
  <meta name="description" content="Experience the vibrant dance scene of Berlin with our premium dance events, workshops, and community." />
</svelte:head>

<div class="home-page">
  <!-- Hero Section -->
  <section class="hero">
    <div class="hero-content">
      <h1>Welcome to Berlin DanceMode</h1>
      <p>Discover your passion for dance in the heart of Berlin</p>
      <div class="hero-actions">
        <a href="/events" class="btn btn-primary">Explore Events</a>
        <a href="/packages" class="btn btn-secondary">View Packages</a>
      </div>
    </div>
  </section>

  <div class="container">
    {#if isLoading}
      <div class="loading">Loading...</div>
    {:else}
      <!-- Featured Events -->
      <section class="featured-events">
        <h2>Upcoming Events</h2>
        <div class="events-grid">
          {#each events as event}
            <div class="event-card">
              <h3>{event.title}</h3>
              <p>{event.description}</p>
              <a href="/events/{event.id}" class="btn btn-outline">Learn More</a>
            </div>
          {/each}
        </div>
      </section>

      <!-- Featured Packages -->
      <section class="featured-packages">
        <h2>Dance Packages</h2>
        <div class="packages-grid">
          {#each packages as pkg}
            <div class="package-card">
              <h3>{pkg.name}</h3>
              <p class="price">€{pkg.price}</p>
              <p>{pkg.description}</p>
              <a href="/packages" class="btn btn-primary">Choose Package</a>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  </div>
</div>

<style>
  .hero {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 100px 0;
    text-align: center;
  }
  
  .hero-content h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }
  
  .hero-actions {
    margin-top: 2rem;
    display: flex;
    gap: 1rem;
    justify-content: center;
  }
  
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .events-grid, .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }
  
  .event-card, .package-card {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
  }
  
  .btn {
    display: inline-block;
    padding: 12px 24px;
    border-radius: 6px;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.3s ease;
  }
  
  .btn-primary {
    background: #667eea;
    color: white;
  }
  
  .btn-secondary {
    background: transparent;
    color: white;
    border: 2px solid white;
  }
  
  .btn-outline {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }
  
  .loading {
    text-align: center;
    padding: 4rem;
  }
</style>
