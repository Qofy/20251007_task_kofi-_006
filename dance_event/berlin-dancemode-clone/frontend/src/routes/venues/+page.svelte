<script>
  import { onMount } from 'svelte';
  import api from '$lib/api.js';
  
  let venues = [];
  let loading = true;
  let error = null;
  let selectedVenue = null;

  onMount(async () => {
    try {
      const response = await api.get('/api/venues');
      venues = Array.isArray(response.data) ? response.data : [];
      loading = false;
    } catch (err) {
      console.error('Error loading venues:', err);
      error = 'Failed to load venues';
      venues = [];
      loading = false;
    }
  });

  function openVenueModal(venue) {
    selectedVenue = venue;
  }

  function closeVenueModal() {
    selectedVenue = null;
  }
</script>

<svelte:head>
  <title>Venues - Berlin DanceMode</title>
  <meta name="description" content="Discover Berlin's legendary electronic music venues and underground clubs" />
</svelte:head>

<div class="venues-page">
  <div class="hero">
    <div class="hero-content">
      <h1>Berlin's Legendary Venues</h1>
      <p>From underground bunkers to world-famous clubs</p>
    </div>
  </div>

  <div class="container">
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading venues...</p>
      </div>
    {:else if error}
      <div class="error">
        <p>{error}</p>
        <button on:click={() => window.location.reload()}>Try Again</button>
      </div>
    {:else}
      <!-- Venue Categories -->
      <section class="venue-categories">
        <h2>Venue Types</h2>
        <div class="categories-grid">
          <div class="category-card">
            <div class="category-icon">🏭</div>
            <h3>Industrial Clubs</h3>
            <p>Raw concrete spaces with world-class sound systems</p>
          </div>
          <div class="category-card">
            <div class="category-icon">🌃</div>
            <h3>Rooftop Venues</h3>
            <p>Dance under the stars with Berlin's skyline views</p>
          </div>
          <div class="category-card">
            <div class="category-icon">🚇</div>
            <h3>Underground</h3>
            <p>Hidden gems in basements and forgotten spaces</p>
          </div>
          <div class="category-card">
            <div class="category-icon">🎭</div>
            <h3>Multi-Room</h3>
            <p>Multiple floors, diverse music styles, endless exploration</p>
          </div>
        </div>
      </section>

      <!-- Venues Grid -->
      <div class="venues-grid">
        {#each venues as venue}
          <div class="venue-card" on:click={() => openVenueModal(venue)} on:keydown={(e) => e.key === 'Enter' && openVenueModal(venue)} role="button" tabindex="0">
            <div class="venue-image">
              <img src={venue.image_url || 'https://picsum.photos/400/250'} alt={venue.name} />
              <div class="venue-overlay">
                <div class="venue-info">
                  <h3>{venue.name}</h3>
                  <p class="venue-location">{venue.location}</p>
                  <div class="venue-capacity">
                    <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                      <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z" />
                    </svg>
                    {venue.capacity} people
                  </div>
                </div>
              </div>
            </div>
            
            <div class="venue-content">
              <div class="venue-details">
                <h4>{venue.name}</h4>
                <p class="venue-description">{venue.description}</p>
                
                <div class="venue-features">
                  <div class="feature">
                    <svg class="feature-icon" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM15.657 6.343a1 1 0 011.414 0A9.972 9.972 0 0119 12a9.972 9.972 0 01-1.929 5.657 1 1 0 11-1.414-1.414A7.971 7.971 0 0017 12c0-2.21-.896-4.21-2.343-5.657a1 1 0 010-1.414zm-2.829 2.828a1 1 0 011.415 0A5.983 5.983 0 0115 12a5.984 5.984 0 01-.757 2.828 1 1 0 11-1.415-1.656A3.989 3.989 0 0013 12a3.988 3.988 0 00-.172-1.172 1 1 0 010-1.415z" clip-rule="evenodd" />
                    </svg>
                    <span>Premium Sound</span>
                  </div>
                  
                  <div class="feature">
                    <svg class="feature-icon" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
                    </svg>
                    <span>{venue.location}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      <!-- Berlin Club Culture Section -->
      <section class="culture-section">
        <div class="culture-content">
          <h2>Berlin Club Culture</h2>
          <div class="culture-grid">
            <div class="culture-text">
              <h3>The Underground Movement</h3>
              <p>Berlin's electronic music scene emerged from the ruins of the Berlin Wall, transforming abandoned industrial spaces into temples of techno. These venues aren't just clubs—they're cultural institutions that have shaped electronic music worldwide.</p>
              
              <h3>What Makes Berlin Special</h3>
              <ul>
                <li><strong>No Photos Policy:</strong> Most venues ban photography to preserve the authenticity and freedom of expression</li>
                <li><strong>Marathon Sessions:</strong> Clubs often run from Friday night until Monday morning</li>
                <li><strong>Dress Code:</strong> All black, leather, and industrial fashion are the norm</li>
                <li><strong>Door Policy:</strong> Getting in is part of the experience—attitude matters more than appearance</li>
              </ul>
            </div>
            
            <div class="culture-stats">
              <div class="stat">
                <div class="stat-number">400+</div>
                <div class="stat-label">Active Venues</div>
              </div>
              <div class="stat">
                <div class="stat-number">48h</div>
                <div class="stat-label">Average Weekend</div>
              </div>
              <div class="stat">
                <div class="stat-number">1989</div>
                <div class="stat-label">Scene Beginning</div>
              </div>
              <div class="stat">
                <div class="stat-number">24/7</div>
                <div class="stat-label">Some Venues</div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Tips Section -->
      <section class="tips-section">
        <h2>Insider Tips</h2>
        <div class="tips-grid">
          <div class="tip">
            <div class="tip-icon">🚪</div>
            <h4>Getting In</h4>
            <p>Arrive confident but respectful. Learn a few German phrases. Don't be pushy with the bouncer.</p>
          </div>
          
          <div class="tip">
            <div class="tip-icon">💰</div>
            <h4>Payment</h4>
            <p>Most venues are cash-only. Bring euros and expect to pay entry fees ranging from €10-25.</p>
          </div>
          
          <div class="tip">
            <div class="tip-icon">⏰</div>
            <h4>Timing</h4>
            <p>The party starts late. Don't arrive before 1 AM on weekends if you want to experience the real vibe.</p>
          </div>
          
          <div class="tip">
            <div class="tip-icon">🤝</div>
            <h4>Respect</h4>
            <p>Respect the no-photo rule, give DJs space, and remember you're entering a sacred space for many.</p>
          </div>
        </div>
      </section>
    {/if}
  </div>
</div>

<!-- Venue Modal -->
{#if selectedVenue}
  <div class="modal-overlay" on:click={closeVenueModal} on:keydown={(e) => e.key === 'Escape' && closeVenueModal()} role="button" tabindex="0">
    <div class="modal-content" on:click|stopPropagation>
      <button class="modal-close" on:click={closeVenueModal}>
        <svg viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
      
      <div class="modal-header">
        <img src={selectedVenue.image_url || 'https://picsum.photos/600/300'} alt={selectedVenue.name} />
        <div class="modal-title">
          <h2>{selectedVenue.name}</h2>
          <p>{selectedVenue.location}</p>
        </div>
      </div>
      
      <div class="modal-body">
        <div class="modal-section">
          <h3>About This Venue</h3>
          <p>{selectedVenue.description}</p>
        </div>
        
        <div class="modal-info">
          <div class="info-item">
            <strong>Capacity:</strong> {selectedVenue.capacity} people
          </div>
          <div class="info-item">
            <strong>Music Style:</strong> Techno, House, Experimental
          </div>
          <div class="info-item">
            <strong>Best Time:</strong> Friday & Saturday nights
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .venues-page {
    min-height: 100vh;
  }

  .hero {
    height: 50vh;
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    text-align: center;
  }

  .hero-content h1 {
    font-size: 3.5rem;
    margin-bottom: 1rem;
    font-weight: 700;
  }

  .hero-content p {
    font-size: 1.2rem;
    opacity: 0.9;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 3rem 2rem;
  }

  .venue-categories {
    margin-bottom: 4rem;
  }

  .venue-categories h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .categories-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
  }

  .category-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    text-align: center;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
  }

  .category-card:hover {
    transform: translateY(-5px);
  }

  .category-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .category-card h3 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .category-card p {
    color: #4a5568;
    line-height: 1.6;
  }

  .venues-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 2rem;
    margin-bottom: 4rem;
  }

  .venue-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    cursor: pointer;
    transition: all 0.3s;
  }

  .venue-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  }

  .venue-image {
    position: relative;
    height: 250px;
    overflow: hidden;
  }

  .venue-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s;
  }

  .venue-card:hover .venue-image img {
    transform: scale(1.05);
  }

  .venue-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
    color: white;
    padding: 2rem 1.5rem 1.5rem;
  }

  .venue-info h3 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .venue-location {
    opacity: 0.8;
    margin-bottom: 0.5rem;
  }

  .venue-capacity {
    display: flex;
    align-items: center;
    font-size: 0.875rem;
    opacity: 0.9;
  }

  .venue-content {
    padding: 1.5rem;
  }

  .venue-details h4 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .venue-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1rem;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .venue-features {
    display: flex;
    gap: 1rem;
  }

  .feature {
    display: flex;
    align-items: center;
    color: #4a5568;
    font-size: 0.875rem;
  }

  .feature-icon, .icon {
    width: 1rem;
    height: 1rem;
    margin-right: 0.5rem;
  }

  .culture-section {
    background: #f7fafc;
    padding: 3rem 2rem;
    border-radius: 16px;
    margin-bottom: 4rem;
  }

  .culture-content h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .culture-grid {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 3rem;
    align-items: start;
  }

  .culture-text h3 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .culture-text p {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 2rem;
  }

  .culture-text ul {
    list-style: none;
    padding: 0;
  }

  .culture-text li {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 0.75rem;
  }

  .culture-stats {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }

  .stat {
    text-align: center;
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .stat-number {
    font-size: 2rem;
    font-weight: 700;
    color: #667eea;
    margin-bottom: 0.25rem;
  }

  .stat-label {
    font-size: 0.875rem;
    color: #4a5568;
    font-weight: 500;
  }

  .tips-section {
    text-align: center;
  }

  .tips-section h2 {
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .tips-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
  }

  .tip {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .tip-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .tip h4 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .tip p {
    color: #4a5568;
    line-height: 1.6;
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
  }

  .modal-content {
    background: white;
    border-radius: 16px;
    max-width: 600px;
    width: 100%;
    max-height: 90vh;
    overflow-y: auto;
    position: relative;
  }

  .modal-close {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(0, 0, 0, 0.5);
    color: white;
    border: none;
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    cursor: pointer;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .modal-close svg {
    width: 1rem;
    height: 1rem;
  }

  .modal-header {
    position: relative;
  }

  .modal-header img {
    width: 100%;
    height: 250px;
    object-fit: cover;
  }

  .modal-title {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
    color: white;
    padding: 2rem 1.5rem 1.5rem;
  }

  .modal-title h2 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .modal-body {
    padding: 2rem;
  }

  .modal-section {
    margin-bottom: 2rem;
  }

  .modal-section h3 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .modal-section p {
    color: #4a5568;
    line-height: 1.6;
  }

  .modal-info {
    display: grid;
    gap: 0.75rem;
  }

  .info-item {
    color: #4a5568;
  }

  .info-item strong {
    color: #1a202c;
  }

  .loading, .error {
    text-align: center;
    padding: 4rem 2rem;
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

  @media (max-width: 768px) {
    .hero-content h1 {
      font-size: 2.5rem;
    }

    .venues-grid {
      grid-template-columns: 1fr;
    }

    .culture-grid {
      grid-template-columns: 1fr;
    }

    .culture-stats {
      grid-template-columns: repeat(2, 1fr);
    }

    .modal-overlay {
      padding: 1rem;
    }
  }
</style>