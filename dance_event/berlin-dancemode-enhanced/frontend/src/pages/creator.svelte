<script>
  import { onMount } from 'svelte'
  import { url } from '@roxi/routify'
  import { authStore } from '../stores/auth.js'
  import { goto } from '@roxi/routify'
  
  // Redirect if not a creator
  onMount(() => {
    if (!$authStore.isAuthenticated || !$authStore.isCreator) {
      $goto('/login')
    }
  })
</script>

<svelte:head>
  <title>Creator Area - Berlin Dance Mode</title>
</svelte:head>

<div class="creator-dashboard">
  <header class="creator-header">
    <h1>Creator Area</h1>
    <p>Welcome, {$authStore.firstName || $authStore.username}! Manage your content and view analytics.</p>
  </header>

  <div class="creator-grid">
    <div class="creator-card">
      <div class="card-icon">📊</div>
      <h3>Analytics</h3>
      <p>View performance metrics for your events and content</p>
      <div class="stats">
        <div class="stat">
          <span class="stat-number">24</span>
          <span class="stat-label">Events Created</span>
        </div>
        <div class="stat">
          <span class="stat-number">1.2k</span>
          <span class="stat-label">Total Attendees</span>
        </div>
      </div>
    </div>

    <div class="creator-card">
      <div class="card-icon">🎵</div>
      <h3>My Events</h3>
      <p>Create and manage your dance events</p>
      <a href={$url('/events')} class="btn-primary">Manage Events</a>
    </div>

    <div class="creator-card">
      <div class="card-icon">🏛️</div>
      <h3>Venues</h3>
      <p>Partner venues and booking information</p>
      <a href={$url('/venues')} class="btn-primary">View Venues</a>
    </div>

    <div class="creator-card">
      <div class="card-icon">📦</div>
      <h3>Packages</h3>
      <p>Create special event packages and offers</p>
      <a href={$url('/packages')} class="btn-primary">Manage Packages</a>
    </div>

    <div class="creator-card wide">
      <div class="card-icon">🎨</div>
      <h3>Event Creation Tools</h3>
      <p>Quick access to create new events and content</p>
      <div class="action-buttons">
        <button class="btn-secondary">New Event</button>
        <button class="btn-secondary">Upload Media</button>
        <button class="btn-secondary">Create Package</button>
      </div>
    </div>
  </div>
</div>

<style>
  .creator-dashboard {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .creator-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .creator-header h1 {
    color: #ff6b35;
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .creator-header p {
    font-size: 1.2rem;
    color: #666;
  }

  .creator-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .creator-card {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 15px;
    padding: 2rem;
    color: white;
    text-align: center;
    box-shadow: 0 10px 30px rgba(0,0,0,0.2);
    transition: transform 0.3s ease;
  }

  .creator-card:hover {
    transform: translateY(-5px);
  }

  .creator-card.wide {
    grid-column: span 2;
  }

  @media (max-width: 768px) {
    .creator-card.wide {
      grid-column: span 1;
    }
  }

  .card-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .creator-card h3 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
  }

  .creator-card p {
    margin-bottom: 1.5rem;
    opacity: 0.9;
  }

  .stats {
    display: flex;
    justify-content: space-around;
    margin-top: 1rem;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-number {
    font-size: 2rem;
    font-weight: bold;
    color: #fff;
  }

  .stat-label {
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .action-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .btn-primary, .btn-secondary {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 25px;
    font-weight: 600;
    text-decoration: none;
    display: inline-block;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .btn-primary {
    background: #ff6b35;
    color: white;
  }

  .btn-primary:hover {
    background: #e55a2b;
    transform: translateY(-2px);
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
</style>