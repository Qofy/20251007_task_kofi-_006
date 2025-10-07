<script>
  import { onMount } from 'svelte';
  
  let currentYear = new Date().getFullYear();
  let stats = {
    events: 2500,
    venues: 150,
    visitors: 75000,
    years: currentYear - 2020
  };
  
  let teamMembers = [
    {
      name: 'Alex Müller',
      role: 'Founder & CEO',
      bio: 'Berlin native with 15+ years in the electronic music scene. Former resident DJ at legendary clubs.',
      image: 'https://picsum.photos/200/200?random=1'
    },
    {
      name: 'Sarah Chen',
      role: 'Head of Events',
      bio: 'Event curator with connections to underground venues across Berlin. Speaks fluent techno.',
      image: 'https://picsum.photos/200/200?random=2'
    },
    {
      name: 'Marco Rodriguez',
      role: 'Community Manager',
      bio: 'Former music journalist turned community builder. Knows every doorman in Kreuzberg.',
      image: 'https://picsum.photos/200/200?random=3'
    }
  ];
</script>

<svelte:head>
  <title>About Us - Berlin DanceMode</title>
  <meta name="description" content="Learn about Berlin DanceMode - your guide to authentic electronic music experiences in Berlin's underground scene" />
</svelte:head>

<div class="about-page">
  <!-- Hero Section -->
  <div class="hero">
    <div class="hero-content">
      <h1>About Berlin DanceMode</h1>
      <p>Your authentic guide to Berlin's underground electronic music scene</p>
    </div>
  </div>

  <div class="container">
    <!-- Story Section -->
    <section class="story-section">
      <div class="story-grid">
        <div class="story-text">
          <h2>Our Story</h2>
          <p>Berlin DanceMode was born from a passion for electronic music and a deep love for Berlin's unique club culture. Founded in 2020, we started as a small group of music enthusiasts who wanted to share the authentic Berlin underground experience with visitors from around the world.</p>
          
          <p>What began as informal tours for friends quickly grew into something bigger. We realized that many people were missing out on Berlin's true electronic music culture because they didn't know where to go or how to navigate the complex social dynamics of the city's underground scene.</p>
          
          <p>Today, we're proud to be Berlin's leading curator of authentic electronic music experiences, connecting thousands of music lovers with the city's most legendary venues and emerging underground spaces.</p>
        </div>
        
        <div class="story-image">
                    <img src="https://picsum.photos/500/400" alt="Berlin nightlife scene" />
        </div>
      </div>
    </section>

    <!-- Mission Section -->
    <section class="mission-section">
      <div class="mission-content">
        <h2>Our Mission</h2>
        <div class="mission-grid">
          <div class="mission-item">
            <div class="mission-icon">🎵</div>
            <h3>Preserve Culture</h3>
            <p>We're committed to preserving and promoting Berlin's unique electronic music culture while respecting its underground roots.</p>
          </div>
          
          <div class="mission-item">
            <div class="mission-icon">🌍</div>
            <h3>Connect Communities</h3>
            <p>We bridge the gap between local scene insiders and international music lovers, creating genuine connections.</p>
          </div>
          
          <div class="mission-item">
            <div class="mission-icon">✨</div>
            <h3>Authentic Experiences</h3>
            <p>Every experience we offer is carefully curated to provide authentic insights into Berlin's electronic music world.</p>
          </div>
          
          <div class="mission-item">
            <div class="mission-icon">🤝</div>
            <h3>Support Artists</h3>
            <p>We work directly with venues, artists, and promoters to support the local electronic music ecosystem.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Stats Section -->
    <section class="stats-section">
      <h2>By the Numbers</h2>
      <div class="stats-grid">
        <div class="stat-item">
          <div class="stat-number">{stats.events.toLocaleString()}+</div>
          <div class="stat-label">Events Curated</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">{stats.venues}+</div>
          <div class="stat-label">Partner Venues</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">{stats.visitors.toLocaleString()}+</div>
          <div class="stat-label">Happy Visitors</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">{stats.years}</div>
          <div class="stat-label">Years of Experience</div>
        </div>
      </div>
    </section>

    <!-- Team Section -->
    <section class="team-section">
      <h2>Meet the Team</h2>
      <div class="team-grid">
        {#each teamMembers as member}
          <div class="team-member">
            <div class="member-image">
              <img src={member.image} alt={member.name} />
            </div>
            <div class="member-info">
              <h3>{member.name}</h3>
              <h4>{member.role}</h4>
              <p>{member.bio}</p>
            </div>
          </div>
        {/each}
      </div>
    </section>

    <!-- Values Section -->
    <section class="values-section">
      <div class="values-content">
        <h2>What We Stand For</h2>
        <div class="values-grid">
          <div class="value-item">
            <h3>Authenticity Over Tourism</h3>
            <p>We believe in showing you the real Berlin, not a sanitized tourist version. Our experiences are designed by locals, for music lovers.</p>
          </div>
          
          <div class="value-item">
            <h3>Respect for the Scene</h3>
            <p>We educate our guests about club etiquette and cultural norms to ensure everyone can enjoy the scene respectfully.</p>
          </div>
          
          <div class="value-item">
            <h3>Community First</h3>
            <p>We're not just a business - we're part of the community. We reinvest in local artists, venues, and cultural initiatives.</p>
          </div>
          
          <div class="value-item">
            <h3>Sustainable Nightlife</h3>
            <p>We promote responsible partying and work with venues that prioritize sustainability and social responsibility.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Partners Section -->
    <section class="partners-section">
      <h2>Our Partners</h2>
      <p class="partners-intro">We work with Berlin's most respected venues, promoters, and artists to bring you authentic experiences.</p>
      
      <div class="partners-categories">
        <div class="partner-category">
          <h3>Legendary Venues</h3>
          <div class="partner-logos">
            <div class="partner-logo">Berghain/Panorama Bar</div>
            <div class="partner-logo">Watergate</div>
            <div class="partner-logo">Tresor</div>
            <div class="partner-logo">://about blank</div>
          </div>
        </div>
        
        <div class="partner-category">
          <h3>Underground Spaces</h3>
          <div class="partner-logos">
            <div class="partner-logo">OHM</div>
            <div class="partner-logo">Griessmuehle</div>
            <div class="partner-logo">Sisyphos</div>
            <div class="partner-logo">Kater Blau</div>
          </div>
        </div>
        
        <div class="partner-category">
          <h3>Cultural Partners</h3>
          <div class="partner-logos">
            <div class="partner-logo">Berlin Music Commission</div>
            <div class="partner-logo">Club Commission</div>
            <div class="partner-logo">Berlin Tourismus</div>
            <div class="partner-logo">Musicboard Berlin</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Contact CTA -->
    <section class="contact-cta">
      <div class="cta-content">
        <h2>Ready to Explore Berlin's Electronic Scene?</h2>
        <p>Join thousands of music lovers who have discovered authentic Berlin through our curated experiences.</p>
        <div class="cta-buttons">
          <a href="/events" class="btn-primary">Browse Events</a>
          <a href="/packages" class="btn-secondary">View Packages</a>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .about-page {
    min-height: 100vh;
  }

  .hero {
    height: 60vh;
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #667eea 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    text-align: center;
  }

  .hero-content h1 {
    font-size: 3.5rem;
    margin-bottom: 1rem;
    font-weight: 700;
  }

  .hero-content p {
    font-size: 1.3rem;
    opacity: 0.9;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
  }

  /* Story Section */
  .story-section {
    padding: 4rem 0;
  }

  .story-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4rem;
    align-items: center;
  }

  .story-text h2 {
    font-size: 2.5rem;
    margin-bottom: 2rem;
    color: #1a202c;
  }

  .story-text p {
    color: #4a5568;
    line-height: 1.7;
    margin-bottom: 1.5rem;
    font-size: 1.1rem;
  }

  .story-image img {
    width: 100%;
    height: 400px;
    object-fit: cover;
    border-radius: 16px;
  }

  /* Mission Section */
  .mission-section {
    background: #f7fafc;
    padding: 4rem 2rem;
    border-radius: 16px;
    margin: 4rem 0;
  }

  .mission-content h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: #1a202c;
  }

  .mission-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
  }

  .mission-item {
    text-align: center;
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .mission-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .mission-item h3 {
    font-size: 1.25rem;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .mission-item p {
    color: #4a5568;
    line-height: 1.6;
  }

  /* Stats Section */
  .stats-section {
    padding: 4rem 0;
    text-align: center;
  }

  .stats-section h2 {
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: #1a202c;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
  }

  .stat-item {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 3rem 2rem;
    border-radius: 16px;
    text-align: center;
  }

  .stat-number {
    font-size: 3rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
  }

  .stat-label {
    font-size: 1.1rem;
    opacity: 0.9;
  }

  /* Team Section */
  .team-section {
    padding: 4rem 0;
  }

  .team-section h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: #1a202c;
  }

  .team-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 3rem;
  }

  .team-member {
    text-align: center;
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
  }

  .team-member:hover {
    transform: translateY(-5px);
  }

  .member-image {
    margin-bottom: 1.5rem;
  }

  .member-image img {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    object-fit: cover;
    border: 4px solid #667eea;
  }

  .member-info h3 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    color: #1a202c;
  }

  .member-info h4 {
    font-size: 1rem;
    color: #667eea;
    margin-bottom: 1rem;
    font-weight: 600;
  }

  .member-info p {
    color: #4a5568;
    line-height: 1.6;
  }

  /* Values Section */
  .values-section {
    background: #1a202c;
    color: white;
    padding: 4rem 2rem;
    border-radius: 16px;
    margin: 4rem 0;
  }

  .values-content h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
  }

  .values-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 2rem;
  }

  .value-item {
    padding: 1.5rem;
  }

  .value-item h3 {
    font-size: 1.25rem;
    margin-bottom: 1rem;
    color: #667eea;
  }

  .value-item p {
    line-height: 1.6;
    opacity: 0.9;
  }

  /* Partners Section */
  .partners-section {
    padding: 4rem 0;
    text-align: center;
  }

  .partners-section h2 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
    color: #1a202c;
  }

  .partners-intro {
    font-size: 1.1rem;
    color: #4a5568;
    margin-bottom: 3rem;
  }

  .partners-categories {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 3rem;
  }

  .partner-category h3 {
    font-size: 1.25rem;
    margin-bottom: 1.5rem;
    color: #1a202c;
  }

  .partner-logos {
    display: grid;
    gap: 1rem;
  }

  .partner-logo {
    background: #f7fafc;
    padding: 1rem;
    border-radius: 8px;
    color: #4a5568;
    font-weight: 500;
  }

  /* Contact CTA */
  .contact-cta {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4rem 2rem;
    border-radius: 16px;
    text-align: center;
    margin: 4rem 0;
  }

  .cta-content h2 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .cta-content p {
    font-size: 1.2rem;
    opacity: 0.9;
    margin-bottom: 2rem;
  }

  .cta-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .btn-primary, .btn-secondary {
    padding: 1rem 2rem;
    border-radius: 12px;
    font-weight: 600;
    text-decoration: none;
    transition: all 0.3s;
    border: 2px solid transparent;
  }

  .btn-primary {
    background: white;
    color: #667eea;
  }

  .btn-primary:hover {
    background: #f7fafc;
  }

  .btn-secondary {
    background: transparent;
    color: white;
    border-color: white;
  }

  .btn-secondary:hover {
    background: white;
    color: #667eea;
  }

  @media (max-width: 768px) {
    .hero-content h1 {
      font-size: 2.5rem;
    }

    .story-grid {
      grid-template-columns: 1fr;
    }

    .mission-grid {
      grid-template-columns: 1fr;
    }

    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .team-grid {
      grid-template-columns: 1fr;
    }

    .values-grid {
      grid-template-columns: 1fr;
    }

    .partners-categories {
      grid-template-columns: 1fr;
    }

    .cta-buttons {
      flex-direction: column;
      align-items: center;
    }

    .btn-primary, .btn-secondary {
      width: 200px;
    }
  }
</style>