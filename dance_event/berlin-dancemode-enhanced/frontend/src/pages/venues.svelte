<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'

  let venues = []
  let events = []
  let userJoinedEvents = []
  let userEventVenues = []
  let bookings = []
  let bookedVenues = [] // Only venues with booked events
  let loading = true
  let activeTab = 'booked-venues' // Only show booked venues by default

  onMount(async () => {
    await Promise.all([loadVenues(), loadEvents()])
    loadBookings()
    loadBookedVenues()
  })

  function loadBookings() {
    const saved = localStorage.getItem('bookings')
    if (saved) {
      bookings = JSON.parse(saved)
    }
  }

  async function loadVenues() {
    try {
      const response = await api.venues.getAll()
      venues = response.data || []
    } catch (error) {
      console.error('Error loading venues:', error)
    }
  }

  async function loadEvents() {
    try {
      const response = await api.events.getAll()
      events = response.data || []
    } catch (error) {
      console.error('Error loading events:', error)
    } finally {
      loading = false
    }
  }

  function loadBookedVenues() {
    if (!bookings.length || !venues.length || !events.length) {
      bookedVenues = []
      return
    }

    // Get unique venue IDs from bookings
    const bookedVenueIds = [...new Set(bookings.map(booking => booking.venueId))]
    
    // Filter venues to only those with booked events
    bookedVenues = venues.filter(venue => bookedVenueIds.includes(venue.id))
    
    // Enhance venues with booking information
    bookedVenues = bookedVenues.map(venue => {
      const venueBookings = bookings.filter(booking => booking.venueId === venue.id)
      const bookedEvents = events.filter(event => 
        venueBookings.some(booking => booking.eventId === event.id)
      )
      
      return {
        ...venue,
        bookedEvents: bookedEvents,
        bookingsCount: venueBookings.length,
        isBookedVenue: true
      }
    })
  }

  function loadUserJoinedEvents() {
    if (!$authStore.isAuthenticated || !Array.isArray(events)) return
    
    const saved = localStorage.getItem('joinedEvents')
    if (saved) {
      const joinedEventIds = JSON.parse(saved)
      userJoinedEvents = events.filter(event => joinedEventIds.includes(event.id))
      
      // Get unique venues from user's joined events
      const venueNames = [...new Set(userJoinedEvents.map(event => event.venue))]
      userEventVenues = Array.isArray(venues) ? venues.filter(venue => 
        venue.name && venueNames.some(venueName => 
          venue.name.toLowerCase().includes(venueName.toLowerCase()) ||
          venueName.toLowerCase().includes(venue.name.toLowerCase())
        )
      ) : []
      
      // Add venues that don't exist in the venues list but are in user events
      venueNames.forEach(venueName => {
        const existsInVenues = Array.isArray(venues) ? venues.some(venue => 
          venue.name && (
            venue.name.toLowerCase().includes(venueName.toLowerCase()) ||
            venueName.toLowerCase().includes(venue.name.toLowerCase())
          )
        ) : false
        
        if (!existsInVenues) {
          // Create a venue entry for events venues not in the main venues list
          const eventsAtVenue = Array.isArray(userJoinedEvents) ? userJoinedEvents.filter(event => event.venue === venueName) : []
          userEventVenues.push({
            id: `venue-${venueName.replace(/\s+/g, '-').toLowerCase()}`,
            name: venueName,
            address: 'Location to be confirmed',
            description: `Venue hosting ${eventsAtVenue.length} of your joined events`,
            capacity: 'TBA',
            music_style: 'Various',
            entry_price: 'Varies',
            isFromEvents: true,
            eventsCount: eventsAtVenue.length
          })
        }
      })
    }
  }

  function setActiveTab(tab) {
    activeTab = tab
  }

  function getDisplayVenues() {
    switch (activeTab) {
      case 'booked-venues':
        return bookedVenues
      case 'bookings':
        return [] // Handled separately in the template
      default:
        return bookedVenues // Default to only showing booked venues
    }
  }

  function getEventsAtVenue(venueName) {
    if (!Array.isArray(userJoinedEvents)) return []
    return userJoinedEvents.filter(event => 
      event.venue && venueName && (
        event.venue.toLowerCase().includes(venueName.toLowerCase()) ||
        venueName.toLowerCase().includes(event.venue.toLowerCase())
      )
    )
  }

  // Helper functions to normalize event data (handle both API formats)
  function getEventName(event) {
    return event.title || event.name || 'Untitled Event'
  }

  function getEventDate(event) {
    return event.start_date || event.date || new Date().toISOString()
  }

  // Reactive statements to update when data changes
  $: if (bookings.length && venues.length && events.length) {
    loadBookedVenues()
  }
</script>

