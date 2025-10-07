<script>
  import { onMount } from 'svelte';
  import { eventsAPI, packagesAPI } from '$lib/api.js';
  import { eventsStore, packagesStore, loadingStore, errorStore } from '$lib/stores.js';
  
  let events = [];
  let packages = [];
  let isLoading = true;
  let error = null;
  
  onMount(async () => {
    try {
      // Load events and packages
      const [eventsResponse, packagesResponse] = await Promise.all([
        eventsAPI.getAll(),
        packagesAPI.getAll()
      ]);
      
      if (eventsResponse.success) {
        events = eventsResponse.data.slice(0, 3); // Show only first 3 events
        eventsStore.set(eventsResponse.data);
      }
      
      if (packagesResponse.success) {
        packages = packagesResponse.data;
        packagesStore.set(packagesResponse.data);
      }
    } catch (err) {
      error = 'Failed to load content. Please try again later.';
      errorStore.set(error);
      console.error('Error loading home page data:', err);
    } finally {
      isLoading = false;
    }
  });
  
  function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { 
      month: 'long', 
      day: 'numeric', 
      year: 'numeric' 
    });
  }
  
  function formatPrice(price) {
    return new Intl.NumberFormat('de-DE', {
      style: 'currency',
      currency: 'EUR'
    }).format(price);
  }
</script>

<svelte:head>
  <title>Berlin DanceMode - Blues & Fusion Dancing Experience</title>
  <meta name="description" content="Experience Blues & Fusion dancing, learning, community and selfcare in Berlin. Join our events, workshops, and immersions." />
</svelte:head>

<!-- Hero Section -->
<section class="hero">
  <div class="hero-content">
    <div class="container">
      <div class="hero-text">
        <h1 class="hero-title">Berlin DanceMode Experience</h1>
        <p class="hero-subtitle">
          Focusses on Blues & Fusion dancing, learning, community and selfcare. 
          All this is happening in Berlin for dancers and music lovers.
        </p>
        <div class="hero-buttons">
          <a href="/events" class="btn btn-primary btn-lg">Discover Events</a>
          <a href="/packages" class="btn btn-outline btn-lg">View Packages</a>
        </div>
      </div>
    </div>
  </div>
  <div class="hero-background"></div>
</section>

