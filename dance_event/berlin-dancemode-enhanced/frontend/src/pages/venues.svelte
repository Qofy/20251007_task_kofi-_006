<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'

  let venues = []
  let events = []
  let userJoinedEvents = []
  let userEventVenues = []
  let loading = true
  let activeTab = 'all' // 'all', 'my-venues'

  onMount(async () => {
    await Promise.all([loadVenues(), loadEvents()])
    loadUserJoinedEvents()
  })

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

  function loadUserJoinedEvents() {
    if (!$authStore.isAuthenticated) return
    
    const saved = localStorage.getItem('joinedEvents')
    if (saved) {
      const joinedEventIds = JSON.parse(saved)
      userJoinedEvents = events.filter(event => joinedEventIds.includes(event.id))
      
      // Get unique venues from user's joined events
      const venueNames = [...new Set(userJoinedEvents.map(event => event.venue))]
      userEventVenues = venues.filter(venue => 
        venue.name && venueNames.some(venueName => 
          venue.name.toLowerCase().includes(venueName.toLowerCase()) ||
          venueName.toLowerCase().includes(venue.name.toLowerCase())
        )
      )
      
      // Add venues that don't exist in the venues list but are in user events
      venueNames.forEach(venueName => {
        const existsInVenues = venues.some(venue => 
          venue.name && (
            venue.name.toLowerCase().includes(venueName.toLowerCase()) ||
            venueName.toLowerCase().includes(venue.name.toLowerCase())
          )
        )
        
        if (!existsInVenues) {
          // Create a venue entry for events venues not in the main venues list
          const eventsAtVenue = userJoinedEvents.filter(event => event.venue === venueName)
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
    return activeTab === 'my-venues' ? userEventVenues : venues
  }

  function getEventsAtVenue(venueName) {
    return userJoinedEvents.filter(event => 
      event.venue.toLowerCase().includes(venueName.toLowerCase()) ||
      venueName.toLowerCase().includes(event.venue.toLowerCase())
    )
  }

  // Reactive statement to update when auth state changes
  $: if ($authStore.isAuthenticated && events.length > 0) {
    loadUserJoinedEvents()
  }
</script>

<svelte:head>
  <title>Venues - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Amazing Venues</h1>
      <p>Discover iconic venues and places where your events are happening</p>
    </header>

    <!-- Venue Tabs -->
    <div class="tabs">
      <button 
        class="tab-button {activeTab === 'all' ? 'active' : ''}" 
        on:click={() => setActiveTab('all')}
      >
        🌟 All Venues ({venues.length})
      </button>
      
      {#if $authStore.isAuthenticated}
        <button 
          class="tab-button {activeTab === 'my-venues' ? 'active' : ''}" 
          on:click={() => setActiveTab('my-venues')}
        >
          🎫 My Event Venues ({userEventVenues.length})
        </button>
      {/if}
    </div>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading venues...</p>
      </div>
    {:else}
      {@const displayVenues = getDisplayVenues()}
      {#if displayVenues.length > 0}
        <div class="venues-grid">
          {#each displayVenues as venue}
            <div class="venue-card {venue.isFromEvents ? 'event-venue' : ''}">
              <div class="venue-header">
                <div class="venue-image">
                  <div class="venue-overlay">
                    <h3>{venue.name}</h3>
                    <div class="venue-rating">
                      {#if venue.isFromEvents}
                        <span class="event-badge">🎫 Your Events</span>
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
                
                {#if venue.isFromEvents}
                  <div class="my-events-section">
                    <h4>🎭 Your Events at this Venue:</h4>
                    <div class="venue-events">
                      {#each getEventsAtVenue(venue.name) as event}
                        <div class="venue-event-item">
                          <span class="event-name">{event.name}</span>
                          <span class="event-date">
                            {new Date(event.date).toLocaleDateString('en', { 
                              month: 'short', 
                              day: 'numeric' 
                            })}
                          </span>
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
                  {#if venue.isFromEvents}
                    <div class="detail-item">
                      <i class="icon">🎟️</i>
                      <span>Events: {venue.eventsCount} joined</span>
                    </div>
                  {/if}
                </div>
                
                <div class="venue-features">
                  {#if venue.isFromEvents}
                    <span class="feature-tag event-tag">Your Events</span>
                    <span class="feature-tag">Joined Venue</span>
                  {:else}
                    <span class="feature-tag">Dance Floor</span>
                    <span class="feature-tag">Bar</span>
                    <span class="feature-tag">VIP Area</span>
                    <span class="feature-tag">Coat Check</span>
                  {/if}
                </div>
              </div>
              
              <div class="venue-footer">
                {#if venue.isFromEvents}
                  <button class="btn-primary">📅 View My Events</button>
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
          <div class="empty-icon">
            {#if activeTab === 'my-venues'}
              🎫
            {:else}
              🏢
            {/if}
          </div>
          <h3>
            {#if activeTab === 'my-venues'}
              No Event Venues Yet
            {:else}
              No Venues Available
            {/if}
          </h3>
          <p>
            {#if activeTab === 'my-venues'}
              Join some events to see their venues here!
            {:else}
              We're adding more amazing venues soon!
            {/if}
          </p>
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

  .venue-card.event-venue {
    border: 2px solid #48bb78;
    box-shadow: 0 4px 20px rgba(72, 187, 120, 0.15);
  }

  .venue-card.event-venue:hover {
    box-shadow: 0 12px 40px rgba(72, 187, 120, 0.25);
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
    background: rgba(72, 187, 120, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .my-events-section {
    background: #f0fff4;
    border: 2px solid #c6f6d5;
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
    border-left: 4px solid #48bb78;
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