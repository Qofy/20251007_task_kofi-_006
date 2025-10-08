<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'

  let events = []
  let userJoinedEvents = []
  let loading = true
  let showCreateForm = false
  let activeTab = 'all' // 'all', 'my-events', 'joined'

  // Event creation form data
  let newEvent = {
    name: '',
    description: '',
    venue: '',
    date: '',
    time: '',
    price: '',
    category: 'Dance',
    capacity: 100
  }

  // User's joined events stored in localStorage
  let joinedEventIds = []

  onMount(async () => {
    await loadEvents()
    loadJoinedEvents()
  })

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

  function loadJoinedEvents() {
    const saved = localStorage.getItem('joinedEvents')
    if (saved) {
      joinedEventIds = JSON.parse(saved)
    }
    updateUserJoinedEvents()
  }

  function updateUserJoinedEvents() {
    userJoinedEvents = events.filter(event => joinedEventIds.includes(event.id))
  }

  function saveJoinedEvents() {
    localStorage.setItem('joinedEvents', JSON.stringify(joinedEventIds))
  }

  async function createEvent() {
    if (!$authStore.isAuthenticated) {
      alert('Please login to create events')
      return
    }

    try {
      const eventData = {
        ...newEvent,
        date: `${newEvent.date}T${newEvent.time}:00.000Z`,
        creator_id: $authStore.user?.id || 'user-' + Date.now(),
        created_at: new Date().toISOString()
      }

      const response = await api.events.create(eventData)
      
      // Add to local events array
      events = [...events, { ...eventData, id: Date.now().toString() }]
      
      // Reset form
      newEvent = {
        name: '',
        description: '',
        venue: '',
        date: '',
        time: '',
        price: '',
        category: 'Dance',
        capacity: 100
      }
      
      showCreateForm = false
      alert('Event created successfully!')
    } catch (error) {
      console.error('Error creating event:', error)
      alert('Failed to create event. Please try again.')
    }
  }

  function joinEvent(event) {
    if (!$authStore.isAuthenticated) {
      alert('Please login to join events')
      return
    }

    if (!joinedEventIds.includes(event.id)) {
      joinedEventIds = [...joinedEventIds, event.id]
      saveJoinedEvents()
      updateUserJoinedEvents()
      alert(`Successfully joined "${event.name}"!`)
    } else {
      alert('You have already joined this event!')
    }
  }

  function leaveEvent(eventId) {
    joinedEventIds = joinedEventIds.filter(id => id !== eventId)
    saveJoinedEvents()
    updateUserJoinedEvents()
    alert('Left event successfully!')
  }

  function isEventJoined(eventId) {
    return joinedEventIds.includes(eventId)
  }

  function getUserCreatedEvents() {
    if (!$authStore.user) return []
    return events.filter(event => event.creator_id === $authStore.user.id)
  }

  function setActiveTab(tab) {
    activeTab = tab
  }

  function getDisplayEvents() {
    switch (activeTab) {
      case 'my-events':
        return getUserCreatedEvents()
      case 'joined':
        return userJoinedEvents
      default:
        return events
    }
  }

  // Reactive statement to update joined events when events change
  $: if (events.length > 0 && joinedEventIds.length > 0) {
    updateUserJoinedEvents()
  }
</script>

