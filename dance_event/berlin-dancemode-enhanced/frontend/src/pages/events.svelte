<script>
  import { onMount } from 'svelte';
    import { createEvent, getEvents, getVenues, bookEvent } from '../services/api.js';
  
  let events = [];
  let loading = true;
  let error = null;
  let searchTerm = '';
  let selectedEventType = '';
  let priceRange = { min: 0, max: 1000 };
  
  // Filter and search functionality
  $: filteredEvents = events.filter(event => {
    const matchesSearch = event.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         event.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesType = !selectedEventType || event.event_type === selectedEventType;
    const matchesPrice = event.price >= priceRange.min && event.price <= priceRange.max;
    
    return matchesSearch && matchesType && matchesPrice;
  });
  
  onMount(async () => {
    try {
      const response = await getEvents();
      events = response.data.data || [];
    } catch (err) {
      console.error('Failed to load events:', err);
      error = 'Failed to load events. Please try again later.';
    } finally {
      loading = false;
    }
  });
  
  function formatPrice(price) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR'
    }).format(price);
  }
  
  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString('en-US', {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  async function handleBooking(eventId) {
    try {
      const response = await bookEvent(eventId);
      if (response.data && response.data.success) {
        alert('Booking successful! A booking card has been created.');
        // Store booking info for venues page
        const bookings = JSON.parse(localStorage.getItem('bookings') || '[]');
        const eventToBook = events.find(e => e.id === eventId);
        if (eventToBook) {
          const booking = {
            id: response.data.data.id,
            eventId: eventId,
            eventTitle: eventToBook.title,
            eventDate: eventToBook.start_date,
            eventPrice: eventToBook.price,
            venueId: eventToBook.venue_id,
            bookedAt: new Date().toISOString(),
            status: 'Pending'
          };
          bookings.push(booking);
          localStorage.setItem('bookings', JSON.stringify(bookings));
        }
      } else {
        alert('Booking failed. Please try again.');
      }
    } catch (err) {
      console.error('Booking error:', err);
      alert('Booking failed. Please try again.');
    }
  }
</script>

<svelte:head>
  <title>Electronic Music Events - Berlin DanceMode</title>
  <meta name="description" content="Discover Berlin's hottest electronic music events, from underground warehouse parties to legendary club nights." />
</svelte:head>

<div class="events-page">
  <div class="page-header">
    <div class="container">
      <h1>Electronic Music Events</h1>
      <p>Discover Berlin's legendary electronic music scene through our curated events</p>
    </div>
  </div>

  <div class="container">
    <!-- Filters -->
    <div class="filters-section">
      <div class="search-filter">
        <input 
          type="text" 
          placeholder="Search events..." 
          bind:value={searchTerm}
          class="search-input"
        />
      </div>
      
      <div class="type-filter">
        <select bind:value={selectedEventType} class="filter-select">
          <option value="">All Event Types</option>
          <option value="Workshop">Workshop</option>
          <option value="Festival">Festival</option>
          <option value="Intensive">Intensive</option>
          <option value="Social">Social</option>
          <option value="Competition">Competition</option>
        </select>
      </div>
      
      <div class="price-filter">
        <label for="price-min">
          Price Range: €{priceRange.min} - €{priceRange.max}
        </label>
        <div class="price-range">
          <input 
            id="price-min"
            type="range" 
            min="0" 
            max="1000" 
            bind:value={priceRange.min}
            class="range-input"
          />
          <input 
            type="range" 
            min="0" 
            max="1000" 
            bind:value={priceRange.max}
            class="range-input"
          />
        </div>
      </div>
    </div>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading events...</p>
      </div>
    {:else if error}
      <div class="error">
        <h3>Error</h3>
        <p>{error}</p>
      </div>
    {:else}
      <div class="events-grid">
        {#each filteredEvents as event}
          <div class="event-card">
            <div class="event-image">
              <img src="https://picsum.photos/400/250?random={event.id}" alt={event.title} />
              <div class="event-type-badge">{event.event_type}</div>
              <div class="event-price">{formatPrice(event.price)}</div>
            </div>
            
            <div class="event-content">
              <h3>{event.title}</h3>
              <p class="event-description">{event.description}</p>
              
              <div class="event-details">
                <div class="event-date">
                  <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd" />
                  </svg>
                  <span>{formatDate(event.start_date)}</span>
                </div>
                
                <div class="event-participants">
                  <svg class="icon" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z" />
                  </svg>
                  <span>{event.current_participants}/{event.max_participants} participants</span>
                </div>
              </div>
              
              <div class="event-actions">
                <button class="btn btn-primary" on:click={() => handleBooking(event.id)}>Book Now</button>
                <button class="btn btn-outline">Learn More</button>
              </div>
            </div>
          </div>
        {/each}
      </div>
      
      {#if filteredEvents.length === 0}
        <div class="no-results">
          <h3>No events found</h3>
          <p>Try adjusting your search criteria or check back later for new events.</p>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .events-page {
    padding-top: 80px; /* Account for fixed navigation */
  }

  .page-header {
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #667eea 100%);
    color: white;
    padding: 4rem 0;
    text-align: center;
  }

  .page-header h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .page-header p {
    font-size: 1.25rem;
    opacity: 0.9;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
  }

  .filters-section {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 2rem;
    margin: 3rem 0;
    padding: 2rem;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }

  .search-input,
  .filter-select {
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.3s;
  }

  .search-input:focus,
  .filter-select:focus {
    outline: none;
    border-color: #667eea;
  }

  .price-filter label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #4a5568;
  }

  .price-range {
    display: flex;
    gap: 1rem;
  }

  .range-input {
    flex: 1;
  }

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
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
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  }

  .event-image {
    position: relative;
    height: 250px;
    overflow: hidden;
  }

  .event-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .event-type-badge {
    position: absolute;
    top: 1rem;
    left: 1rem;
    background: rgba(102, 126, 234, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .event-price {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(0, 0, 0, 0.8);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-weight: 700;
  }

  .event-content {
    padding: 2rem;
  }

  .event-content h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 1rem;
  }

  .event-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1.5rem;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .event-details {
    margin-bottom: 2rem;
  }

  .event-date,
  .event-participants {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    color: #4a5568;
    font-size: 0.875rem;
  }

  .icon {
    width: 1.25rem;
    height: 1.25rem;
    color: #667eea;
  }

  .event-actions {
    display: flex;
    gap: 1rem;
  }

  .btn {
    flex: 1;
    padding: 1rem 2rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    text-decoration: none;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s;
  }

  .btn-primary {
    background: #667eea;
    color: white;
  }

  .btn-primary:hover {
    background: #5a67d8;
  }

  .btn-outline {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .btn-outline:hover {
    background: #667eea;
    color: white;
  }

  .loading,
  .error,
  .no-results {
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

  .error {
    color: #e53e3e;
  }

  .no-results h3 {
    color: #4a5568;
    margin-bottom: 1rem;
  }

  .no-results p {
    color: #718096;
  }

  @media (max-width: 768px) {
    .filters-section {
      grid-template-columns: 1fr;
    }

    .events-grid {
      grid-template-columns: 1fr;
    }

    .event-actions {
      flex-direction: column;
    }
  }
</style>