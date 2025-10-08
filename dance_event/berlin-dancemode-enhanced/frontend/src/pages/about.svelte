<svelte:head>
  <title>About - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="hero-section">
      <h1>About Berlin DanceMode</h1>
      <p class="hero-subtitle">Where Berlin's nightlife comes alive</p>
    </header>

    <section class="story-section">
      <div class="story-content">
        <h2>Our Story</h2>
        <p>
          Berlin DanceMode was born from a passion for Berlin's incredible dance and electronic music scene. 
          Since 2020, we've been connecting music lovers with the most authentic and exciting experiences 
          the city has to offer.
        </p>
        <p>
          From intimate underground venues to world-renowned clubs, we curate experiences that capture 
          the true spirit of Berlin's nightlife culture. Our platform brings together event organizers, 
          venues, and music enthusiasts in one vibrant community.
        </p>
      </div>
      <div class="story-visual">
        <div class="visual-card">
          <div class="card-icon">🎵</div>
          <h3>Music First</h3>
          <p>Curated events featuring the best electronic music artists</p>
        </div>
        <div class="visual-card">
          <div class="card-icon">🌃</div>
          <h3>Berlin Culture</h3>
          <p>Authentic experiences in the heart of the city's nightlife</p>
        </div>
        <div class="visual-card">
          <div class="card-icon">👥</div>
          <h3>Community</h3>
          <p>Connecting music lovers and creators across Berlin</p>
        </div>
      </div>
    </section>

    <section class="mission-section">
      <div class="mission-content">
        <h2>Our Mission</h2>
        <div class="mission-grid">
          <div class="mission-item">
            <div class="mission-icon">🎯</div>
            <h3>Connect</h3>
            <p>Bringing together the Berlin dance community through unforgettable events and experiences</p>
          </div>
          <div class="mission-item">
            <div class="mission-icon">⚡</div>
            <h3>Innovate</h3>
            <p>Using technology to enhance how people discover and experience Berlin's nightlife</p>
          </div>
          <div class="mission-item">
            <div class="mission-icon">🌟</div>
            <h3>Inspire</h3>
            <p>Supporting local artists, venues, and creators in the electronic music ecosystem</p>
          </div>
        </div>
      </div>
    </section>

    <section class="team-section">
      <h2>Meet the Team</h2>
      <div class="team-grid">
        <div class="team-member">
          <div class="member-avatar">👨‍💻</div>
          <h3>Alex Mueller</h3>
          <p class="member-role">Founder & CEO</p>
          <p class="member-bio">Electronic music enthusiast with 10+ years in Berlin's club scene</p>
        </div>
        <div class="team-member">
          <div class="member-avatar">👩‍🎨</div>
          <h3>Sarah Chen</h3>
          <p class="member-role">Creative Director</p>
          <p class="member-bio">Visual artist and event curator specializing in immersive experiences</p>
        </div>
        <div class="team-member">
          <div class="member-avatar">👨‍🎵</div>
          <h3>Marcus König</h3>
          <p class="member-role">Music Director</p>
          <p class="member-bio">DJ and producer with deep connections in Berlin's underground scene</p>
        </div>
      </div>
    </section>

    <section class="stats-section">
      <h2>By the Numbers</h2>
      <div class="stats-grid">
        <div class="stat-item">
          <div class="stat-number">10,000+</div>
          <div class="stat-label">Events Hosted</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">50+</div>
          <div class="stat-label">Partner Venues</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">100,000+</div>
          <div class="stat-label">Community Members</div>
        </div>
        <div class="stat-item">
          <div class="stat-number">24/7</div>
          <div class="stat-label">Berlin Vibes</div>
        </div>
      </div>
    </section>

    <section class="contact-section">
      <div class="contact-content">
        <h2>Get in Touch</h2>
        <p>Have questions or want to collaborate? We'd love to hear from you!</p>
        <div class="contact-info">
          <div class="contact-item">
            <div class="contact-icon">📧</div>
            <div>
              <h4>Email</h4>
              <p>hello@berlindancemode.com</p>
            </div>
          </div>
          <div class="contact-item">
            <div class="contact-icon">📍</div>
            <div>
              <h4>Location</h4>
              <p>Kreuzberg, Berlin, Germany</p>
            </div>
          </div>
          <div class="contact-item">
            <div class="contact-icon">📱</div>
            <div>
              <h4>Social</h4>
              <p>@berlindancemode</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</main>