<svelte:head>
  <title>Events - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Dance Events</h1>
      <p>Discover, create, and join the hottest dance events in Berlin</p>
      
      {#if $authStore.isAuthenticated}
        <div class="header-actions">
          <button class="btn-create" on:click={() => showCreateForm = !showCreateForm}>
            {showCreateForm ? '❌ Cancel' : '➕ Create Event'}
          </button>
        </div>
      {/if}
    </header>

    <!-- Event Tabs -->
    <div class="tabs">
      <button 
        class="tab-button {activeTab === 'all' ? 'active' : ''}" 
        on:click={() => setActiveTab('all')}
      >
        🌟 All Events ({events.length})
      </button>
      
      {#if $authStore.isAuthenticated}
        <button 
          class="tab-button {activeTab === 'my-events' ? 'active' : ''}" 
          on:click={() => setActiveTab('my-events')}
        >
          📝 My Events ({getUserCreatedEvents().length})
        </button>
        
        <button 
          class="tab-button {activeTab === 'joined' ? 'active' : ''}" 
          on:click={() => setActiveTab('joined')}
        >
          🎫 Joined Events ({userJoinedEvents.length})
        </button>
      {/if}
    </div>

    <!-- Create Event Form -->
    {#if showCreateForm && $authStore.isAuthenticated}
      <div class="create-form-container">
        <form class="create-form" on:submit|preventDefault={createEvent}>
          <h3>🎉 Create New Event</h3>
          
          <div class="form-row">
            <div class="form-group">
              <label for="eventName">Event Name</label>
              <input 
                id="eventName"
                type="text" 
                bind:value={newEvent.name} 
                placeholder="Enter event name"
                required
              />
            </div>
            
            <div class="form-group">
              <label for="category">Category</label>
              <select id="category" bind:value={newEvent.category}>
                <option value="Dance">Dance</option>
                <option value="Blues">Blues</option>
                <option value="Fusion">Fusion</option>
                <option value="Workshop">Workshop</option>
                <option value="Social">Social</option>
                <option value="Competition">Competition</option>
              </select>
            </div>
          </div>

          <div class="form-group">
            <label for="venue">Venue</label>
            <input 
              id="venue"
              type="text" 
              bind:value={newEvent.venue} 
              placeholder="Enter venue name"
              required
            />
          </div>

          <div class="form-group">
            <label for="description">Description</label>
            <textarea 
              id="description"
              bind:value={newEvent.description} 
              placeholder="Describe your event..."
              rows="3"
              required
            ></textarea>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label for="date">Date</label>
              <input 
                id="date"
                type="date" 
                bind:value={newEvent.date} 
                required
              />
            </div>
            
            <div class="form-group">
              <label for="time">Time</label>
              <input 
                id="time"
                type="time" 
                bind:value={newEvent.time} 
                required
              />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label for="price">Price (€)</label>
              <input 
                id="price"
                type="number" 
                bind:value={newEvent.price} 
                placeholder="0"
                min="0"
                step="0.01"
                required
              />
            </div>
            
            <div class="form-group">
              <label for="capacity">Capacity</label>
              <input 
                id="capacity"
                type="number" 
                bind:value={newEvent.capacity} 
                placeholder="100"
                min="1"
                required
              />
            </div>
          </div>

          <div class="form-actions">
            <button type="submit" class="btn-primary">🚀 Create Event</button>
            <button type="button" class="btn-secondary" on:click={() => showCreateForm = false}>
              Cancel
            </button>
          </div>
        </form>
      </div>
    {/if}

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading events...</p>
      </div>
    {:else}
      {@const displayEvents = getDisplayEvents()}
      {#if displayEvents.length > 0}
        <div class="events-grid">
          {#each displayEvents as event}
            <div class="event-card">
              <div class="event-image">
                <div class="event-date">
                  <span class="day">{new Date(event.date).getDate()}</span>
                  <span class="month">{new Date(event.date).toLocaleDateString('en', { month: 'short' })}</span>
                </div>
                <div class="event-category">{event.category || 'Dance'}</div>
                
                {#if event.creator_id === $authStore.user?.id}
                  <div class="creator-badge">👑 Your Event</div>
                {/if}
                
                {#if isEventJoined(event.id)}
                  <div class="joined-badge">✅ Joined</div>
                {/if}
              </div>
              
              <div class="event-content">
                <h3>{event.name}</h3>
                <p class="venue">📍 {event.venue}</p>
                <p class="description">{event.description}</p>
                
                <div class="event-details">
                  <div class="time">
                    <i class="icon">🕒</i>
                    {new Date(event.date).toLocaleTimeString('en', { hour: '2-digit', minute: '2-digit' })}
                  </div>
                  <div class="price">
                    <i class="icon">💰</i>
                    €{event.price}
                  </div>
                  <div class="capacity">
                    <i class="icon">👥</i>
                    {event.capacity} spots
                  </div>
                </div>

                <div class="event-actions">
                  {#if !$authStore.isAuthenticated}
                    <button class="btn-primary">Login to Join</button>
                  {:else if event.creator_id === $authStore.user?.id}
                    <button class="btn-secondary">📊 Manage Event</button>
                  {:else if isEventJoined(event.id)}
                    <button class="btn-danger" on:click={() => leaveEvent(event.id)}>
                      ❌ Leave Event
                    </button>
                  {:else}
                    <button class="btn-primary" on:click={() => joinEvent(event)}>
                      🎫 Join Event
                    </button>
                  {/if}
                </div>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="empty-state">
          <div class="empty-icon">
            {#if activeTab === 'my-events'}
              📝
            {:else if activeTab === 'joined'}
              🎫
            {:else}
              🎭
            {/if}
          </div>
          <h3>
            {#if activeTab === 'my-events'}
              No Events Created Yet
            {:else if activeTab === 'joined'}
              No Events Joined Yet
            {:else}
              No Events Available
            {/if}
          </h3>
          <p>
            {#if activeTab === 'my-events'}
              Create your first event to get started!
            {:else if activeTab === 'joined'}
              Join some events to see them here!
            {:else}
              Check back soon for exciting dance events!
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
    margin: 0 auto 1.5rem;
  }

  .header-actions {
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  .btn-create {
    background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-create:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(72, 187, 120, 0.4);
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

  /* Create Form */
  .create-form-container {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    margin-bottom: 2rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    border: 2px solid #e2e8f0;
  }

  .create-form h3 {
    color: #2d3748;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #4a5568;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.2s ease;
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: #667eea;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1.5rem;
  }

  .form-actions button {
    flex: 1;
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

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 2rem;
  }

  .event-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .event-card:hover {
    transform: translateY(-8px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
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
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: white;
    border-radius: 12px;
    padding: 0.75rem;
    text-align: center;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }

  .event-date .day {
    display: block;
    font-size: 1.5rem;
    font-weight: bold;
    color: #2d3748;
  }

  .event-date .month {
    display: block;
    font-size: 0.8rem;
    color: #718096;
    text-transform: uppercase;
    font-weight: 600;
  }

  .event-category {
    position: absolute;
    top: 1rem;
    left: 1rem;
    background: rgba(255, 255, 255, 0.2);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    backdrop-filter: blur(10px);
  }

  .creator-badge {
    position: absolute;
    bottom: 1rem;
    left: 1rem;
    background: rgba(255, 215, 0, 0.9);
    color: #2d3748;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    backdrop-filter: blur(10px);
  }

  .joined-badge {
    position: absolute;
    bottom: 1rem;
    right: 1rem;
    background: rgba(72, 187, 120, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    backdrop-filter: blur(10px);
  }

  .event-content {
    padding: 1.5rem;
  }

  .event-content h3 {
    font-size: 1.25rem;
    font-weight: bold;
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .venue {
    color: #667eea;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .description {
    color: #718096;
    line-height: 1.6;
    margin-bottom: 1.5rem;
  }

  .event-details {
    display: flex;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: #f7fafc;
    border-radius: 8px;
  }

  .time, .price, .capacity {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #4a5568;
    font-weight: 500;
    font-size: 0.9rem;
  }

  .price {
    color: #667eea;
    font-weight: bold;
    font-size: 1rem;
  }

  .event-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-primary, .btn-secondary, .btn-danger {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.9rem;
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

  .btn-danger {
    background: linear-gradient(135deg, #f56565 0%, #e53e3e 100%);
    color: white;
  }

  .btn-danger:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(245, 101, 101, 0.4);
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

    .events-grid {
      grid-template-columns: 1fr;
    }

    .tabs {
      flex-direction: column;
      align-items: stretch;
    }

    .tab-button {
      margin-bottom: 0.5rem;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .create-form-container {
      margin: 1rem;
      padding: 1.5rem;
    }

    .form-actions {
      flex-direction: column;
    }

    .event-details {
      flex-direction: column;
      gap: 0.75rem;
    }

    .event-actions {
      flex-direction: column;
    }

    .creator-badge,
    .joined-badge {
      position: static;
      display: inline-block;
      margin: 0.5rem 0;
    }
  }
</style>