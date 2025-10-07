<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores.js';
  import api from '$lib/api.js';

  let userBookings = [];
  let userFavorites = [];
  let loading = true;
  let error = null;

  onMount(async () => {
    // Check if user is authenticated
    if (!$authStore.isAuthenticated) {
      goto('/login');
      return;
    }

    try {
      // Simulate loading user data - in a real app, these would be API calls
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Mock data for now
      userBookings = [
        {
          id: 'booking_1',
          event: 'Techno Night at Berghain',
          date: '2025-10-15',
          package: 'VIP Experience',
          status: 'confirmed'
        },
        {
          id: 'booking_2',
          event: 'House Music Festival',
          date: '2025-10-20',
          package: 'Premium Package',
          status: 'pending'
        }
      ];

      userFavorites = [
        { id: 'event_1', name: 'Warehouse Rave', genre: 'Techno' },
        { id: 'event_2', name: 'Deep House Sessions', genre: 'House' },
        { id: 'event_3', name: 'Minimal Monday', genre: 'Minimal' }
      ];

      loading = false;
    } catch (err) {
      console.error('Error loading dashboard data:', err);
      error = 'Failed to load dashboard data';
      loading = false;
    }
  });

  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  }

  function getStatusColor(status) {
    switch (status) {
      case 'confirmed': return '#10b981';
      case 'pending': return '#f59e0b';
      case 'cancelled': return '#ef4444';
      default: return '#6b7280';
    }
  }
</script>

<svelte:head>
  <title>Dashboard - Berlin DanceMode</title>
  <meta name="description" content="Your personal dashboard for Berlin DanceMode events and bookings" />
</svelte:head>

