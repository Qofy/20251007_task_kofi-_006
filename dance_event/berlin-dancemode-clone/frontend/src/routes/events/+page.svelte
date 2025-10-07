<script>
  import { onMount } from 'svelte';
  import api from '$lib/api.js';
  
  let events = [];
  let loading = true;
  let error = null;
  let searchTerm = '';
  let selectedGenre = 'all';
  let genres = ['all', 'Workshop', 'Festival', 'Intensive'];

  onMount(async () => {
    try {
      const response = await api.get('/api/events');
      console.log('Events API response:', response);
      events = Array.isArray(response.data) ? response.data : [];
      loading = false;
    } catch (err) {
      console.error('Error loading events:', err);
      error = 'Failed to load events';
      events = [];
      loading = false;
    }
  });

  $: filteredEvents = Array.isArray(events) ? events.filter(event => {
    const matchesSearch = event.title?.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         event.description?.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesGenre = selectedGenre === 'all' || event.event_type === selectedGenre;
    return matchesSearch && matchesGenre;
  }) : [];

  function formatDate(dateStr) {
    return new Date(dateStr).toLocaleDateString('en-US', {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function formatPrice(price) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR'
    }).format(price);
  }
</script>

<svelte:head>
  <title>Events - Berlin DanceMode</title>
  <meta name="description" content="Discover the hottest electronic music events in Berlin's underground scene" />
</svelte:head>