<!-- Upcoming Events Section -->
<section class="section">
  <div class="container">
    <div class="section-header">
      <h2>Upcoming Events</h2>
      <p>Join us for unforgettable dance experiences in Berlin</p>
    </div>
    
    {#if isLoading}
      <div class="loading">Loading events...</div>
    {:else if error}
      <div class="error">
        <p>{error}</p>
        <button class="btn btn-primary" on:click={() => window.location.reload()}>
          Try Again
        </button>
      </div>
    {:else if events.length === 0}
      <div class="empty-state">
        <p>No upcoming events at the moment. Check back soon!</p>
      </div>
    {:else}
      <div class="events-grid">
        {#each events as event (event.id)}
          <div class="event-card card">
            <div class="card-header">
              <div class="event-type">{event.event_type}</div>
              <div class="event-date">
                {formatDate(event.start_date)}
              </div>
            </div>
            <div class="card-body">
              <h3 class="event-title">{event.title}</h3>
              <p class="event-description">{event.description}</p>
              <div class="event-details">
                <div class="event-price">{formatPrice(event.price)}</div>
                <div class="event-participants">
                  {event.current_participants}/{event.max_participants} participants
                </div>
              </div>
            </div>
            <div class="card-footer">
              <a href="/events/{event.id}" class="btn btn-primary">Learn More</a>
            </div>
          </div>
        {/each}
      </div>
      
      <div class="section-footer">
        <a href="/events" class="btn btn-outline">View All Events</a>
      </div>
    {/if}
  </div>
</section>

<!-- Dance Packages Section -->
<section class="section bg-secondary">
  <div class="container">
    <div class="section-header">
      <h2>Dance Packages</h2>
      <p>Choose the perfect package for your dance journey</p>
    </div>
    
    {#if packages.length > 0}
      <div class="packages-grid">
        {#each packages as package (package.id)}
          <div class="package-card card">
            <div class="card-body">
              <h3 class="package-name">{package.name}</h3>
              <div class="package-price">{formatPrice(package.price)}</div>
              <p class="package-description">{package.description}</p>
              <ul class="package-features">
                <li>{package.duration_days} day{package.duration_days > 1 ? 's' : ''} of instruction</li>
                <li>Max {package.max_participants} participants</li>
                <li>All skill levels welcome</li>
                <li>Community support</li>
              </ul>
            </div>
            <div class="card-footer">
              <a href="/packages" class="btn btn-primary">Choose Package</a>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</section>

<!-- About Section -->
<section class="section">
  <div class="container">
    <div class="about-content">
      <div class="about-text">
        <h2>About Berlin DanceMode</h2>
        <p>
          Berlin DanceMode Experience is a vibrant community dedicated to Blues & Fusion dancing. 
          We believe in the power of dance to bring people together, foster creativity, and promote personal growth.
        </p>
        <p>
          Our events range from beginner-friendly workshops to intensive immersions for experienced dancers. 
          We focus not just on technique, but also on building community, encouraging self-expression, and creating a supportive environment for all dancers.
        </p>
        <div class="about-features">
          <div class="feature">
            <h4>Community Focused</h4>
            <p>Building connections through dance</p>
          </div>
          <div class="feature">
            <h4>All Levels</h4>
            <p>From beginners to professionals</p>
          </div>
          <div class="feature">
            <h4>Self Care</h4>
            <p>Mindful movement and personal growth</p>
          </div>
        </div>
        <a href="/about" class="btn btn-outline">Learn More About Us</a>
      </div>
    </div>
  </div>
</section>

<style>
  /* Hero Section */
  .hero {
    position: relative;
    min-height: 70vh;
    display: flex;
    align-items: center;
    color: white;
    overflow: hidden;
  }
  
  .hero-background {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
    z-index: -1;
  }
  
  .hero-background::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image: var(--hero-pattern);
    opacity: 0.1;
  }
  
  .hero-content {
    position: relative;
    z-index: 1;
  }
  
  .hero-title {
    font-size: 3.5rem;
    font-weight: 800;
    margin-bottom: var(--spacing-md);
    line-height: 1.1;
  }
  
  .hero-subtitle {
    font-size: 1.25rem;
    margin-bottom: var(--spacing-xl);
    max-width: 600px;
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.9);
  }
  
  .hero-buttons {
    display: flex;
    gap: var(--spacing-md);
    flex-wrap: wrap;
  }
  
  /* Section Styles */
  .section {
    padding: var(--spacing-3xl) 0;
  }
  
  .bg-secondary {
    background-color: var(--bg-secondary);
  }
  
  .section-header {
    text-align: center;
    margin-bottom: var(--spacing-3xl);
  }
  
  .section-header h2 {
    font-size: 2.5rem;
    margin-bottom: var(--spacing-md);
  }
  
  .section-header p {
    font-size: 1.125rem;
    color: var(--text-secondary);
  }
  
  .section-footer {
    text-align: center;
    margin-top: var(--spacing-xl);
  }
  
  /* Events Grid */
  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: var(--spacing-xl);
  }
  
  .event-card {
    transition: transform 0.2s ease;
  }
  
  .event-card:hover {
    transform: translateY(-5px);
  }
  
  .event-type {
    background-color: var(--accent-color);
    color: white;
    padding: var(--spacing-xs) var(--spacing-sm);
    border-radius: var(--border-radius-sm);
    font-size: 0.875rem;
    font-weight: 500;
    text-transform: uppercase;
  }
  
  .event-date {
    color: var(--text-secondary);
    font-size: 0.875rem;
  }
  
  .event-title {
    margin-bottom: var(--spacing-sm);
  }
  
  .event-description {
    color: var(--text-secondary);
    margin-bottom: var(--spacing-md);
  }
  
  .event-details {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-md);
  }
  
  .event-price {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--primary-color);
  }
  
  .event-participants {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }
  
  /* Packages Grid */
  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: var(--spacing-xl);
  }
  
  .package-card {
    text-align: center;
    transition: transform 0.2s ease;
  }
  
  .package-card:hover {
    transform: translateY(-5px);
  }
  
  .package-name {
    color: var(--primary-color);
    margin-bottom: var(--spacing-md);
  }
  
  .package-price {
    font-size: 2rem;
    font-weight: 700;
    color: var(--secondary-color);
    margin-bottom: var(--spacing-md);
  }
  
  .package-description {
    margin-bottom: var(--spacing-lg);
  }
  
  .package-features {
    list-style: none;
    margin-bottom: var(--spacing-lg);
  }
  
  .package-features li {
    padding: var(--spacing-xs) 0;
    color: var(--text-secondary);
  }
  
  .package-features li::before {
    content: '✓';
    color: var(--primary-color);
    margin-right: var(--spacing-sm);
    font-weight: bold;
  }
  
  /* About Section */
  .about-text {
    max-width: 800px;
    margin: 0 auto;
    text-align: center;
  }
  
  .about-text h2 {
    margin-bottom: var(--spacing-lg);
  }
  
  .about-text p {
    font-size: 1.125rem;
    margin-bottom: var(--spacing-lg);
  }
  
  .about-features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--spacing-xl);
    margin: var(--spacing-3xl) 0;
  }
  
  .feature h4 {
    color: var(--primary-color);
    margin-bottom: var(--spacing-sm);
  }
  
  .feature p {
    font-size: 1rem;
    margin-bottom: 0;
  }
  
  /* Utility Styles */
  .loading, .error, .empty-state {
    text-align: center;
    padding: var(--spacing-3xl);
  }
  
  .error {
    color: var(--text-secondary);
  }
  
  /* Responsive Design */
  @media (max-width: 768px) {
    .hero-title {
      font-size: 2.5rem;
    }
    
    .hero-subtitle {
      font-size: 1.125rem;
    }
    
    .hero-buttons {
      justify-content: center;
    }
    
    .section-header h2 {
      font-size: 2rem;
    }
    
    .events-grid,
    .packages-grid {
      grid-template-columns: 1fr;
    }
    
    .about-features {
      grid-template-columns: 1fr;
      gap: var(--spacing-lg);
    }
  }
</style>