<div class="dashboard-page">
  <div class="hero">
    <div class="hero-content">
      <h1>Welcome back, {$authStore.user?.username || 'User'}!</h1>
      <p>Your personal dance event command center</p>
    </div>
  </div>

  <div class="container">
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading your dashboard...</p>
      </div>
    {:else if error}
      <div class="error">
        <p>{error}</p>
      </div>
    {:else}
      <div class="dashboard-grid">
        <!-- Quick Stats -->
        <section class="stats-section">
          <h2>Quick Stats</h2>
          <div class="stats-grid">
            <div class="stat-card">
              <div class="stat-icon">🎫</div>
              <div class="stat-content">
                <h3>{userBookings.length}</h3>
                <p>Active Bookings</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">❤️</div>
              <div class="stat-content">
                <h3>{userFavorites.length}</h3>
                <p>Favorite Events</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">🎵</div>
              <div class="stat-content">
                <h3>12</h3>
                <p>Events Attended</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">⭐</div>
              <div class="stat-content">
                <h3>VIP</h3>
                <p>Member Status</p>
              </div>
            </div>
          </div>
        </section>

        <!-- Recent Bookings -->
        <section class="bookings-section">
          <div class="section-header">
            <h2>Your Bookings</h2>
            <a href="/packages" class="view-all-btn">Book More Events</a>
          </div>
          
          {#if userBookings.length > 0}
            <div class="bookings-list">
              {#each userBookings as booking}
                <div class="booking-card">
                  <div class="booking-info">
                    <h3>{booking.event}</h3>
                    <p class="booking-date">{formatDate(booking.date)}</p>
                    <p class="booking-package">{booking.package}</p>
                  </div>
                  <div class="booking-status">
                    <span 
                      class="status-badge" 
                      style="background-color: {getStatusColor(booking.status)};"
                    >
                      {booking.status.toUpperCase()}
                    </span>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="empty-state">
              <p>No bookings yet. Ready to join the party?</p>
              <a href="/packages" class="btn">Explore Packages</a>
            </div>
          {/if}
        </section>

        <!-- Favorite Events -->
        <section class="favorites-section">
          <div class="section-header">
            <h2>Your Favorites</h2>
            <a href="/events" class="view-all-btn">Browse Events</a>
          </div>
          
          {#if userFavorites.length > 0}
            <div class="favorites-list">
              {#each userFavorites as favorite}
                <div class="favorite-card">
                  <div class="favorite-icon">🎵</div>
                  <div class="favorite-info">
                    <h3>{favorite.name}</h3>
                    <p>{favorite.genre}</p>
                  </div>
                  <button class="remove-favorite">×</button>
                </div>
              {/each}
            </div>
          {:else}
            <div class="empty-state">
              <p>No favorites yet. Discover amazing events!</p>
              <a href="/events" class="btn">Find Events</a>
            </div>
          {/if}
        </section>

        <!-- Quick Actions -->
        <section class="actions-section">
          <h2>Quick Actions</h2>
          <div class="actions-grid">
            <a href="/events" class="action-card">
              <div class="action-icon">🔍</div>
              <h3>Find Events</h3>
              <p>Discover new parties and events</p>
            </a>
            <a href="/packages" class="action-card">
              <div class="action-icon">🎫</div>
              <h3>Book Package</h3>
              <p>Get exclusive access deals</p>
            </a>
            <a href="/venues" class="action-card">
              <div class="action-icon">📍</div>
              <h3>Explore Venues</h3>
              <p>Find the hottest club spots</p>
            </a>
          </div>
        </section>
      </div>
    {/if}
  </div>
</div>

<style>
  .dashboard-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
  }

  .hero {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4rem 0 3rem;
    text-align: center;
  }

  .hero-content h1 {
    font-size: 2.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
  }

  .hero-content p {
    font-size: 1.2rem;
    opacity: 0.9;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .loading, .error {
    text-align: center;
    padding: 4rem 0;
  }

  .spinner {
    width: 50px;
    height: 50px;
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

  .dashboard-grid {
    display: grid;
    gap: 2rem;
    grid-template-columns: 1fr;
  }

  @media (min-width: 768px) {
    .dashboard-grid {
      grid-template-columns: 1fr 1fr;
    }
  }

  @media (min-width: 1024px) {
    .dashboard-grid {
      grid-template-columns: 2fr 1fr;
    }
  }

  section {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .section-header h2 {
    font-size: 1.5rem;
    font-weight: bold;
    color: #2d3748;
  }

  .view-all-btn {
    color: #667eea;
    text-decoration: none;
    font-weight: 500;
    font-size: 0.9rem;
  }

  .view-all-btn:hover {
    text-decoration: underline;
  }

  /* Stats Section */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-radius: 12px;
  }

  .stat-icon {
    font-size: 2rem;
  }

  .stat-content h3 {
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
  }

  .stat-content p {
    margin: 0;
    opacity: 0.9;
    font-size: 0.9rem;
  }

  /* Bookings Section */
  .bookings-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .booking-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: #f7fafc;
    border-radius: 8px;
    border-left: 4px solid #667eea;
  }

  .booking-info h3 {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: #2d3748;
  }

  .booking-date {
    font-size: 0.9rem;
    color: #4a5568;
    margin: 0 0 0.25rem 0;
  }

  .booking-package {
    font-size: 0.8rem;
    color: #667eea;
    font-weight: 500;
    margin: 0;
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    color: white;
    font-size: 0.75rem;
    font-weight: bold;
  }

  /* Favorites Section */
  .favorites-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .favorite-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background: #f7fafc;
    border-radius: 8px;
  }

  .favorite-icon {
    font-size: 1.5rem;
  }

  .favorite-info {
    flex: 1;
  }

  .favorite-info h3 {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
    color: #2d3748;
  }

  .favorite-info p {
    font-size: 0.8rem;
    color: #667eea;
    margin: 0.25rem 0 0 0;
  }

  .remove-favorite {
    background: none;
    border: none;
    color: #a0aec0;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }

  .remove-favorite:hover {
    background: #fed7e2;
    color: #e53e3e;
  }

  /* Actions Section */
  .actions-grid {
    display: grid;
    gap: 1rem;
  }

  .action-card {
    display: block;
    padding: 1.5rem;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border-radius: 12px;
    text-decoration: none;
    color: #2d3748;
    transition: all 0.3s ease;
    border: 2px solid transparent;
  }

  .action-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(102, 126, 234, 0.15);
    border-color: #667eea;
  }

  .action-icon {
    font-size: 2rem;
    margin-bottom: 1rem;
  }

  .action-card h3 {
    font-size: 1.1rem;
    font-weight: bold;
    margin: 0 0 0.5rem 0;
  }

  .action-card p {
    font-size: 0.9rem;
    color: #4a5568;
    margin: 0;
  }

  /* Empty State */
  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #4a5568;
  }

  .empty-state p {
    margin-bottom: 1rem;
  }

  .btn {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    text-decoration: none;
    border-radius: 8px;
    font-weight: 500;
    transition: all 0.3s ease;
  }

  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
  }
</style>