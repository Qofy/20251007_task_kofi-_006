<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'
  import { url } from '@roxi/routify'
  import Cards from './slots/cards.svelte';

  let events = []
  let packages = []
  let venues = []
  let loading = true

  onMount(async () => {
    try {
      const [eventsData, packagesData, venuesData] = await Promise.all([
        api.events.getAll(),
        api.packages.getAll(),
        api.venues.getAll()
      ])
      
      events = eventsData.data || []
      packages = packagesData.data || []
      venues = venuesData.data || []
    } catch (error) {
      console.error('Error loading home data:', error)
    } finally {
      loading = false
    }
  })
</script>

<main>
  <!-- Hero Section -->
  <section class="hero">
    <div class="hero-content">
      <h1>
Welcome to
Berlin DanceMode Experience
Edition 2025
</h1>
      <p>
In our 14th year celebrating:
 We will focus again on
﻿Blues & Fusion dancing,
learning, community and self-care.

All this is happening in Berlin for
dancers and music lovers:
</p>
      
      {#if $authStore.isAuthenticated}
        <div class="hero-actions">
          <a href={$url('/dashboard')} class="btn-primary">Go to Dashboard</a>
          {#if $authStore.isCreator}
            <a href={$url('/creator')} class="btn-secondary">Creator Area</a>
          {/if}
        </div>
      {:else}
        <div class="hero-actions">
          <a href={$url('/register')} class="btn-primary">Join Now</a>
          <a href={$url('/login')} class="btn-secondary">Login</a>
        </div>
      {/if}
    </div>
    <div class="hero-visual">
      <div class="hero-gradient"></div>
    </div>
  </section>

  <!-- Featured Events Cards Section -->
  <section class="cards-section">
    <div class="container">
      <div class="cards-grid">
        <!-- Card 1: Blues & Fusion Alumni Training -->
        <Cards>
          <div slot="header">
            <h4>Berlin Blues & Fusion Alumni Training</h4>
          </div>
          <div slot="content">
            <p>Join our intensive training program designed for experienced dancers looking to deepen their blues and fusion skills.</p>
            <div class="event-details">
              <span class="date">📅 September 25 - October 1</span>
              <span class="location">📍 Berlin Dance Studio</span>
              <span class="level">⭐ Advanced Level</span>
            </div>
          </div>
          <div slot="footer">
            <a class="btn-outline" href="https://berlin-dancemode.com/events">Find Out More</a>
          </div>
        </Cards>

        <!-- Card 2: Fusion Dance Weekend -->
        <Cards>
          <div slot="header">
            <h4>Fusion Dance Weekend Intensive</h4>
          </div>
          <div slot="content">
            <p>A weekend dedicated to exploring connection, musicality, and creative expression through fusion dance.</p>
            <div class="event-details">
              <span class="date">📅 October 14 - 15</span>
              <span class="location">📍 Movement Space Berlin</span>
              <span class="level">⭐ All Levels Welcome</span>
            </div>
          </div>
          <div slot="footer">
            <a class="btn-outline" href="https://berlin-dancemode.com/events">Find Out More</a>
          </div>
        </Cards>

        <!-- Card 3: Community Social Dance -->
        <Cards>
          <div slot="header">
            <h4>Monthly Community Social</h4>
          </div>
          <div slot="content">
            <p>Connect with the Berlin dance community in a welcoming environment. Live DJ, refreshments, and great vibes!</p>
            <div class="event-details">
              <span class="date">📅 Every Last Saturday</span>
              <span class="location">📍 Community Center</span>
              <span class="level">⭐ Open to Everyone</span>
            </div>
          </div>
          <div slot="footer">
            <a class="btn-outline" href="https://berlin-dancemode.com/events">Find Out More</a>
          </div>
        </Cards>
      </div>
    </div>
  </section>
</main>

<style>
  main {
    min-height: 100vh;
  }

  h4 {
    color: white;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    font-size: 1.8rem;
    margin: 0;
    line-height: 1.3;
  }

  /* Cards Section Styles */
  .cards-section {
    padding: 4rem 2rem;
    min-height: 80vh;
    display: flex;
    align-items: center;
  }

  .cards-section h2 {
    text-align: center;
    font-size: 2.8rem;
    margin-bottom: 3rem;
    color: #2d3748;
    font-weight: 700;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .cards-grid {
    display: flex;
    justify-content: center;
    gap: 2rem;
    flex-wrap: wrap;
    max-width: 1400px;
    margin: 0 auto;
  }

  .event-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .event-details span {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.8);
    font-weight: 500;
  }

  .date {
    color: rgba(255, 255, 255, 0.95) !important;
  }

  .location {
    color: rgba(255, 255, 255, 0.85) !important;
  }

  .level {
    color: rgba(255, 255, 255, 0.9) !important;
    font-weight: 600 !important;
  }
  .hero {
    background: url('https://www.wearemci.com/uploads/media/1920x/04/5594-Experiences%20%26%20Events.jpg?v=1-0');
    color: white;
    padding: 4rem 2rem;
    display: flex;
    align-items: center;
    min-height: 60vh;
    position: relative;
    overflow: hidden;
  }

  .hero-content {
    max-width: 1200px;
    margin: 0 auto;
    z-index: 2;
  }

  .hero h1 {
    font-size: 3.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
    line-height: 1.2;
  }

  .hero p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
    opacity: 0.9;
    max-width: 600px;
  }

  .hero-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .hero-visual {
    position: absolute;
    top: 0;
    right: 0;
    width: 50%;
    height: 100%;
    background: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.1'%3E%3Ccircle cx='30' cy='30' r='30'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E") repeat;
  }

  .hero-gradient {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(45deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%);
  }

  .featured-section {
    padding: 4rem 2rem;
  }

  .packages-section {
    background: #f8fafc;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
  }

  .featured-section h2 {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: #2d3748;
  }

  .loading, .empty-state {
    text-align: center;
    padding: 2rem;
    color: #718096;
  }

  .events-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
  }

  .event-card {
    background: white;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    cursor: pointer;
  }

  .event-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
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
    background: white;
    border-radius: 8px;
    padding: 0.5rem;
    text-align: center;
    color: #2d3748;
    font-weight: bold;
    position: absolute;
    top: 1rem;
    right: 1rem;
  }

  .event-date .day {
    display: block;
    font-size: 1.5rem;
  }

  .event-date .month {
    display: block;
    font-size: 0.8rem;
    text-transform: uppercase;
  }

  .event-info {
    padding: 1.5rem;
  }

  .event-info h3 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: #2d3748;
  }

  .venue {
    color: #667eea;
    font-weight: 500;
    margin-bottom: 0.5rem;
  }

  .description {
    color: #718096;
    line-height: 1.5;
    margin-bottom: 1rem;
  }

  .event-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .price {
    font-size: 1.25rem;
    font-weight: bold;
    color: #667eea;
  }

  .time {
    color: #718096;
    font-size: 0.9rem;
  }

  .packages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .package-card {
    background: white;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease;
  }

  .package-card:hover {
    transform: translateY(-2px);
  }

  .package-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .package-header h3 {
    font-size: 1.25rem;
    font-weight: bold;
  }

  .package-price {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .package-content {
    padding: 1.5rem;
  }

  .package-features {
    list-style: none;
    padding: 0;
    margin: 1rem 0 0 0;
  }

  .package-features li {
    padding: 0.25rem 0;
    color: #4a5568;
  }

  .package-features li:before {
    content: "✓";
    color: #667eea;
    font-weight: bold;
    margin-right: 0.5rem;
  }

  .package-footer {
    padding: 0 1.5rem 1.5rem 1.5rem;
  }

  .venues-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
  }

  .venue-card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease;
  }

  .venue-card:hover {
    transform: translateY(-2px);
  }

  .venue-info h3 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: #2d3748;
  }

  .venue-address {
    color: #667eea;
    font-weight: 500;
    margin-bottom: 1rem;
  }

  .venue-description {
    color: #718096;
    line-height: 1.5;
    margin-bottom: 1rem;
  }

  .venue-meta {
    color: #4a5568;
    font-size: 0.9rem;
  }

  .capacity {
    font-weight: 500;
  }

  .section-footer {
    text-align: center;
    margin-top: 3rem;
  }

  .btn-primary, .btn-secondary, .btn-outline {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s ease;
    border: none;
    cursor: pointer;
    font-size: 1rem;
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
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 2px solid rgba(255, 255, 255, 0.3);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.3);
    transform: translateY(-2px);
  }

  .btn-outline {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .btn-outline:hover {
    background: #667eea;
    color: white;
    transform: translateY(-2px);
  }

  @media (max-width: 768px) {
    .hero h1 {
      font-size: 2.5rem;
    }

    .hero p {
      font-size: 1rem;
    }

    .hero-actions {
      justify-content: center;
    }

    .featured-section {
      padding: 2rem 1rem;
    }

    .featured-section h2 {
      font-size: 2rem;
    }

    .events-grid, .packages-grid, .venues-grid {
      grid-template-columns: 1fr;
    }

    /* Cards section responsive */
    .cards-section {
      padding: 2rem 1rem;
    }

    .cards-section h2 {
      font-size: 2.2rem;
      margin-bottom: 2rem;
    }

    .cards-grid {
      flex-direction: column;
      align-items: center;
      gap: 1.5rem;
    }

    h4 {
      font-size: 1.5rem;
    }
  }

  @media (max-width: 1200px) and (min-width: 769px) {
    .cards-grid {
      gap: 1.5rem;
    }
  }
</style>