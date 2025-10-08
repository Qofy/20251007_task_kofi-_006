<script>
  import { onMount } from 'svelte'
  import api from '../services/api.js'

  let packages = []
  let loading = true

  onMount(async () => {
    try {
      const response = await api.packages.getAll()
      packages = response.data || []
    } catch (error) {
      console.error('Error loading packages:', error)
    } finally {
      loading = false
    }
  })
</script>

<svelte:head>
  <title>Packages - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Exclusive Packages</h1>
      <p>Choose from our carefully curated packages for the ultimate Berlin dance experience</p>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading packages...</p>
      </div>
    {:else if packages.length > 0}
      <div class="packages-grid">
        {#each packages as pkg}
          <div class="package-card">
            <div class="package-header">
              <h3>{pkg.name}</h3>
              <div class="package-badge">Popular</div>
            </div>
            <div class="package-price">
              <span class="currency">€</span>
              <span class="amount">{pkg.price}</span>
              <span class="period">/person</span>
            </div>
            <div class="package-content">
              <p class="package-description">{pkg.description}</p>
              <div class="features-section">
                <h4>What's Included:</h4>
                <ul class="features-list">
                  {#each pkg.features || ['Event access', 'Welcome drink', 'VIP area access'] as feature}
                    <li>{feature}</li>
                  {/each}
                </ul>
              </div>
            </div>
            <div class="package-footer">
              <button class="btn-primary">Select Package</button>
              <p class="terms">No hidden fees • Cancel anytime</p>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">📦</div>
        <h3>No Packages Available</h3>
        <p>We're working on exciting new packages for you!</p>
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

  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
  }

  .package-card {
    background: white;
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
    position: relative;
  }

  .package-card:hover {
    transform: translateY(-8px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
  }

  .package-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 2rem;
    text-align: center;
    position: relative;
  }

  .package-badge {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(255, 255, 255, 0.2);
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    backdrop-filter: blur(10px);
  }

  .package-header h3 {
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
  }

  .package-price {
    text-align: center;
    padding: 2rem;
    background: #f8fafc;
    border-bottom: 1px solid #e2e8f0;
  }

  .currency {
    font-size: 1.5rem;
    color: #718096;
    vertical-align: top;
  }

  .amount {
    font-size: 3rem;
    font-weight: bold;
    color: #2d3748;
  }

  .period {
    font-size: 1rem;
    color: #718096;
  }

  .package-content {
    padding: 2rem;
  }

  .package-description {
    color: #718096;
    line-height: 1.6;
    margin-bottom: 2rem;
    text-align: center;
  }

  .features-section h4 {
    color: #2d3748;
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .features-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .features-list li {
    padding: 0.75rem 0;
    color: #4a5568;
    position: relative;
    padding-left: 2rem;
  }

  .features-list li:before {
    content: "✓";
    position: absolute;
    left: 0;
    color: #667eea;
    font-weight: bold;
    font-size: 1.2rem;
  }

  .features-list li:not(:last-child) {
    border-bottom: 1px solid #f1f5f9;
  }

  .package-footer {
    padding: 2rem;
    background: #f8fafc;
    text-align: center;
  }

  .btn-primary {
    width: 100%;
    padding: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 12px;
    font-weight: 600;
    font-size: 1.1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 1rem;
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  }

  .terms {
    color: #718096;
    font-size: 0.9rem;
    margin: 0;
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

    .packages-grid {
      grid-template-columns: 1fr;
    }

    .amount {
      font-size: 2.5rem;
    }
  }
</style>