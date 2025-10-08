<script>
  import { onMount } from 'svelte'

  let packages = [
    {
      id: 1,
      name: "Weekend Warrior",
      price: 89,
      description: "Perfect for newcomers to Berlin's dance scene. Experience the city's underground culture with guided access to the best clubs.",
      badge: "Most Popular",
      features: [
        "Access to 3 premium clubs (Berghain, Watergate, Tresor)",
        "Skip-the-line priority entry",
        "Welcome drink at each venue",
        "Professional club guide for first-timers",
        "Digital map with club locations",
        "24/7 emergency support hotline",
        "Exclusive DanceMode wristband"
      ],
      duration: "2 nights",
      includes: "Friday & Saturday night events"
    },
    {
      id: 2,
      name: "Techno Devotee",
      price: 145,
      description: "For serious techno enthusiasts. Deep dive into Berlin's legendary techno temples with VIP treatment and exclusive areas.",
      badge: "VIP Experience",
      features: [
        "VIP entry to 5 legendary techno venues",
        "Backstage meet & greet with resident DJs",
        "Reserved table in VIP lounge areas",
        "Premium cocktail package (5 drinks)",
        "Exclusive after-party invitations",
        "Limited edition Berghain merchandise",
        "Personal concierge service",
        "Photo session in iconic club locations"
      ],
      duration: "3 nights",
      includes: "Thursday, Friday & Saturday events"
    },
    {
      id: 3,
      name: "Berlin Explorer",
      price: 199,
      description: "The ultimate Berlin dance experience. Explore diverse music scenes from techno to house, electronic to experimental across the city.",
      badge: "Complete Experience",
      features: [
        "Access to 8 different venues across all districts",
        "Multi-genre experience (Techno, House, Minimal, Experimental)",
        "Private transportation between venues",
        "Gourmet dinner at renowned Berlin restaurant",
        "Meet local DJs and producers",
        "Studio tour at iconic electronic music labels",
        "Exclusive vinyl record from Berlin Underground",
        "Professional photographer for memories",
        "Customized playlist from your weekend"
      ],
      duration: "Full weekend",
      includes: "Thursday through Sunday events + cultural activities"
    },
    {
      id: 4,
      name: "Culture & Beats",
      price: 115,
      description: "Combine Berlin's rich cultural heritage with its modern electronic music scene. Perfect balance of day culture and night beats.",
      badge: "Cultural Fusion",
      features: [
        "Guided tour of Berlin Wall and electronic music history",
        "Visit to iconic record stores (Spacehall, Hard Wax)",
        "Access to 4 diverse venues (clubs + cultural spaces)",
        "Workshop: 'Introduction to Electronic Music Production'",
        "Traditional German dinner with electronic ambient",
        "Museum pass for music and contemporary art",
        "Local artist networking session",
        "Souvenir package with Berlin music memorabilia"
      ],
      duration: "2 days, 2 nights",
      includes: "Cultural activities + weekend nightlife"
    }
  ]

  let loading = false
  let selectedPackage = null

  function selectPackage(pkg) {
    selectedPackage = pkg
    // Here you would typically integrate with a booking system
    alert(`Selected: ${pkg.name} - €${pkg.price}. Booking system integration coming soon!`)
  }
</script>

