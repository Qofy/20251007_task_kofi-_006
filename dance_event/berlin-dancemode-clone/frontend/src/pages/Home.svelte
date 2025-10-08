<script>
  import { onMount } from 'svelte';
  import api from '../lib/api.js';
  
  let events = [];
  let packages = [];
  let venues = [];
  let isLoading = true;
  let stats = {
    events: 2500,
    venues: 150,
    visitors: 75000,
    years: 4
  };
  
  onMount(async () => {
    try {
      const [eventsRes, packagesRes, venuesRes] = await Promise.all([
        api.get('/api/events'),
        api.get('/api/packages'), 
        api.get('/api/venues')
      ]);
      
      events = Array.isArray(eventsRes.data) ? eventsRes.data.slice(0, 3) : [];
      packages = Array.isArray(packagesRes.data) ? packagesRes.data.slice(0, 3) : [];
      venues = Array.isArray(venuesRes.data) ? venuesRes.data.slice(0, 3) : [];
    } catch (error) {
      console.error('Error loading data:', error);
      events = [];
      packages = [];
      venues = [];
    } finally {
      isLoading = false;
    }
  });

  function formatPrice(price) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR'
    }).format(price);
  }
</script>

<svelte:head>
  <title>Berlin DanceMode - Authentic Electronic Music Experiences</title>
  <meta name="description" content="Discover Berlin's legendary electronic music scene with curated events, exclusive packages, and insider access to the city's best venues." />
</svelte:head>

