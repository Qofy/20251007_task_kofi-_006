<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'
  import { url } from '@roxi/routify'

  let events = []
  let packages = []
  let venues = []
  let loading = true

  onMount(async () => {
    try {
      const [eventsData, packagesData, venuesData] = await Promise.all([
        api.events.getAll(),
        api.packages.getAll(),
        api.venues.getAll()
      ])
      
      events = eventsData.data || []
      packages = packagesData.data || []
      venues = venuesData.data || []
    } catch (error) {
      console.error('Error loading home data:', error)
    } finally {
      loading = false
    }
  })
</script>

<main>
  <!-- Hero Section -->
  <section class="hero">
    <div class="hero-content">
      <h1>Welcome to Berlin DanceMode</h1>
      <p>Discover the hottest dance events, exclusive packages, and amazing venues in Berlin's vibrant nightlife scene.</p>
      
      {#if $authStore.isAuthenticated}
        <div class="hero-actions">
          <a href={$url('/dashboard')} class="btn-primary">Go to Dashboard</a>
          {#if $authStore.isCreator}
            <a href={$url('/creator')} class="btn-secondary">Creator Area</a>
          {/if}
        </div>
      {:else}
        <div class="hero-actions">
          <a href={$url('/register')} class="btn-primary">Join Now</a>
          <a href={$url('/login')} class="btn-secondary">Login</a>
        </div>
      {/if}
    </div>
    <div class="hero-visual">
      <div class="hero-gradient"></div>
    </div>
  </section>

  <!-- Featured Events -->
  <section class="featured-section">
    <div class="container">
      <h2>Featured Events</h2>
      {#if loading}
        <div class="loading">Loading events...</div>
      {:else if events.length > 0}
        <div class="events-grid">
          {#each events as event}
            <div class="event-card">
              <div class="event-image">
                <div class="event-date">
                  <span class="day">{new Date(event.date).getDate()}</span>
                  <span class="month">{new Date(event.date).toLocaleDateString('en', { month: 'short' })}</span>
                </div>
              </div>
              <div class="event-info">
                <h3>{event.name}</h3>
                <p class="venue">{event.venue}</p>
                <p class="description">{event.description.substring(0, 100)}...</p>
                <div class="event-meta">
                  <span class="price">€{event.price}</span>
                  <span class="time">{new Date(event.date).toLocaleTimeString('en', { hour: '2-digit', minute: '2-digit' })}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
        <div class="section-footer">
          <a href={$url('/events')} class="btn-outline">View All Events</a>
        </div>
      {:else}
        <div class="empty-state">
          <p>No events available at the moment.</p>
        </div>
      {/if}
    </div>
  </section>

  <!-- Featured Packages -->
  <section class="featured-section packages-section">
    <div class="container">
      <h2>Exclusive Packages</h2>
      {#if loading}
        <div class="loading">Loading packages...</div>
      {:else if packages.length > 0}
        <div class="packages-grid">
          {#each packages as pkg}
            <div class="package-card">
              <div class="package-header">
                <h3>{pkg.name}</h3>
                <div class="package-price">€{pkg.price}</div>
              </div>
              <div class="package-content">
                <p>{pkg.description}</p>
                <ul class="package-features">
                  {#each pkg.features || [] as feature}
                    <li>{feature}</li>
                  {/each}
                </ul>
              </div>
              <div class="package-footer">
                <button class="btn-primary">Select Package</button>
              </div>
            </div>
          {/each}
        </div>
        <div class="section-footer">
          <a href={$url('/packages')} class="btn-outline">View All Packages</a>
        </div>
      {:else}
        <div class="empty-state">
          <p>No packages available at the moment.</p>
        </div>
      {/if}
    </div>
  </section>

  <!-- Featured Venues -->
  <section class="featured-section">
    <div class="container">
      <h2>Popular Venues</h2>
      {#if loading}
        <div class="loading">Loading venues...</div>
      {:else if venues.length > 0}
        <div class="venues-grid">
          {#each venues as venue}
            <div class="venue-card">
              <div class="venue-info">
                <h3>{venue.name}</h3>
                <p class="venue-address">{venue.address}</p>
                <p class="venue-description">{venue.description.substring(0, 120)}...</p>
                <div class="venue-meta">
                  <span class="capacity">Capacity: {venue.capacity}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
        <div class="section-footer">
          <a href={$url('/venues')} class="btn-outline">View All Venues</a>
        </div>
      {:else}
        <div class="empty-state">
          <p>No venues available at the moment.</p>
        </div>
      {/if}
    </div>
  </section>
</main>

<style>
  main {
    min-height: 100vh;
  }

  .hero {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4rem 2rem;
    display: flex;
    align-items: center;
    min-height: 60vh;
    position: relative;
    overflow: hidden;
  }

  .hero-content {
    max-width: 1200px;
    margin: 0 auto;
    z-index: 2;
  }

  .hero h1 {
    font-size: 3.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
    line-height: 1.2;
  }

  .hero p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
    opacity: 0.9;
    max-width: 600px;
  }

  .hero-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .hero-visual {
    position: absolute;
    top: 0;
    right: 0;
    width: 50%;
    height: 100%;
    background: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.1'%3E%3Ccircle cx='30' cy='30' r='30'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E") repeat;
  }

  .hero-gradient {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(45deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%);
  }

  .featured-section {
    padding: 4rem 2rem;
  }

  .packages-section {
    background: #f8fafc;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
  }

  .featured-section h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: #2d3748;
  }

  .loading, .empty-state {
    text-align: center;
    padding: 2rem;
    color: #718096;
  }

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
  }

  .event-card {
    background: white;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    cursor: pointer;
  }

  .event-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  }

  .event-image {
    height: 200px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .event-date {
    background: white;
    border-radius: 8px;
    padding: 0.5rem;
    text-align: center;
    color: #2d3748;
    font-weight: bold;
    position: absolute;
    top: 1rem;
    right: 1rem;
  }

  .event-date .day {
    display: block;
    font-size: 1.5rem;
  }

  .event-date .month {
    display: block;
    font-size: 0.8rem;
    text-transform: uppercase;
  }

  .event-info {
    padding: 1.5rem;
  }

  .event-info h3 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: #2d3748;
  }

  .venue {
    color: #667eea;
    font-weight: 500;
    margin-bottom: 0.5rem;
  }

  .description {
    color: #718096;
    line-height: 1.5;
    margin-bottom: 1rem;
  }

  .event-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .price {
    font-size: 1.25rem;
    font-weight: bold;
    color: #667eea;
  }

  .time {
    color: #718096;
    font-size: 0.9rem;
  }

  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .package-card {
    background: white;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease;
  }

  .package-card:hover {
    transform: translateY(-2px);
  }

  .package-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .package-header h3 {
    font-size: 1.25rem;
    font-weight: bold;
  }

  .package-price {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .package-content {
    padding: 1.5rem;
  }

  .package-features {
    list-style: none;
    padding: 0;
    margin: 1rem 0 0 0;
  }

  .package-features li {
    padding: 0.25rem 0;
    color: #4a5568;
  }

  .package-features li:before {
    content: "✓";
    color: #667eea;
    font-weight: bold;
    margin-right: 0.5rem;
  }

  .package-footer {
    padding: 0 1.5rem 1.5rem 1.5rem;
  }

  .venues-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
  }

  .venue-card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease;
  }

  .venue-card:hover {
    transform: translateY(-2px);
  }

  .venue-info h3 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: #2d3748;
  }

  .venue-address {
    color: #667eea;
    font-weight: 500;
    margin-bottom: 1rem;
  }

  .venue-description {
    color: #718096;
    line-height: 1.5;
    margin-bottom: 1rem;
  }

  .venue-meta {
    color: #4a5568;
    font-size: 0.9rem;
  }

  .capacity {
    font-weight: 500;
  }

  .section-footer {
    text-align: center;
    margin-top: 3rem;
  }

  .btn-primary, .btn-secondary, .btn-outline {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s ease;
    border: none;
    cursor: pointer;
    font-size: 1rem;
  }

  .btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 2px solid rgba(255, 255, 255, 0.3);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.3);
    transform: translateY(-2px);
  }

  .btn-outline {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .btn-outline:hover {
    background: #667eea;
    color: white;
    transform: translateY(-2px);
  }

  @media (max-width: 768px) {
    .hero h1 {
      font-size: 2.5rem;
    }

    .hero p {
      font-size: 1rem;
    }

    .hero-actions {
      justify-content: center;
    }

    .featured-section {
      padding: 2rem 1rem;
    }

    .featured-section h2 {
      font-size: 2rem;
    }

    .events-grid, .packages-grid, .venues-grid {
      grid-template-columns: 1fr;
    }
  }
</style>