<style>
  main {
    min-height: 100vh;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
  }

  .hero-section {
    text-align: center;
    padding: 4rem 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    margin: 0 -2rem 4rem -2rem;
    color: white;
  }

  .hero-section h1 {
    font-size: 3.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
  }

  .hero-subtitle {
    font-size: 1.5rem;
    opacity: 0.9;
    margin: 0;
  }

  .story-section {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4rem;
    margin-bottom: 6rem;
    align-items: center;
  }

  .story-content h2 {
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 2rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .story-content p {
    color: #718096;
    line-height: 1.8;
    font-size: 1.1rem;
    margin-bottom: 1.5rem;
  }

  .story-visual {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .visual-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: transform 0.2s ease;
  }

  .visual-card:hover {
    transform: translateY(-4px);
  }

  .card-icon {
    font-size: 2rem;
    margin-bottom: 1rem;
  }

  .visual-card h3 {
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .visual-card p {
    color: #718096;
    margin: 0;
  }

  .mission-section {
    margin-bottom: 6rem;
  }

  .mission-content h2 {
    text-align: center;
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 3rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .mission-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .mission-item {
    background: white;
    padding: 2.5rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    text-align: center;
    transition: transform 0.2s ease;
  }

  .mission-item:hover {
    transform: translateY(-4px);
  }

  .mission-icon {
    font-size: 3rem;
    margin-bottom: 1.5rem;
  }

  .mission-item h3 {
    color: #2d3748;
    font-size: 1.5rem;
    margin-bottom: 1rem;
  }

  .mission-item p {
    color: #718096;
    line-height: 1.6;
    margin: 0;
  }

  .team-section {
    margin-bottom: 6rem;
  }

  .team-section h2 {
    text-align: center;
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 3rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .team-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .team-member {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    text-align: center;
    transition: transform 0.2s ease;
  }

  .team-member:hover {
    transform: translateY(-4px);
  }

  .member-avatar {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .team-member h3 {
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .member-role {
    color: #667eea;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .member-bio {
    color: #718096;
    line-height: 1.6;
    margin: 0;
  }

  .stats-section {
    margin-bottom: 6rem;
    padding: 4rem 0;
    background: white;
    border-radius: 20px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }

  .stats-section h2 {
    text-align: center;
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 3rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
  }

  .stat-item {
    text-align: center;
  }

  .stat-number {
    font-size: 3rem;
    font-weight: bold;
    color: #667eea;
    margin-bottom: 0.5rem;
  }

  .stat-label {
    color: #718096;
    font-weight: 600;
  }

  .contact-section {
    margin-bottom: 4rem;
  }

  .contact-content h2 {
    text-align: center;
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .contact-content > p {
    text-align: center;
    color: #718096;
    font-size: 1.2rem;
    margin-bottom: 3rem;
  }

  .contact-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
  }

  .contact-item {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .contact-icon {
    font-size: 2rem;
  }

  .contact-item h4 {
    color: #2d3748;
    margin: 0 0 0.5rem 0;
  }

  .contact-item p {
    color: #718096;
    margin: 0;
  }

  @media (max-width: 768px) {
    .hero-section h1 {
      font-size: 2.5rem;
    }

    .hero-subtitle {
      font-size: 1.2rem;
    }

    .story-section {
      grid-template-columns: 1fr;
      gap: 2rem;
    }

    .container {
      padding: 0 1rem;
    }

    .hero-section {
      margin: 0 -1rem 2rem -1rem;
      padding: 2rem 0;
    }

    .story-content h2,
    .mission-content h2,
    .team-section h2,
    .stats-section h2,
    .contact-content h2 {
      font-size: 2rem;
    }
  }
</style>