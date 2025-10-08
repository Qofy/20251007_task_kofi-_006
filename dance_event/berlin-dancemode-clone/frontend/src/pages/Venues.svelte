<script>
  import { onMount } from 'svelte';
  import api from '../lib/api.js';
  
  let venues = [];
  let loading = true;
  let error = null;
  
  onMount(async () => {
    try {
      const response = await api.get('/api/venues');
      venues = response.data.data || [];
    } catch (err) {
      console.error('Failed to load venues:', err);
      error = 'Failed to load venues. Please try again later.';
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>Venues - Berlin DanceMode</title>
  <meta name="description" content="Explore Berlin's legendary electronic music venues, from world-famous clubs to underground locations." />
</svelte:head>

<div class="venues-page">
  <div class="page-header">
    <div class="container">
      <h1>Legendary Venues</h1>
      <p>The clubs that shaped electronic music history</p>
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
        <h3>Error</h3>
        <p>{error}</p>
      </div>
    {:else}
      <div class="venues-grid">
        {#each venues as venue}
          <div class="venue-card">
            <div class="venue-image">
              <img src="https://picsum.photos/500/300?random={venue.id}" alt={venue.name} />
            </div>
            <div class="venue-content">
              <h3>{venue.name}</h3>
              <p class="venue-address">{venue.address}</p>
              <p class="venue-description">{venue.description || 'A legendary venue in Berlin\'s electronic music scene.'}</p>
              <div class="venue-stats">
                <div class="stat">
                  <span class="stat-label">Capacity</span>
                  <span class="stat-value">{venue.capacity}</span>
                </div>
              </div>
              <button class="btn btn-primary">Learn More</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .venues-page {
    padding-top: 80px;
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

  .venues-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 2rem;
    margin: 3rem 0;
  }

  .venue-card {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s, box-shadow 0.3s;
  }

  .venue-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  }

  .venue-image {
    height: 250px;
    overflow: hidden;
  }

  .venue-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .venue-content {
    padding: 2rem;
  }

  .venue-content h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .venue-address {
    color: #667eea;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .venue-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1.5rem;
  }

  .venue-stats {
    margin-bottom: 2rem;
  }

  .stat {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .stat-label {
    color: #4a5568;
    font-weight: 600;
  }

  .stat-value {
    color: #1a202c;
    font-weight: 700;
    font-size: 1.1rem;
  }

  .btn {
    width: 100%;
    padding: 1rem 2rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.3s;
  }

  .btn:hover {
    background: #5a67d8;
  }

  .loading,
  .error {
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

  @media (max-width: 768px) {
    .venues-grid {
      grid-template-columns: 1fr;
    }
  }
</style>