<div class="home-page">
  <!-- Hero Section -->
  <section class="hero">
    <div class="hero-overlay"></div>
    <div class="hero-content">
      <h1>Berlin's Electronic Underground</h1>
      <p class="hero-subtitle">Authentic experiences in the world's techno capital</p>
      <p class="hero-description">From legendary clubs to hidden warehouse parties, we unlock the real Berlin electronic music scene</p>
      <div class="hero-actions">
        <a href="/events" class="btn btn-primary">
          <svg class="btn-icon" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
          </svg>
          Explore Events
        </a>
        <a href="/packages" class="btn btn-secondary">
          <svg class="btn-icon" viewBox="0 0 20 20" fill="currentColor">
            <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" />
          </svg>
          View Packages
        </a>
      </div>
    </div>
    
    <!-- Floating Stats -->
    <div class="hero-stats">
      <div class="stat-item">
        <span class="stat-number">{stats.events.toLocaleString()}+</span>
        <span class="stat-label">Events</span>
      </div>
      <div class="stat-item">
        <span class="stat-number">{stats.venues}+</span>
        <span class="stat-label">Venues</span>
      </div>
      <div class="stat-item">
        <span class="stat-number">{stats.visitors.toLocaleString()}+</span>
        <span class="stat-label">Visitors</span>
      </div>
    </div>
  </section>

  <!-- What We Offer Section -->
  <section class="offerings">
    <div class="container">
      <div class="section-header">
        <h2>What We Offer</h2>
        <p>Curated experiences that go beyond typical tourist attractions</p>
      </div>
      
      <div class="offerings-grid">
        <div class="offering-card">
          <div class="offering-icon">🎧</div>
          <h3>Underground Events</h3>
          <p>Access to exclusive warehouse parties, rooftop raves, and intimate club nights that locals keep secret</p>
        </div>
        
        <div class="offering-card">
          <div class="offering-icon">🏭</div>
          <h3>Legendary Venues</h3>
          <p>Skip the lines at world-famous clubs like Berghain, Tresor, and Watergate with our insider connections</p>
        </div>
        
        <div class="offering-card">
          <div class="offering-icon">👥</div>
          <h3>Expert Guides</h3>
          <p>Local scene insiders who know the culture, etiquette, and hidden gems of Berlin's electronic music world</p>
        </div>
        
        <div class="offering-card">
          <div class="offering-icon">🌃</div>
          <h3>Multi-Venue Tours</h3>
          <p>Experience multiple venues in one night, from underground techno bunkers to sophisticated house lounges</p>
        </div>
      </div>
    </div>
  </section>

  <div class="container">
    {#if isLoading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading Berlin's finest...</p>
      </div>
    {:else}
      <!-- Featured Events -->
      <section class="featured-section">
        <div class="section-header">
          <h2>This Week's Highlights</h2>
          <p>Hand-picked events that showcase the best of Berlin's electronic scene</p>
        </div>
        
        <div class="events-grid">
          {#each events as event}
            <div class="event-card">
              <div class="event-image">
                <img src={event.image_url || 'https://picsum.photos/300/200'} alt={event.title} />
                <div class="event-genre">{event.event_type || 'Electronic'}</div>
              </div>
              <div class="event-content">
                <h3>{event.title}</h3>
                <p class="event-venue">{event.venue_name}</p>
                <p class="event-description">{event.description}</p>
                <div class="event-meta">
                  <span class="event-date">{new Date(event.start_date).toLocaleDateString()}</span>
                  <span class="event-price">{formatPrice(event.price)}</span>
                </div>
                <a href="/events" class="btn btn-outline">Get Tickets</a>
              </div>
            </div>
          {/each}
        </div>
        
        <div class="section-cta">
          <a href="/events" class="btn btn-primary">View All Events</a>
        </div>
      </section>

      <!-- Featured Packages -->
      <section class="featured-section packages-section">
        <div class="section-header">
          <h2>Experience Packages</h2>
          <p>Curated multi-venue experiences for the ultimate Berlin night out</p>
        </div>
        
        <div class="packages-grid">
          {#each packages as pkg}
            <div class="package-card" class:featured={pkg.featured}>
              {#if pkg.featured}
                <div class="package-badge">Most Popular</div>
              {/if}
              <div class="package-header">
                <h3>{pkg.name}</h3>
                <div class="package-price">
                  <span class="price-currency">€</span>
                  <span class="price-amount">{pkg.price}</span>
                  <span class="price-period">per person</span>
                </div>
              </div>
              <div class="package-content">
                <p>{pkg.description}</p>
                <ul class="package-features">
                  <li>2-3 exclusive venues</li>
                  <li>Skip-the-line access</li>
                  <li>Welcome drinks included</li>
                  <li>Expert local guide</li>
                </ul>
                <a href="/packages" class="btn btn-package">Choose Package</a>
              </div>
            </div>
          {/each}
        </div>
        
        <div class="section-cta">
          <a href="/packages" class="btn btn-secondary">View All Packages</a>
        </div>
      </section>

      <!-- Featured Venues -->
      <section class="venues-section">
        <div class="section-header">
          <h2>Legendary Venues</h2>
          <p>The clubs that shaped electronic music history</p>
        </div>
        
        <div class="venues-showcase">
          {#each venues as venue}
            <div class="venue-showcase-item">
              <div class="venue-image">
                <img src={venue.image_url || 'https://picsum.photos/400/250'} alt={venue.name} />
              </div>
              <div class="venue-info">
                <h3>{venue.name}</h3>
                <p class="venue-location">{venue.location}</p>
                <p class="venue-description">{venue.description}</p>
                <div class="venue-stats">
                  <span>Capacity: {venue.capacity}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
        
        <div class="section-cta">
          <a href="/venues" class="btn btn-outline">Explore All Venues</a>
        </div>
      </section>
    {/if}

    <!-- Berlin Scene Info -->
    <section class="scene-info">
      <div class="scene-content">
        <div class="scene-text">
          <h2>Why Berlin?</h2>
          <p class="scene-intro">Berlin isn't just a city with clubs—it's the birthplace of modern electronic music culture.</p>
          
          <div class="scene-points">
            <div class="scene-point">
              <h4>🏛️ Historical Significance</h4>
              <p>Born from the ruins of the Berlin Wall, the electronic scene represents freedom and unity</p>
            </div>
            
            <div class="scene-point">
              <h4>🎵 Musical Innovation</h4>
              <p>Home to legendary labels, world-class DJs, and cutting-edge electronic music production</p>
            </div>
            
            <div class="scene-point">
              <h4>🌍 Global Community</h4>
              <p>A melting pot where artists and music lovers from every corner of the world converge</p>
            </div>
            
            <div class="scene-point">
              <h4>⏰ Never-Ending Parties</h4>
              <p>Some clubs run continuously from Friday night to Monday morning—this is marathon partying</p>
            </div>
          </div>
        </div>
        
        <div class="scene-visual">
          <div class="scene-stats-large">
            <div class="large-stat">
              <span class="large-stat-number">48h</span>
              <span class="large-stat-label">Average Weekend</span>
            </div>
            <div class="large-stat">
              <span class="large-stat-number">1989</span>
              <span class="large-stat-label">Scene Birth Year</span>
            </div>
            <div class="large-stat">
              <span class="large-stat-number">24/7</span>
              <span class="large-stat-label">Some Venues</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Call to Action -->
    <section class="final-cta">
      <div class="cta-content">
        <h2>Ready to Experience the Real Berlin?</h2>
        <p>Join thousands who have discovered authentic Berlin through our curated experiences</p>
        <div class="cta-actions">
          <a href="/register" class="btn btn-primary btn-large">
            Start Your Journey
          </a>
          <a href="/about" class="btn btn-secondary btn-large">
            Learn More
          </a>
        </div>
        
        <div class="trust-indicators">
          <div class="trust-item">
            <span class="trust-number">4.9/5</span>
            <span class="trust-label">Average Rating</span>
          </div>
          <div class="trust-item">
            <span class="trust-number">10K+</span>
            <span class="trust-label">Happy Customers</span>
          </div>
          <div class="trust-item">
            <span class="trust-number">150+</span>
            <span class="trust-label">Partner Venues</span>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .home-page {
    overflow-x: hidden;
    padding-top: 80px; /* Account for fixed navigation */
  }

  /* Hero Section */
  .hero {
    position: relative;
    min-height: 100vh;
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #667eea 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 2rem;
  }

  .hero-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.3);
    z-index: 1;
  }

  .hero-content {
    position: relative;
    z-index: 2;
    max-width: 800px;
  }

  .hero-content h1 {
    font-size: 4rem;
    margin-bottom: 1rem;
    font-weight: 700;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
  }

  .hero-subtitle {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    opacity: 0.9;
    font-weight: 300;
  }

  .hero-description {
    font-size: 1.1rem;
    margin-bottom: 2rem;
    opacity: 0.8;
    line-height: 1.6;
  }

  .hero-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-bottom: 3rem;
  }

  .hero-stats {
    position: absolute;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 3rem;
    z-index: 2;
  }

  .stat-item {
    text-align: center;
  }

  .stat-number {
    display: block;
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }

  .stat-label {
    font-size: 0.875rem;
    opacity: 0.8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem 2rem;
    border-radius: 12px;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.3s ease;
    border: 2px solid transparent;
  }

  .btn-icon {
    width: 1.25rem;
    height: 1.25rem;
  }

  .btn-primary {
    background: #667eea;
    color: white;
  }

  .btn-primary:hover {
    background: #5a67d8;
    transform: translateY(-2px);
  }

  .btn-secondary {
    background: transparent;
    color: white;
    border-color: white;
  }

  .btn-secondary:hover {
    background: white;
    color: #667eea;
  }

  .btn-outline {
    background: transparent;
    color: #667eea;
    border-color: #667eea;
  }

  .btn-outline:hover {
    background: #667eea;
    color: white;
  }

  .btn-large {
    padding: 1.25rem 3rem;
    font-size: 1.1rem;
  }

  .btn-package {
    width: 100%;
    background: #f7fafc;
    color: #4a5568;
    border-color: #e2e8f0;
  }

  .btn-package:hover {
    background: #edf2f7;
  }

  /* Container */
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
  }

  /* Section Headers */
  .section-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .section-header h2 {
    font-size: 2.5rem;
    color: #1a202c;
    margin-bottom: 1rem;
  }

  .section-header p {
    font-size: 1.1rem;
    color: #4a5568;
    max-width: 600px;
    margin: 0 auto;
  }

  /* Offerings Section */
  .offerings {
    padding: 5rem 0;
    background: #f7fafc;
  }

  .offerings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 2rem;
  }

  .offering-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    text-align: center;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
  }

  .offering-card:hover {
    transform: translateY(-5px);
  }

  .offering-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .offering-card h3 {
    font-size: 1.25rem;
    color: #1a202c;
    margin-bottom: 1rem;
  }

  .offering-card p {
    color: #4a5568;
    line-height: 1.6;
  }

  /* Featured Sections */
  .featured-section {
    padding: 5rem 0;
  }

  .packages-section {
    background: #f7fafc;
  }

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
    margin-bottom: 3rem;
  }

  .event-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
  }

  .event-card:hover {
    transform: translateY(-5px);
  }

  .event-image {
    position: relative;
    height: 200px;
    overflow: hidden;
  }

  .event-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .event-genre {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(102, 126, 234, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.875rem;
  }

  .event-content {
    padding: 1.5rem;
  }

  .event-content h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .event-venue {
    color: #667eea;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .event-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1rem;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .event-meta {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .event-date {
    color: #4a5568;
  }

  .event-price {
    color: #667eea;
    font-weight: 600;
  }

  /* Packages */
  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 2rem;
    margin-bottom: 3rem;
  }

  .package-card {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    border: 2px solid transparent;
    position: relative;
    transition: all 0.3s;
  }

  .package-card:hover {
    transform: translateY(-5px);
  }

  .package-card.featured {
    border-color: #667eea;
    transform: scale(1.05);
  }

  .package-badge {
    position: absolute;
    top: -1rem;
    left: 50%;
    transform: translateX(-50%);
    background: #667eea;
    color: white;
    padding: 0.5rem 1.5rem;
    border-radius: 20px;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .package-header {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .package-header h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 1rem;
  }

  .package-price {
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 0.25rem;
  }

  .price-currency {
    font-size: 1.25rem;
    color: #4a5568;
  }

  .price-amount {
    font-size: 2.5rem;
    font-weight: 700;
    color: #667eea;
  }

  .price-period {
    color: #4a5568;
  }

  .package-content p {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1.5rem;
  }

  .package-features {
    list-style: none;
    padding: 0;
    margin-bottom: 2rem;
  }

  .package-features li {
    color: #4a5568;
    margin-bottom: 0.5rem;
    padding-left: 1.5rem;
    position: relative;
  }

  .package-features li::before {
    content: '✓';
    position: absolute;
    left: 0;
    color: #48bb78;
    font-weight: bold;
  }

  /* Venues Showcase */
  .venues-section {
    padding: 5rem 0;
  }

  .venues-showcase {
    display: grid;
    gap: 3rem;
    margin-bottom: 3rem;
  }

  .venue-showcase-item {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 3rem;
    align-items: center;
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .venue-showcase-item:nth-child(even) {
    direction: rtl;
  }

  .venue-showcase-item:nth-child(even) > * {
    direction: ltr;
  }

  .venue-image img {
    width: 100%;
    height: 250px;
    object-fit: cover;
  }

  .venue-info {
    padding: 2rem;
  }

  .venue-info h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .venue-location {
    color: #667eea;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .venue-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1rem;
  }

  .venue-stats {
    color: #4a5568;
    font-size: 0.875rem;
  }

  /* Scene Info */
  .scene-info {
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 100%);
    color: white;
    padding: 5rem 0;
    margin: 5rem 0;
  }

  .scene-content {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 4rem;
    align-items: center;
  }

  .scene-text h2 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .scene-intro {
    font-size: 1.2rem;
    opacity: 0.9;
    margin-bottom: 2rem;
  }

  .scene-points {
    display: grid;
    gap: 1.5rem;
  }

  .scene-point h4 {
    font-size: 1.1rem;
    margin-bottom: 0.5rem;
    color: #667eea;
  }

  .scene-point p {
    opacity: 0.8;
    line-height: 1.6;
  }

  .scene-stats-large {
    display: grid;
    gap: 2rem;
  }

  .large-stat {
    text-align: center;
    background: rgba(255, 255, 255, 0.1);
    padding: 2rem;
    border-radius: 12px;
  }

  .large-stat-number {
    display: block;
    font-size: 3rem;
    font-weight: 700;
    color: #667eea;
    margin-bottom: 0.5rem;
  }

  .large-stat-label {
    font-size: 0.875rem;
    opacity: 0.8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Final CTA */
  .final-cta {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 5rem 2rem;
    text-align: center;
    border-radius: 16px;
    margin: 5rem 0;
  }

  .cta-content h2 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .cta-content p {
    font-size: 1.2rem;
    opacity: 0.9;
    margin-bottom: 2rem;
  }

  .cta-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-bottom: 3rem;
  }

  .trust-indicators {
    display: flex;
    justify-content: center;
    gap: 3rem;
    opacity: 0.8;
  }

  .trust-item {
    text-align: center;
  }

  .trust-number {
    display: block;
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }

  .trust-label {
    font-size: 0.875rem;
  }

  /* Section CTAs */
  .section-cta {
    text-align: center;
  }

  /* Loading */
  .loading {
    text-align: center;
    padding: 4rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e2e8f0;
    border-top: 4px solid #667eea;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  /* Responsive */
  @media (max-width: 968px) {
    .hero-content h1 {
      font-size: 3rem;
    }

    .hero-stats {
      position: relative;
      bottom: auto;
      left: auto;
      transform: none;
      margin-top: 2rem;
    }

    .scene-content {
      grid-template-columns: 1fr;
    }

    .venue-showcase-item {
      grid-template-columns: 1fr;
    }

    .venue-showcase-item:nth-child(even) {
      direction: ltr;
    }

    .cta-actions {
      flex-direction: column;
      align-items: center;
    }

    .trust-indicators {
      flex-direction: column;
      gap: 1rem;
    }
  }

  @media (max-width: 640px) {
    .hero-content h1 {
      font-size: 2.5rem;
    }

    .hero-actions {
      flex-direction: column;
      align-items: center;
    }

    .hero-stats {
      flex-direction: column;
      gap: 1rem;
    }

    .events-grid,
    .packages-grid {
      grid-template-columns: 1fr;
    }

    .package-card.featured {
      transform: none;
    }

    .offerings-grid {
      grid-template-columns: 1fr;
    }
  }
</style>