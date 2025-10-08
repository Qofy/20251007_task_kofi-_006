<script>
  import { onMount } from 'svelte'
  import api from '../services/api.js'

  let events = []
  let loading = true

  onMount(async () => {
    try {
      const response = await api.events.getAll()
      events = response.data || []
    } catch (error) {
      console.error('Error loading events:', error)
    } finally {
      loading = false
    }
  })
</script>

<svelte:head>
  <title>Events - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Dance Events</h1>
      <p>Discover the hottest dance events in Berlin's vibrant nightlife scene</p>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading events...</p>
      </div>
    {:else if events.length > 0}
      <div class="events-grid">
        {#each events as event}
          <div class="event-card">
            <div class="event-image">
              <div class="event-date">
                <span class="day">{new Date(event.date).getDate()}</span>
                <span class="month">{new Date(event.date).toLocaleDateString('en', { month: 'short' })}</span>
              </div>
              <div class="event-category">{event.category || 'Dance'}</div>
            </div>
            <div class="event-content">
              <h3>{event.name}</h3>
              <p class="venue">{event.venue}</p>
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
              </div>
              <button class="btn-primary">Book Now</button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">🎭</div>
        <h3>No Events Available</h3>
        <p>Check back soon for exciting dance events!</p>
      </div>
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
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: #f7fafc;
    border-radius: 8px;
  }

  .time, .price {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #4a5568;
    font-weight: 500;
  }

  .price {
    color: #667eea;
    font-weight: bold;
    font-size: 1.1rem;
  }

  .btn-primary {
    width: 100%;
    padding: 0.75rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
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
  }
</style>