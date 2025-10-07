<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import api from '$lib/api.js';
  
  let packages = [];
  let loading = true;
  let error = null;

  onMount(async () => {
    try {
      const response = await api.get('/api/packages');
      packages = Array.isArray(response.data) ? response.data : [];
      loading = false;
    } catch (err) {
      console.error('Error loading packages:', err);
      error = 'Failed to load packages';
      packages = [];
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
  <title>Event Packages - Berlin DanceMode</title>
  <meta name="description" content="Exclusive event packages for Berlin's electronic music scene" />
</svelte:head>

<div class="packages-page">
  <div class="hero">
    <div class="hero-content">
      <h1>Event Packages</h1>
      <p>Curated experiences for the ultimate night out in Berlin</p>
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
        <p>{error}</p>
        <button on:click={() => window.location.reload()}>Try Again</button>
      </div>
    {:else}
      <div class="packages-grid">
        {#each packages as pkg}
          <div class="package-card" class:featured={pkg.featured}>
            {#if pkg.featured}
              <div class="featured-badge">Most Popular</div>
            {/if}
            
            <div class="package-header">
              <h3>{pkg.name}</h3>
              <div class="price">
                <span class="currency">€</span>
                <span class="amount">{pkg.price}</span>
                <span class="period">per person</span>
              </div>
            </div>
            
            <div class="package-description">
              <p>{pkg.description}</p>
            </div>
            
            <div class="package-features">
              <h4>What's Included:</h4>
              <ul>
                {#each pkg.features || [] as feature}
                  <li>
                    <svg class="check-icon" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    {feature}
                  </li>
                {/each}
              </ul>
            </div>
            
            <div class="package-action">
              <button class="btn-package" class:btn-featured={pkg.featured} on:click={() => goto(`/payment?package=${pkg.id}`)}>
                {pkg.featured ? 'Buy Now - €' + pkg.price : 'Buy Now - €' + pkg.price}
              </button>
            </div>
          </div>
        {/each}
      </div>

      <!-- Custom Package Section -->
      <section class="custom-package">
        <div class="custom-content">
          <h2>Need Something Special?</h2>
          <p>We can create a custom package tailored to your group's preferences and budget.</p>
          <div class="custom-features">
            <div class="custom-feature">
              <div class="custom-icon">🎭</div>
              <h4>VIP Access</h4>
              <p>Backstage passes and artist meet & greets</p>
            </div>
            <div class="custom-feature">
              <div class="custom-icon">🚐</div>
              <h4>Transport</h4>
              <p>Private transportation between venues</p>
            </div>
            <div class="custom-feature">
              <div class="custom-icon">🍾</div>
              <h4>Hospitality</h4>
              <p>Premium drinks and dining experiences</p>
            </div>
            <div class="custom-feature">
              <div class="custom-icon">📸</div>
              <h4>Photography</h4>
              <p>Professional event photography included</p>
            </div>
          </div>
          <button class="btn-custom">Request Custom Package</button>
        </div>
      </section>

      <!-- Package Comparison -->
      <section class="comparison-section">
        <h2>Compare Packages</h2>
        <div class="comparison-table">
          <div class="comparison-header">
            <div class="feature-column">Features</div>
            {#each (Array.isArray(packages) ? packages.slice(0, 3) : []) as pkg}
              <div class="package-column">{pkg.name}</div>
            {/each}
          </div>
          
          <div class="comparison-row">
            <div class="feature-name">Entry to 2+ Venues</div>
            <div class="check">✓</div>
            <div class="check">✓</div>
            <div class="check">✓</div>
          </div>
          
          <div class="comparison-row">
            <div class="feature-name">Welcome Drinks</div>
            <div class="check">✓</div>
            <div class="check">✓</div>
            <div class="check">✓</div>
          </div>
          
          <div class="comparison-row">
            <div class="feature-name">Skip the Line Access</div>
            <div class="cross">✗</div>
            <div class="check">✓</div>
            <div class="check">✓</div>
          </div>
          
          <div class="comparison-row">
            <div class="feature-name">VIP Area Access</div>
            <div class="cross">✗</div>
            <div class="cross">✗</div>
            <div class="check">✓</div>
          </div>
          
          <div class="comparison-row">
            <div class="feature-name">Personal Guide</div>
            <div class="cross">✗</div>
            <div class="cross">✗</div>
            <div class="check">✓</div>
          </div>
        </div>
      </section>

      <!-- Testimonials -->
      <section class="testimonials">
        <h2>What Our Guests Say</h2>
        <div class="testimonials-grid">
          <div class="testimonial">
            <div class="testimonial-content">
              <p>"The Ultimate Experience package was incredible! We got to see Berlin's underground scene like never before."</p>
            </div>
            <div class="testimonial-author">
              <strong>Sarah M.</strong>
              <span>London, UK</span>
            </div>
          </div>
          
          <div class="testimonial">
            <div class="testimonial-content">
              <p>"Perfect introduction to Berlin's techno scene. The guide knew all the best spots and we never waited in line."</p>
            </div>
            <div class="testimonial-author">
              <strong>Marco R.</strong>
              <span>Barcelona, Spain</span>
            </div>
          </div>
          
          <div class="testimonial">
            <div class="testimonial-content">
              <p>"Amazing value for money. We experienced more in one night than we could have in a whole weekend on our own."</p>
            </div>
            <div class="testimonial-author">
              <strong>Jenny K.</strong>
              <span>Amsterdam, Netherlands</span>
            </div>
          </div>
        </div>
      </section>
    {/if}
  </div>
</div>

<style>
  .packages-page {
    min-height: 100vh;
  }

  .hero {
    height: 40vh;
    background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    text-align: center;
  }

  .hero-content h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
    font-weight: 700;
  }

  .hero-content p {
    font-size: 1.2rem;
    opacity: 0.9;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 3rem 2rem;
  }

  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
    margin-bottom: 4rem;
  }

  .package-card {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    border: 2px solid transparent;
    position: relative;
    transition: all 0.3s;
  }

  .package-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  }

  .package-card.featured {
    border-color: #667eea;
    transform: scale(1.05);
  }

  .featured-badge {
    position: absolute;
    top: -1rem;
    left: 50%;
    transform: translateX(-50%);
    background: #667eea;
    color: white;
    padding: 0.5rem 1.5rem;
    border-radius: 20px;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .package-header {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .package-header h3 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .price {
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 0.25rem;
  }

  .currency {
    font-size: 1.25rem;
    color: #4a5568;
  }

  .amount {
    font-size: 2.5rem;
    font-weight: 700;
    color: #667eea;
  }

  .period {
    font-size: 1rem;
    color: #4a5568;
  }

  .package-description {
    margin-bottom: 2rem;
    text-align: center;
  }

  .package-description p {
    color: #4a5568;
    line-height: 1.6;
  }

  .package-features {
    margin-bottom: 2rem;
  }

  .package-features h4 {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .package-features ul {
    list-style: none;
    padding: 0;
  }

  .package-features li {
    display: flex;
    align-items: center;
    margin-bottom: 0.75rem;
    color: #4a5568;
  }

  .check-icon {
    width: 1.25rem;
    height: 1.25rem;
    color: #48bb78;
    margin-right: 0.75rem;
    flex-shrink: 0;
  }

  .btn-package {
    width: 100%;
    padding: 1rem 1.5rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    border: none;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .btn-package:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
    background: linear-gradient(135deg, #5a67d8 0%, #6b46c1 100%);
  }

  .btn-package:active {
    transform: translateY(0);
  }

  .btn-featured {
    background: linear-gradient(135deg, #ffd700 0%, #ff8c00 100%);
    color: #1a202c;
    border: none;
    box-shadow: 0 4px 15px rgba(255, 215, 0, 0.4);
    font-weight: 700;
    animation: pulse 2s infinite;
  }

  .btn-featured:hover {
    background: linear-gradient(135deg, #ffed4e 0%, #ff9500 100%);
    transform: translateY(-3px);
    box-shadow: 0 8px 25px rgba(255, 215, 0, 0.5);
  }

  @keyframes pulse {
    0% { box-shadow: 0 4px 15px rgba(255, 215, 0, 0.4); }
    50% { box-shadow: 0 6px 20px rgba(255, 215, 0, 0.6); }
    100% { box-shadow: 0 4px 15px rgba(255, 215, 0, 0.4); }
  }

  .custom-package {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 16px;
    padding: 3rem 2rem;
    text-align: center;
    color: white;
    margin-bottom: 4rem;
  }

  .custom-content h2 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .custom-content > p {
    font-size: 1.2rem;
    opacity: 0.9;
    margin-bottom: 2rem;
  }

  .custom-features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .custom-feature {
    text-align: center;
  }

  .custom-icon {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  .custom-feature h4 {
    font-size: 1.1rem;
    margin-bottom: 0.5rem;
  }

  .custom-feature p {
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .btn-custom {
    background: white;
    color: #667eea;
    padding: 1rem 2rem;
    border-radius: 12px;
    font-weight: 600;
    border: none;
    cursor: pointer;
    transition: all 0.3s;
  }

  .btn-custom:hover {
    background: #f7fafc;
  }

  .comparison-section {
    margin-bottom: 4rem;
  }

  .comparison-section h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .comparison-table {
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .comparison-header {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr;
    background: #667eea;
    color: white;
    font-weight: 600;
  }

  .comparison-header > div {
    padding: 1.5rem;
    text-align: center;
  }

  .feature-column {
    text-align: left !important;
  }

  .comparison-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr;
    border-bottom: 1px solid #e2e8f0;
  }

  .comparison-row > div {
    padding: 1rem 1.5rem;
    text-align: center;
  }

  .feature-name {
    text-align: left !important;
    font-weight: 500;
    color: #1a202c;
  }

  .check {
    color: #48bb78;
    font-weight: 700;
  }

  .cross {
    color: #f56565;
    font-weight: 700;
  }

  .testimonials {
    text-align: center;
  }

  .testimonials h2 {
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .testimonials-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .testimonial {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .testimonial-content p {
    font-style: italic;
    color: #4a5568;
    line-height: 1.6;
    margin-bottom: 1rem;
  }

  .testimonial-author strong {
    color: #1a202c;
    display: block;
  }

  .testimonial-author span {
    color: #4a5568;
    font-size: 0.875rem;
  }

  .loading, .error {
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

  @media (max-width: 768px) {
    .hero-content h1 {
      font-size: 2rem;
    }

    .packages-grid {
      grid-template-columns: 1fr;
    }

    .package-card.featured {
      transform: none;
    }

    .comparison-header,
    .comparison-row {
      grid-template-columns: 1fr;
    }

    .comparison-header > div:not(.feature-column),
    .comparison-row > div:not(.feature-name) {
      display: none;
    }

    .custom-features {
      grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    }
  }
</style>