<script>
  import { onMount } from 'svelte';
  import api from '../lib/api.js';
  
  let packages = [];
  let loading = true;
  let error = null;
  
  onMount(async () => {
    try {
      const response = await api.get('/api/packages');
      packages = response.data.data || [];
    } catch (err) {
      console.error('Failed to load packages:', err);
      error = 'Failed to load packages. Please try again later.';
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
</script>

<svelte:head>
  <title>Experience Packages - Berlin DanceMode</title>
  <meta name="description" content="Choose from our curated electronic music experience packages designed for the ultimate Berlin adventure." />
</svelte:head>

<div class="packages-page">
  <div class="page-header">
    <div class="container">
      <h1>Experience Packages</h1>
      <p>Curated electronic music experiences for the ultimate Berlin adventure</p>
    </div>
  </div>

  <div class="container">
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading packages...</p>
      </div>
    {:else if error}
      <div class="error">
        <h3>Error</h3>
        <p>{error}</p>
      </div>
    {:else}
      <div class="packages-grid">
        {#each packages as pkg}
          <div class="package-card">
            <div class="package-header">
              <h3>{pkg.name}</h3>
              <div class="package-price">
                <span class="price-currency">€</span>
                <span class="price-amount">{pkg.price}</span>
                <span class="price-period">per person</span>
              </div>
            </div>
            <div class="package-content">
              <p class="package-description">{pkg.description}</p>
              <div class="package-features">
                <div class="feature">
                  <strong>Duration:</strong> {pkg.duration_days} day{pkg.duration_days > 1 ? 's' : ''}
                </div>
                <div class="feature">
                  <strong>Max Participants:</strong> {pkg.max_participants}
                </div>
              </div>
              <button class="btn btn-primary">Choose Package</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .packages-page {
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

  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 2rem;
    margin: 3rem 0;
  }

  .package-card {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s, box-shadow 0.3s;
  }

  .package-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  }

  .package-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .package-header h3 {
    font-size: 1.5rem;
    color: #1a202c;
    margin-bottom: 1rem;
  }

  .package-price {
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 0.25rem;
  }

  .price-currency {
    font-size: 1.25rem;
    color: #4a5568;
  }

  .price-amount {
    font-size: 2.5rem;
    font-weight: 700;
    color: #667eea;
  }

  .price-period {
    color: #4a5568;
  }

  .package-description {
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 2rem;
  }

  .package-features {
    margin-bottom: 2rem;
  }

  .feature {
    color: #4a5568;
    margin-bottom: 1rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e2e8f0;
  }

  .feature:last-child {
    border-bottom: none;
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
    .packages-grid {
      grid-template-columns: 1fr;
    }
  }
</style>