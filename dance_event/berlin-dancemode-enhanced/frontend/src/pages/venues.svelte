<script>
  import { onMount } from 'svelte'
  import api from '../services/api.js'

  let venues = []
  let loading = true

  onMount(async () => {
    try {
      const response = await api.venues.getAll()
      venues = response.data || []
    } catch (error) {
      console.error('Error loading venues:', error)
    } finally {
      loading = false
    }
  })
</script>

<svelte:head>
  <title>Venues - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Amazing Venues</h1>
      <p>Discover the most iconic dance venues and clubs in Berlin</p>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading venues...</p>
      </div>
    {:else if venues.length > 0}
      <div class="venues-grid">
        {#each venues as venue}
          <div class="venue-card">
            <div class="venue-header">
              <div class="venue-image">
                <div class="venue-overlay">
                  <h3>{venue.name}</h3>
                  <div class="venue-rating">
                    <span class="stars">★★★★★</span>
                    <span class="rating-text">4.8</span>
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
              </div>
              <div class="venue-features">
                <span class="feature-tag">Dance Floor</span>
                <span class="feature-tag">Bar</span>
                <span class="feature-tag">VIP Area</span>
                <span class="feature-tag">Coat Check</span>
              </div>
            </div>
            <div class="venue-footer">
              <button class="btn-primary">View Events</button>
              <button class="btn-secondary">Get Directions</button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">🏢</div>
        <h3>No Venues Available</h3>
        <p>We're adding more amazing venues soon!</p>
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
  }
</style>