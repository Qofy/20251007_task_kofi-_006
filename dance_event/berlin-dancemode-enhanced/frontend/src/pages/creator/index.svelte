<script>
  import { onMount } from 'svelte'
  import { authStore } from '../../stores/auth.js'
  import api from '../../services/api.js'
  import { url, goto } from '@roxi/routify'

  let stats = {
    events: 0,
    totalRevenue: 0,
    totalAttendees: 0,
    upcomingEvents: 0
  }
  let recentEvents = []
  let loading = true

  onMount(async () => {
    if (!$authStore.isCreator) {
      $goto('/')
      return
    }

    try {
      const [analyticsData, eventsData] = await Promise.all([
        api.getCreatorAnalytics(),
        api.getCreatorEvents({ limit: 5 })
      ])
      
      stats = analyticsData.data || stats
      recentEvents = eventsData.data || []
    } catch (error) {
      console.error('Error loading creator dashboard:', error)
    } finally {
      loading = false
    }
  })
</script>

<main class="creator-dashboard">
  <div class="dashboard-header">
    <h1>Creator Dashboard</h1>
    <p>Manage your events, track performance, and grow your audience</p>
  </div>

  {#if loading}
    <div class="loading">Loading dashboard...</div>
  {:else}
    <!-- Stats Overview -->
    <section class="stats-section">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon events-icon">📅</div>
          <div class="stat-content">
            <h3>{stats.events}</h3>
            <p>Total Events</p>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon revenue-icon">💰</div>
          <div class="stat-content">
            <h3>€{stats.totalRevenue.toLocaleString()}</h3>
            <p>Total Revenue</p>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon attendees-icon">👥</div>
          <div class="stat-content">
            <h3>{stats.totalAttendees}</h3>
            <p>Total Attendees</p>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon upcoming-icon">🎯</div>
          <div class="stat-content">
            <h3>{stats.upcomingEvents}</h3>
            <p>Upcoming Events</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Quick Actions -->
    <section class="actions-section">
      <h2>Quick Actions</h2>
      <div class="actions-grid">
        <a href={$url('/creator/events/create')} class="action-card primary">
          <div class="action-icon">➕</div>
          <h3>Create Event</h3>
          <p>Add a new dance event to your portfolio</p>
        </a>
        
        <a href={$url('/creator/events')} class="action-card">
          <div class="action-icon">📋</div>
          <h3>Manage Events</h3>
          <p>View and edit your existing events</p>
        </a>
        
        <a href={$url('/creator/analytics')} class="action-card">
          <div class="action-icon">📊</div>
          <h3>View Analytics</h3>
          <p>Track performance and audience insights</p>
        </a>
        
        <a href={$url('/creator/elements')} class="action-card">
          <div class="action-icon">🎨</div>
          <h3>Design Elements</h3>
          <p>Manage your custom frontend elements</p>
        </a>
      </div>
    </section>

    <!-- Recent Events -->
    <section class="recent-events-section">
      <div class="section-header">
        <h2>Recent Events</h2>
        <a href={$url('/creator/events')} class="view-all-link">View All</a>
      </div>
      
      {#if recentEvents.length > 0}
        <div class="events-list">
          {#each recentEvents as event}
            <div class="event-item">
              <div class="event-date">
                <span class="day">{new Date(event.date).getDate()}</span>
                <span class="month">{new Date(event.date).toLocaleDateString('en', { month: 'short' })}</span>
              </div>
              <div class="event-details">
                <h3>{event.name}</h3>
                <p class="venue">{event.venue}</p>
                <p class="description">{event.description.substring(0, 80)}...</p>
              </div>
              <div class="event-meta">
                <span class="price">€{event.price}</span>
                <span class="status status-{event.status?.toLowerCase() || 'draft'}">{event.status || 'Draft'}</span>
              </div>
              <div class="event-actions">
                <a href={$url('/creator/events/[id]', { id: event.id })} class="edit-btn">Edit</a>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="empty-state">
          <div class="empty-icon">📅</div>
          <h3>No events yet</h3>
          <p>Create your first event to get started</p>
          <a href={$url('/creator/events/create')} class="btn-primary">Create Event</a>
        </div>
      {/if}
    </section>
  {/if}
</main>

<style>
  .creator-dashboard {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .dashboard-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .dashboard-header h1 {
    font-size: 3rem;
    font-weight: bold;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 0.5rem;
  }

  .dashboard-header p {
    color: #718096;
    font-size: 1.1rem;
  }

  .loading {
    text-align: center;
    padding: 4rem;
    color: #718096;
    font-size: 1.1rem;
  }

  .stats-section {
    margin-bottom: 3rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .stat-card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
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
    width: 60px;
    height: 60px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .events-icon {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  .revenue-icon {
    background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
  }

  .attendees-icon {
    background: linear-gradient(135deg, #ed8936 0%, #dd6b20 100%);
  }

  .upcoming-icon {
    background: linear-gradient(135deg, #9f7aea 0%, #805ad5 100%);
  }

  .stat-content h3 {
    font-size: 2rem;
    font-weight: bold;
    color: #2d3748;
    margin-bottom: 0.25rem;
  }

  .stat-content p {
    color: #718096;
    font-size: 0.9rem;
  }

  .actions-section {
    margin-bottom: 3rem;
  }

  .actions-section h2 {
    font-size: 2rem;
    color: #2d3748;
    margin-bottom: 1.5rem;
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .action-card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    text-decoration: none;
    color: inherit;
    transition: all 0.2s ease;
    border: 2px solid transparent;
  }

  .action-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  }

  .action-card.primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .action-card.primary:hover {
    box-shadow: 0 8px 25px rgba(102, 126, 234, 0.3);
  }

  .action-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .action-card h3 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
  }

  .action-card p {
    opacity: 0.8;
    line-height: 1.5;
  }

  .recent-events-section h2 {
    font-size: 2rem;
    color: #2d3748;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .view-all-link {
    color: #667eea;
    text-decoration: none;
    font-weight: 600;
    transition: color 0.2s ease;
  }

  .view-all-link:hover {
    color: #764ba2;
  }

  .events-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .event-item {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    display: grid;
    grid-template-columns: auto 1fr auto auto;
    gap: 1.5rem;
    align-items: center;
    transition: transform 0.2s ease;
  }

  .event-item:hover {
    transform: translateY(-2px);
  }

  .event-date {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-radius: 8px;
    padding: 0.75rem;
    text-align: center;
    font-weight: bold;
    min-width: 60px;
  }

  .event-date .day {
    display: block;
    font-size: 1.25rem;
  }

  .event-date .month {
    display: block;
    font-size: 0.75rem;
    text-transform: uppercase;
  }

  .event-details h3 {
    font-size: 1.1rem;
    font-weight: bold;
    color: #2d3748;
    margin-bottom: 0.25rem;
  }

  .venue {
    color: #667eea;
    font-weight: 500;
    font-size: 0.9rem;
    margin-bottom: 0.25rem;
  }

  .description {
    color: #718096;
    font-size: 0.9rem;
    line-height: 1.4;
  }

  .event-meta {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-end;
  }

  .price {
    font-size: 1.1rem;
    font-weight: bold;
    color: #667eea;
  }

  .status {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-published {
    background: #c6f6d5;
    color: #22543d;
  }

  .status-draft {
    background: #fed7d7;
    color: #742a2a;
  }

  .status-scheduled {
    background: #bee3f8;
    color: #2a4365;
  }

  .edit-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .edit-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
  }

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: white;
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
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
    color: #718096;
    margin-bottom: 1.5rem;
  }

  .btn-primary {
    display: inline-block;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    text-decoration: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    transition: all 0.2s ease;
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  }

  @media (max-width: 768px) {
    .creator-dashboard {
      padding: 1rem;
    }

    .dashboard-header h1 {
      font-size: 2.5rem;
    }

    .stats-grid,
    .actions-grid {
      grid-template-columns: 1fr;
    }

    .event-item {
      grid-template-columns: 1fr;
      text-align: center;
    }

    .event-meta {
      align-items: center;
      flex-direction: row;
      justify-content: center;
    }
  }
</style>