<svelte:head>
  <title>Packages - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Berlin Dance Packages</h1>
      <p>Immerse yourself in Berlin's legendary electronic music scene with our expertly curated experiences. From techno temples to underground culture, discover the heartbeat of Europe's dance capital.</p>
      <div class="header-stats">
        <div class="stat">
          <span class="stat-number">20+</span>
          <span class="stat-label">Premium Venues</span>
        </div>
        <div class="stat">
          <span class="stat-number">50+</span>
          <span class="stat-label">World-Class DJs</span>
        </div>
        <div class="stat">
          <span class="stat-number">24/7</span>
          <span class="stat-label">Support</span>
        </div>
      </div>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading packages...</p>
      </div>
    {:else if packages.length > 0}
      <div class="packages-grid">
        {#each packages as pkg}
          <div class="package-card" class:featured={pkg.badge === 'Most Popular'}>
            <div class="package-header">
              <h3>{pkg.name}</h3>
              <div class="package-badge {pkg.badge === 'Most Popular' ? 'popular' : pkg.badge === 'VIP Experience' ? 'vip' : pkg.badge === 'Complete Experience' ? 'complete' : 'cultural'}">{pkg.badge}</div>
            </div>
            <div class="package-price">
              <span class="currency">€</span>
              <span class="amount">{pkg.price}</span>
              <span class="period">/person</span>
            </div>
            <div class="package-details">
              <div class="duration-info">
                <span class="duration-label">Duration:</span>
                <span class="duration-value">{pkg.duration}</span>
              </div>
              <div class="includes-info">
                <span class="includes-label">Includes:</span>
                <span class="includes-value">{pkg.includes}</span>
              </div>
            </div>
            <div class="package-content">
              <p class="package-description">{pkg.description}</p>
              <div class="features-section">
                <h4>What's Included:</h4>
                <ul class="features-list">
                  {#each pkg.features as feature}
                    <li>{feature}</li>
                  {/each}
                </ul>
              </div>
            </div>
            <div class="package-footer">
              <button class="btn-primary" on:click={() => selectPackage(pkg)}>
                Select {pkg.name} Package
              </button>
              <p class="terms">Free cancellation up to 24h • Secure payment</p>
            </div>
          </div>
        {/each}
      </div>
      
      <!-- Additional Information Section -->
      <div class="info-section">
        <div class="info-grid">
          <div class="info-card">
            <div class="info-icon">🎧</div>
            <h3>Expert Curation</h3>
            <p>Our packages are designed by Berlin nightlife experts who know the city's hidden gems and legendary venues.</p>
          </div>
          <div class="info-card">
            <div class="info-icon">🔒</div>
            <h3>Guaranteed Entry</h3>
            <p>Skip the uncertainty. All our packages include guaranteed entry to venues, even on the busiest nights.</p>
          </div>
          <div class="info-card">
            <div class="info-icon">🌟</div>
            <h3>Local Connections</h3>
            <p>Connect with Berlin's electronic music community through exclusive events and networking opportunities.</p>
          </div>
        </div>
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
    max-width: 700px;
    margin: 0 auto 2rem auto;
    line-height: 1.6;
  }

  .header-stats {
    display: flex;
    justify-content: center;
    gap: 3rem;
    margin-top: 2rem;
  }

  .stat {
    text-align: center;
  }

  .stat-number {
    display: block;
    font-size: 2rem;
    font-weight: bold;
    color: #667eea;
    margin-bottom: 0.5rem;
  }

  .stat-label {
    font-size: 0.9rem;
    color: #718096;
    font-weight: 500;
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
    border: 2px solid transparent;
  }

  .package-card.featured {
    border: 2px solid #667eea;
    box-shadow: 0 8px 30px rgba(102, 126, 234, 0.2);
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
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    backdrop-filter: blur(10px);
  }

  .package-badge.popular {
    background: rgba(255, 215, 0, 0.9);
    color: #333;
  }

  .package-badge.vip {
    background: rgba(255, 20, 147, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  .package-badge.complete {
    background: rgba(34, 197, 94, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  .package-badge.cultural {
    background: rgba(168, 85, 247, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
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

  .package-details {
    padding: 1.5rem 2rem;
    background: #f8fafc;
    border-bottom: 1px solid #e2e8f0;
  }

  .duration-info, .includes-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .includes-info {
    margin-bottom: 0;
  }

  .duration-label, .includes-label {
    font-weight: 600;
    color: #4a5568;
    font-size: 0.9rem;
  }

  .duration-value, .includes-value {
    color: #667eea;
    font-weight: 500;
    font-size: 0.9rem;
    text-align: right;
    max-width: 60%;
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

  .info-section {
    margin-top: 4rem;
    padding: 3rem 0;
    background: white;
    border-radius: 20px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    padding: 0 2rem;
  }

  .info-card {
    text-align: center;
    padding: 2rem;
  }

  .info-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .info-card h3 {
    color: #2d3748;
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .info-card p {
    color: #718096;
    line-height: 1.6;
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

    .info-grid {
      grid-template-columns: 1fr;
      padding: 0 1rem;
    }

    .duration-value, .includes-value {
      max-width: 50%;
      font-size: 0.8rem;
    }

    .header-stats {
      gap: 2rem;
    }

    .stat-number {
      font-size: 1.5rem;
    }
  }
</style>