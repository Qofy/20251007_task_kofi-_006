<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'

  let venues = []
  let events = []
  let bookings = []
  let loading = true

  onMount(async () => {
    await Promise.all([loadVenues(), loadEvents()])
    loadBookings()
  })

  function loadBookings() {
    const saved = localStorage.getItem('event-bookings') || localStorage.getItem('bookings')
    if (saved) {
      const rawBookings = JSON.parse(saved)
      // Ensure events and venues are arrays before using find
      const eventsArray = Array.isArray(events) ? events : []
      const venuesArray = Array.isArray(venues) ? venues : []
      
      bookings = rawBookings.map(booking => {
        const event = eventsArray.find(e => e.id === booking.eventId)
        const venue = venuesArray.find(v => v.id === booking.venueId)
        return {
          ...booking,
          eventName: event ? event.name || event.title : 'Unknown Event',
          eventDate: event ? event.date || event.start_date : 'Unknown Date',
          eventTime: event ? event.time || event.start_time : 'Unknown Time',
          eventPrice: event ? event.price : 0,
          venueName: venue ? venue.name : 'Unknown Venue',
          venueLocation: venue ? venue.location || venue.address : 'Unknown Location'
        }
      })
    }
  }

  async function loadVenues() {
    try {
      const response = await api.venues.getAll()
      // Handle different response formats
      venues = Array.isArray(response) ? response : (response.data || response.venues || [])
      console.log('Loaded venues:', venues)
    } catch (error) {
      console.error('Error loading venues:', error)
      venues = []
    }
  }

  async function loadEvents() {
    try {
      const response = await api.events.getAll()
      // Handle different response formats
      events = Array.isArray(response) ? response : (response.data || response.events || [])
      console.log('Loaded events:', events)
    } catch (error) {
      console.error('Error loading events:', error)
      events = []
    } finally {
      loading = false
    }
  }

  function cancelBooking(bookingId) {
    // Remove booking from localStorage
    const saved = localStorage.getItem('event-bookings') || localStorage.getItem('bookings')
    if (saved) {
      const allBookings = JSON.parse(saved)
      const updatedBookings = allBookings.filter(booking => booking.id !== bookingId)
      
      // Update both possible localStorage keys to ensure consistency
      localStorage.setItem('event-bookings', JSON.stringify(updatedBookings))
      localStorage.setItem('bookings', JSON.stringify(updatedBookings))
      
      // Reload bookings to update the display
      loadBookings()
    }
  }
</script>

<svelte:head>
  <title>My Bookings - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>My Bookings</h1>
      <p>Manage your event bookings and cancellations</p>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading bookings...</p>
      </div>
    {:else if bookings.length > 0}
      <div class="bookings-grid">
        {#each bookings as booking}
          <div class="booking-card">
            <div class="booking-header">
              <div class="booking-status confirmed">
                Confirmed
              </div>
              <div class="booking-date">
                Booked: {new Date(booking.bookedAt || booking.createdAt || Date.now()).toLocaleDateString()}
              </div>
            </div>
            
            <div class="booking-content">
              <h3>{booking.eventName || booking.eventTitle || 'Event'}</h3>
              <div class="booking-details">
                <div class="detail-row">
                  <span class="label">📍 Venue:</span>
                  <span class="value">{booking.venueName || 'Unknown Venue'}</span>
                </div>
                <div class="detail-row">
                  <span class="label">📅 Event Date:</span>
                  <span class="value">{new Date(booking.eventDate || Date.now()).toLocaleDateString('en-US', { 
                    weekday: 'long', 
                    month: 'long', 
                    day: 'numeric' 
                  })}</span>
                </div>
                <div class="detail-row">
                  <span class="label">⏰ Time:</span>
                  <span class="value">{booking.eventTime || 'TBA'}</span>
                </div>
                <div class="detail-row">
                  <span class="label">💰 Price:</span>
                  <span class="value">€{booking.eventPrice || booking.price || 0}</span>
                </div>
                <div class="detail-row">
                  <span class="label">🎫 Booking ID:</span>
                  <span class="value">{booking.id.slice(0, 8)}...</span>
                </div>
              </div>
            </div>
            
            <div class="booking-actions">
              <button class="btn-primary">View Details</button>
              <button class="btn-cancel" on:click={() => cancelBooking(booking.id)}>Cancel Booking</button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">📋</div>
        <h3>No Bookings Yet</h3>
        <p>Book some events to see your bookings here!</p>
        <a href="/events" class="btn-primary">Browse Events</a>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Page Layout */
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .page-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .page-header h1 {
    font-size: 3rem;
    margin-bottom: 0.5rem;
    color: #2d3748;
  }

  .page-header p {
    color: #718096;
    font-size: 1.2rem;
  }

  /* Bookings Grid */
  .bookings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }

  .booking-card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }

  .booking-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
  }

  .booking-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 1.5rem;
    color: white;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .booking-status {
    background: rgba(255, 255, 255, 0.2);
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .booking-status.confirmed {
    background: rgba(72, 187, 120, 0.9);
  }

  .booking-date {
    font-size: 0.9rem;
    opacity: 0.9;
  }

  .booking-content {
    padding: 1.5rem;
  }

  .booking-content h3 {
    margin: 0 0 1rem 0;
    color: #2d3748;
    font-size: 1.3rem;
  }

  .booking-details {
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e2e8f0;
  }

  .detail-row:last-child {
    border-bottom: none;
  }

  .detail-row .label {
    font-weight: 600;
    color: #4a5568;
    font-size: 0.9rem;
  }

  .detail-row .value {
    color: #2d3748;
    font-weight: 500;
  }

  .booking-actions {
    padding: 1.5rem;
    background: #f7fafc;
    display: flex;
    gap: 1rem;
  }

  .btn-primary {
    flex: 1;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
    text-decoration: none;
    text-align: center;
    display: inline-block;
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  }

  .btn-cancel {
    flex: 1;
    background: linear-gradient(135deg, #fc8181 0%, #e53e3e 100%);
    color: white;
    border: none;
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
  }

  .btn-cancel:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(229, 62, 62, 0.4);
  }

  /* Empty State */
  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    color: #718096;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1.5rem;
  }

  .empty-state h3 {
    margin-bottom: 1rem;
    color: #2d3748;
    font-size: 1.5rem;
  }

  .empty-state p {
    margin-bottom: 2rem;
    font-size: 1.1rem;
  }

  .empty-state .btn-primary {
    display: inline-block;
    text-decoration: none;
    padding: 1rem 2rem;
  }

  /* Loading Styles */
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

  /* Responsive Design */
  @media (max-width: 768px) {
    .container {
      padding: 1rem;
    }

    .page-header h1 {
      font-size: 2rem;
    }

    .bookings-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }

    .booking-actions {
      flex-direction: column;
    }

    .detail-row {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>