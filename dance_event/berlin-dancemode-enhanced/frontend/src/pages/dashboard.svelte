<script>
  import { onMount } from 'svelte'
  import { authStore } from '../stores/auth.js'
  import api from '../services/api.js'
  import { url } from '@roxi/routify'

  let userRegistrations = []
  let loading = true
  let userStats = {
    totalEvents: 0,
    upcomingEvents: 0,
    totalSpent: 0
  }

  onMount(async () => {
    if (!$authStore.isAuthenticated) {
      // Redirect to login if not authenticated
      window.location.href = '/login'
      return
    }

    try {
      // In a real app, we'd fetch user-specific data
      // For now, we'll show some mock data
      userRegistrations = [
        {
          id: '1', 
          event_name: 'Techno Night at Berghain',
          date: '2025-10-15T22:00:00Z',
          status: 'confirmed',
          amount: 25
        },
        {
          id: '2',
          event_name: 'Underground House Session',
          date: '2025-10-20T23:00:00Z', 
          status: 'pending',
          amount: 20
        }
      ]
      
      userStats = {
        totalEvents: userRegistrations.length,
        upcomingEvents: userRegistrations.filter(r => new Date(r.date) > new Date()).length,
        totalSpent: userRegistrations.reduce((sum, r) => sum + r.amount, 0)
      }
    } catch (error) {
      console.error('Error loading dashboard data:', error)
    } finally {
      loading = false
    }
  })

  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString('en-US', {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  function getStatusColor(status) {
    switch (status) {
      case 'confirmed': return '#10b981'
      case 'pending': return '#f59e0b'
      case 'cancelled': return '#ef4444'
      default: return '#6b7280'
    }
  }
</script>

<svelte:head>
  <title>Dashboard - Berlin DanceMode</title>
</svelte:head>

<main>
  <div class="container">
    <header class="page-header">
      <h1>Welcome back, {$authStore.user?.username || 'User'}!</h1>
      <p>Your personal dance event dashboard</p>
    </header>

    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading your dashboard...</p>
      </div>
    {:else}
      <!-- Stats Cards -->
      <section class="stats-section">
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon">🎫</div>
            <div class="stat-content">
              <h3>{userStats.totalEvents}</h3>
              <p>Total Events</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">📅</div>
            <div class="stat-content">
              <h3>{userStats.upcomingEvents}</h3>
              <p>Upcoming Events</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">💰</div>
            <div class="stat-content">
              <h3>€{userStats.totalSpent}</h3>
              <p>Total Spent</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">⭐</div>
            <div class="stat-content">
              <h3>{$authStore.isCreator ? 'Creator' : 'Member'}</h3>
              <p>Account Type</p>
            </div>
          </div>
        </div>
      </section>

      <!-- Quick Actions -->
      <section class="actions-section">
        <h2>Quick Actions</h2>
        <div class="actions-grid">
          <a href={$url('/events')} class="action-card">
            <div class="action-icon">🎪</div>
            <h3>Browse Events</h3>
            <p>Discover new dance events in Berlin</p>
          </a>
          <a href={$url('/packages')} class="action-card">
            <div class="action-icon">📦</div>
            <h3>View Packages</h3>
            <p>Check out exclusive event packages</p>
          </a>
          <a href={$url('/venues')} class="action-card">
            <div class="action-icon">🏢</div>
            <h3>Explore Venues</h3>
            <p>Find the best dance venues in the city</p>
          </a>
          {#if $authStore.isCreator}
            <a href={$url('/creator')} class="action-card creator-card">
              <div class="action-icon">✨</div>
              <h3>Creator Area</h3>
              <p>Manage your events and content</p>
            </a>
          {/if}
        </div>
      </section>

      <!-- Recent Registrations -->
      <section class="registrations-section">
        <h2>Your Event Registrations</h2>
        {#if userRegistrations.length > 0}
          <div class="registrations-list">
            {#each userRegistrations as registration}
              <div class="registration-card">
                <div class="registration-info">
                  <h3>{registration.event_name}</h3>
                  <p class="registration-date">{formatDate(registration.date)}</p>
                  <div class="registration-meta">
                    <span class="amount">€{registration.amount}</span>
                    <span 
                      class="status" 
                      style="color: {getStatusColor(registration.status)}"
                    >
                      {registration.status.charAt(0).toUpperCase() + registration.status.slice(1)}
                    </span>
                  </div>
                </div>
                <div class="registration-actions">
                  <button class="btn-secondary">View Details</button>
                  {#if registration.status === 'confirmed'}
                    <button class="btn-primary">Get Ticket</button>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="empty-state">
            <div class="empty-icon">🎭</div>
            <h3>No Events Yet</h3>
            <p>You haven't registered for any events yet. Start exploring!</p>
            <a href={$url('/events')} class="btn-primary">Browse Events</a>
          </div>
        {/if}
      </section>

      <!-- Account Settings -->
      <section class="settings-section">
        <h2>Account Settings</h2>
        <div class="settings-card">
          <div class="settings-info">
            <h3>Profile Information</h3>
            <p>Manage your account details and preferences</p>
          </div>
          <div class="settings-actions">
            <button class="btn-outline">Edit Profile</button>
            <button class="btn-outline">Preferences</button>
          </div>
        </div>
      </section>
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
    font-size: 2.5rem;
    color: #2d3748;
    margin-bottom: 0.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .page-header p {
    font-size: 1.1rem;
    color: #718096;
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

  .stats-section {
    margin-bottom: 3rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .stat-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    gap: 1rem;
    transition: transform 0.2s ease;
  }

  .stat-card:hover {
    transform: translateY(-2px);
  }

  .stat-icon {
    font-size: 2.5rem;
  }

  .stat-content h3 {
    font-size: 1.8rem;
    font-weight: bold;
    color: #2d3748;
    margin: 0 0 0.25rem 0;
  }

  .stat-content p {
    color: #718096;
    margin: 0;
    font-weight: 500;
  }

  .actions-section, .registrations-section, .settings-section {
    margin-bottom: 3rem;
  }

  .actions-section h2, .registrations-section h2, .settings-section h2 {
    font-size: 1.8rem;
    color: #2d3748;
    margin-bottom: 1.5rem;
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .action-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    text-decoration: none;
    color: inherit;
    transition: all 0.2s ease;
    text-align: center;
  }

  .action-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
  }

  .creator-card {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .action-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .action-card h3 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
  }

  .action-card p {
    color: inherit;
    opacity: 0.8;
    margin: 0;
  }

  .registrations-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .registration-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: transform 0.2s ease;
  }

  .registration-card:hover {
    transform: translateY(-2px);
  }

  .registration-info h3 {
    font-size: 1.1rem;
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .registration-date {
    color: #667eea;
    font-weight: 500;
    margin-bottom: 0.5rem;
  }

  .registration-meta {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .amount {
    font-weight: bold;
    color: #2d3748;
  }

  .status {
    font-weight: 600;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .registration-actions {
    display: flex;
    gap: 0.5rem;
  }

  .settings-card {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .settings-info h3 {
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .settings-info p {
    color: #718096;
    margin: 0;
  }

  .settings-actions {
    display: flex;
    gap: 1rem;
  }

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
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

  .empty-state p {
    margin-bottom: 2rem;
  }

  .btn-primary, .btn-secondary, .btn-outline {
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    text-decoration: none;
    display: inline-block;
    font-size: 0.9rem;
    border: none;
  }

  .btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .btn-primary:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  }

  .btn-secondary {
    background: #f7fafc;
    color: #4a5568;
    border: 1px solid #e2e8f0;
  }

  .btn-secondary:hover {
    background: #edf2f7;
    transform: translateY(-1px);
  }

  .btn-outline {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .btn-outline:hover {
    background: #667eea;
    color: white;
    transform: translateY(-1px);
  }

  @media (max-width: 768px) {
    main {
      padding: 1rem;
    }

    .page-header h1 {
      font-size: 2rem;
    }

    .stats-grid {
      grid-template-columns: 1fr;
    }

    .registration-card {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .registration-actions {
      width: 100%;
      justify-content: flex-end;
    }

    .settings-card {
      flex-direction: column;
      align-items: flex-start;
      gap: 1.5rem;
    }

    .settings-actions {
      width: 100%;
      justify-content: flex-start;
    }
  }
</style>