<svelte:head>
  <title>Venues - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Your Booked Venues</h1>
      <p>Venues where you have booked events from the events page</p>
    </header>

    <!-- Venue Tabs -->
    <div class="tabs">
      <button 
        class="tab-button {activeTab === 'booked-venues' ? 'active' : ''}" 
        on:click={() => setActiveTab('booked-venues')}
      >
        � Booked Venues ({Array.isArray(bookedVenues) ? bookedVenues.length : 0})
      </button>
      
      <button 
        class="tab-button {activeTab === 'bookings' ? 'active' : ''}" 
        on:click={() => setActiveTab('bookings')}
      >
        📋 My Bookings ({Array.isArray(bookings) ? bookings.length : 0})
      </button>
    </div>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading venues...</p>
      </div>
    {:else if activeTab === 'bookings'}
      {#if bookings.length > 0}
        <div class="bookings-grid">
          {#each bookings as booking}
            <div class="booking-card">
              <div class="booking-header">
                <div class="booking-status {booking.status.toLowerCase()}">
                  {booking.status}
                </div>
                <div class="booking-date">
                  Booked: {new Date(booking.bookedAt).toLocaleDateString()}
                </div>
              </div>
              
              <div class="booking-content">
                <h3>{booking.eventTitle}</h3>
                <div class="booking-details">
                  <div class="detail-row">
                    <span class="label">📅 Event Date:</span>
                    <span class="value">{new Date(booking.eventDate).toLocaleDateString('en-US', { 
                      weekday: 'long', 
                      month: 'long', 
                      day: 'numeric' 
                    })}</span>
                  </div>
                  <div class="detail-row">
                    <span class="label">💰 Price:</span>
                    <span class="value">€{booking.eventPrice}</span>
                  </div>
                  <div class="detail-row">
                    <span class="label">🎫 Booking ID:</span>
                    <span class="value">{booking.id.slice(0, 8)}...</span>
                  </div>
                </div>
              </div>
              
              <div class="booking-actions">
                <button class="btn-primary">View Details</button>
                <button class="btn-secondary">Cancel Booking</button>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="empty-state">
          <div class="empty-icon">📋</div>
          <h3>No Bookings Yet</h3>
          <p>Book some events to see your bookings here!</p>
        </div>
      {/if}
    {:else}
      {@const displayVenues = getDisplayVenues()}
      {#if displayVenues.length > 0}
        <div class="venues-grid">
          {#each displayVenues as venue}
            <div class="venue-card {venue.isBookedVenue ? 'booked-venue' : ''}">
              <div class="venue-header">
                <div class="venue-image">
                  <div class="venue-overlay">
                    <h3>{venue.name}</h3>
                    <div class="venue-rating">
                      {#if venue.isBookedVenue}
                        <span class="event-badge">📅 {venue.bookingsCount} Booked Events</span>
                      {:else}
                        <span class="stars">★★★★★</span>
                        <span class="rating-text">4.8</span>
                      {/if}
                    </div>
                  </div>
                </div>
              </div>
              
              <div class="venue-content">
                <div class="venue-location">
                  <i class="icon">📍</i>
                  <span>{venue.address}</span>
                </div>
                <p class="venue-description">{venue.description}</p>
                
                {#if venue.isBookedVenue && venue.bookedEvents}
                  <div class="my-events-section">
                    <h4>� Your Booked Events at this Venue:</h4>
                    <div class="venue-events">
                      {#each venue.bookedEvents as event}
                        <div class="venue-event-item">
                          <span class="event-name">{event.title || event.name}</span>
                          <span class="event-date">
                            {new Date(event.start_date || event.date).toLocaleDateString('en', { 
                              month: 'short', 
                              day: 'numeric' 
                            })}
                          </span>
                          <span class="event-price">€{event.price}</span>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/if}
                
                <div class="venue-details">
                  <div class="detail-item">
                    <i class="icon">👥</i>
                    <span>Capacity: {venue.capacity}</span>
                  </div>
                  <div class="detail-item">
                    <i class="icon">🎵</i>
                    <span>Music: {venue.music_style || 'Electronic'}</span>
                  </div>
                  <div class="detail-item">
                    <i class="icon">💰</i>
                    <span>Entry: €{venue.entry_price || '15-25'}</span>
                  </div>
                  {#if venue.isBookedVenue}
                    <div class="detail-item">
                      <i class="icon">🎟️</i>
                      <span>Bookings: {venue.bookingsCount} events booked</span>
                    </div>
                  {/if}
                </div>
                
                <div class="venue-features">
                  {#if venue.isBookedVenue}
                    <span class="feature-tag booked-tag">Booked Events</span>
                    <span class="feature-tag">Your Venue</span>
                  {:else}
                    <span class="feature-tag">Dance Floor</span>
                    <span class="feature-tag">Bar</span>
                    <span class="feature-tag">VIP Area</span>
                    <span class="feature-tag">Coat Check</span>
                  {/if}
                </div>
              </div>
              
              <div class="venue-footer">
                {#if venue.isBookedVenue}
                  <button class="btn-primary">📅 View Booked Events</button>
                  <button class="btn-secondary">📍 Get Directions</button>
                {:else}
                  <button class="btn-primary">View Events</button>
                  <button class="btn-secondary">Get Directions</button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="empty-state">
          <div class="empty-icon"></div>
          <h3>No Booked Venues Yet</h3>
          <p>Book some events from the Events page to see their venues here!</p>
          <div class="empty-actions">
            <a href="/events" class="btn-primary">Browse Events</a>
          </div>
        </div>
      {/if}
    {/if}
  </div>
</main>

<style>
  main {
    padding: 2rem;
    min-height: 100vh;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
  }

  .page-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .page-header h1 {
    font-size: 3rem;
    color: #2d3748;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .page-header p {
    font-size: 1.2rem;
    color: #718096;
    max-width: 600px;
    margin: 0 auto;
  }

  /* Tabs */
  .tabs {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .tab-button {
    background: white;
    border: 2px solid #e2e8f0;
    padding: 0.75rem 1.5rem;
    border-radius: 25px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    color: #718096;
  }

  .tab-button.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-color: transparent;
  }

  .tab-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    color: #718096;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e2e8f0;
    border-top: 4px solid #667eea;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .venues-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 2rem;
  }

  .venue-card {
    background: white;
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
  }

  .venue-card:hover {
    transform: translateY(-8px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
  }

  .venue-card.booked-venue {
    border: 2px solid #667eea;
    box-shadow: 0 4px 20px rgba(102, 126, 234, 0.15);
  }

  .venue-card.booked-venue:hover {
    box-shadow: 0 12px 40px rgba(102, 126, 234, 0.25);
  }

  .venue-header {
    position: relative;
  }

  .venue-image {
    height: 250px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .venue-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
    color: white;
    padding: 2rem;
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .venue-overlay h3 {
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
  }

  .venue-rating {
    text-align: right;
  }

  .stars {
    color: #ffd700;
    font-size: 1rem;
  }

  .rating-text {
    display: block;
    font-size: 0.9rem;
    margin-top: 0.25rem;
  }

  .event-badge {
    background: rgba(102, 126, 234, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .my-events-section {
    background: #f0f4ff;
    border: 2px solid #c3d9ff;
    border-radius: 12px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .my-events-section h4 {
    color: #2d3748;
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }

  .venue-events {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .venue-event-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: white;
    padding: 0.5rem 0.75rem;
    border-radius: 8px;
    border-left: 4px solid #667eea;
  }

  .event-price {
    color: #667eea;
    font-weight: 600;
    font-size: 0.8rem;
  }

  .event-name {
    font-weight: 600;
    color: #2d3748;
    font-size: 0.9rem;
  }

  .event-date {
    color: #718096;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .venue-content {
    padding: 2rem;
  }

  .venue-location {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #667eea;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .venue-description {
    color: #718096;
    line-height: 1.6;
    margin-bottom: 1.5rem;
  }

  .venue-details {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }

  .detail-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: #4a5568;
    font-weight: 500;
  }

  .venue-features {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .feature-tag {
    background: #f0f4ff;
    color: #667eea;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .feature-tag.event-tag {
    background: #f0fff4;
    color: #48bb78;
    border: 1px solid #c6f6d5;
  }

  .venue-footer {
    padding: 1.5rem 2rem;
    background: #f8fafc;
    display: flex;
    gap: 1rem;
  }

  .btn-primary, .btn-secondary {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
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
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .btn-secondary:hover {
    background: #667eea;
    color: white;
    transform: translateY(-2px);
  }

  .empty-state {
    text-align: center;
    padding: 4rem;
    color: #718096;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .empty-state h3 {
    font-size: 1.5rem;
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  /* Bookings Styles */
  .bookings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
  }

  .booking-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
    border-left: 4px solid #667eea;
  }

  .booking-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
  }

  .booking-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1rem 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .booking-status {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .booking-status.pending {
    background: rgba(255, 193, 7, 0.2);
    color: #ffc107;
  }

  .booking-status.confirmed {
    background: rgba(40, 167, 69, 0.2);
    color: #28a745;
  }

  .booking-date {
    font-size: 0.85rem;
    opacity: 0.9;
  }

  .booking-content {
    padding: 1.5rem;
  }

  .booking-content h3 {
    color: #2d3748;
    margin-bottom: 1rem;
    font-size: 1.25rem;
  }

  .booking-details {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .detail-row .label {
    color: #718096;
    font-size: 0.9rem;
  }

  .detail-row .value {
    color: #2d3748;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .booking-actions {
    padding: 1rem 1.5rem;
    background: #f8fafc;
    display: flex;
    gap: 1rem;
  }

  @media (max-width: 768px) {
    main {
      padding: 1rem;
    }

    .page-header h1 {
      font-size: 2rem;
    }

    .venues-grid {
      grid-template-columns: 1fr;
    }

    .venue-footer {
      flex-direction: column;
    }

    .tabs {
      flex-direction: column;
      align-items: stretch;
    }

    .tab-button {
      margin-bottom: 0.5rem;
    }

    .venue-event-item {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .my-events-section {
      margin: 1rem;
      padding: 0.75rem;
    }
  }
</style>