<div class="events-page">
  <div class="hero">
    <div class="hero-content">
      <h1>Berlin's Electronic Events</h1>
      <p>Experience the pulse of Berlin's legendary electronic music scene</p>
    </div>
    <div class="hero-overlay"></div>
  </div>

  <div class="container">
    <!-- Search and Filter Section -->
    <div class="filters">
      <div class="search-box">
        <input
          type="text"
          placeholder="Search events..."
          bind:value={searchTerm}
          class="search-input"
        />
        <svg class="search-icon" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
        </svg>
      </div>
      
      <select bind:value={selectedGenre} class="genre-filter">
        {#each genres as genre}
          <option value={genre}>
            {genre === 'all' ? 'All Genres' : genre.charAt(0).toUpperCase() + genre.slice(1)}
          </option>
        {/each}
      </select>
    </div>

    <!-- Events Grid -->
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading events...</p>
      </div>
    {:else if error}
      <div class="error">
        <p>{error}</p>
        <button on:click={() => window.location.reload()}>Try Again</button>
      </div>
    {:else if filteredEvents.length === 0}
      <div class="no-results">
        <h3>No events found</h3>
        <p>Try adjusting your search or filter criteria</p>
      </div>
    {:else}
      <div class="events-grid">
        {#each filteredEvents as event}
          <div class="event-card">
            <div class="event-image">
              <img src={event.image_url || 'https://picsum.photos/400/250'} alt={event.title} />
              <div class="event-genre">{event.event_type}</div>
            </div>
            
            <div class="event-content">
              <h3>{event.title}</h3>
              <p class="event-description">{event.description}</p>
              
              <div class="event-details">
                <div class="detail-item">
                  <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd" />
                  </svg>
                  <span>{formatDate(event.start_date)}</span>
                </div>
                
                <div class="detail-item">
                  <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
                  </svg>
                  <span>{event.venue_name}</span>
                </div>
                
                <div class="detail-item price">
                  <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M8.433 7.418c.155-.103.346-.196.567-.267v1.698a2.305 2.305 0 01-.567-.267C8.07 8.34 8 8.114 8 8c0-.114.07-.34.433-.582zM11 12.849v-1.698c.22.071.412.164.567.267.364.243.433.468.433.582 0 .114-.07.34-.433.582a2.305 2.305 0 01-.567.267z" />
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-13a1 1 0 10-2 0v.092a4.535 4.535 0 00-1.676.662C6.602 6.234 6 7.009 6 8c0 .99.602 1.765 1.324 2.246.48.32 1.054.545 1.676.662v1.941c-.391-.127-.68-.317-.843-.504a1 1 0 10-1.51 1.31c.562.649 1.413 1.076 2.353 1.253V15a1 1 0 102 0v-.092a4.535 4.535 0 001.676-.662C13.398 13.766 14 12.991 14 12c0-.99-.602-1.765-1.324-2.246A4.535 4.535 0 0011 9.092V7.151c.391.127.68.317.843.504a1 1 0 101.511-1.31c-.563-.649-1.413-1.076-2.354-1.253V5z" clip-rule="evenodd" />
                  </svg>
                  <span class="price-amount">{formatPrice(event.price)}</span>
                </div>
              </div>
              
              <div class="event-actions">
                <button class="btn-primary">
                  Book Tickets
                </button>
                <button class="btn-secondary">
                  View Details
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <!-- Featured Section -->
    <section class="featured-section">
      <h2>Why Choose Berlin DanceMode?</h2>
      <div class="features-grid">
        <div class="feature">
          <div class="feature-icon">🎵</div>
          <h3>Curated Events</h3>
          <p>Hand-picked events from Berlin's best underground venues</p>
        </div>
        <div class="feature">
          <div class="feature-icon">🎫</div>
          <h3>Instant Booking</h3>
          <p>Secure your spot at exclusive events with instant confirmation</p>
        </div>
        <div class="feature">
          <div class="feature-icon">🌃</div>
          <h3>Local Insights</h3>
          <p>Insider knowledge of Berlin's electronic music scene</p>
        </div>
        <div class="feature">
          <div class="feature-icon">💯</div>
          <h3>Authentic Experience</h3>
          <p>Connect with the real underground culture of Berlin</p>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .events-page {
    min-height: 100vh;
  }

  .hero {
    height: 50vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
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
    padding: 2rem;
  }

  .filters {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    align-items: center;
  }

  .search-box {
    position: relative;
    flex: 1;
    max-width: 400px;
  }

  .search-input {
    width: 100%;
    padding: 1rem 1rem 1rem 3rem;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    font-size: 1rem;
    transition: border-color 0.3s;
  }

  .search-input:focus {
    outline: none;
    border-color: #667eea;
  }

  .search-icon {
    position: absolute;
    left: 1rem;
    top: 50%;
    transform: translateY(-50%);
    width: 1.25rem;
    height: 1.25rem;
    color: #94a3b8;
  }

  .genre-filter {
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    font-size: 1rem;
    background: white;
    cursor: pointer;
  }

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
    margin-bottom: 4rem;
  }

  .event-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s, box-shadow 0.3s;
  }

  .event-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
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
    font-weight: 500;
    text-transform: capitalize;
  }

  .event-content {
    padding: 1.5rem;
  }

  .event-content h3 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .event-description {
    color: #4a5568;
    margin-bottom: 1rem;
    line-height: 1.6;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .event-details {
    margin-bottom: 1.5rem;
  }

  .detail-item {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
    color: #4a5568;
  }

  .detail-item.price {
    color: #667eea;
    font-weight: 600;
  }

  .icon {
    width: 1rem;
    height: 1rem;
    margin-right: 0.5rem;
    flex-shrink: 0;
  }

  .price-amount {
    font-size: 1.1rem;
    font-weight: 700;
  }

  .event-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn-primary, .btn-secondary {
    flex: 1;
    padding: 0.75rem;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s;
    border: none;
  }

  .btn-primary {
    background: #667eea;
    color: white;
  }

  .btn-primary:hover {
    background: #5a67d8;
  }

  .btn-secondary {
    background: #f7fafc;
    color: #4a5568;
    border: 1px solid #e2e8f0;
  }

  .btn-secondary:hover {
    background: #edf2f7;
  }

  .loading, .error, .no-results {
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

  .featured-section {
    margin-top: 4rem;
    padding: 3rem 0;
    background: #f7fafc;
    border-radius: 16px;
    text-align: center;
  }

  .featured-section h2 {
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
    padding: 0 2rem;
  }

  .feature {
    padding: 1.5rem;
  }

  .feature-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .feature h3 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .feature p {
    color: #4a5568;
    line-height: 1.6;
  }

  @media (max-width: 768px) {
    .hero-content h1 {
      font-size: 2.5rem;
    }

    .filters {
      flex-direction: column;
      align-items: stretch;
    }

    .search-box {
      max-width: none;
    }

    .events-grid {
      grid-template-columns: 1fr;
    }

    .event-actions {
      flex-direction: column;
    }
  }
</style>