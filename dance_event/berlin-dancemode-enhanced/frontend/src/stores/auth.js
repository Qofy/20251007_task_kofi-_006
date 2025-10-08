import { writable } from 'svelte/store'

// Auth store with localStorage persistence
function createAuthStore() {
  const { subscribe, set, update } = writable({
    isAuthenticated: false,
    user: null,
    token: null,
    isCreator: false
  })

  // Initialize from localStorage if available
  if (typeof localStorage !== 'undefined') {
    const stored = localStorage.getItem('auth')
    if (stored) {
      try {
        set(JSON.parse(stored))
      } catch (e) {
        console.error('Error parsing stored auth:', e)
      }
    }
  }

  return {
    subscribe,
    login: (userData, token) => {
      // Clean token to ensure it only contains valid ASCII characters
      const cleanToken = token ? token.replace(/[^\x00-\x7F]/g, "") : null
      
      const authData = {
        isAuthenticated: true,
        user: userData,
        token: cleanToken,
        isCreator: userData.role === 'creator' || userData.isCreator
      }
      set(authData)
      if (typeof localStorage !== 'undefined') {
        try {
          localStorage.setItem('auth', JSON.stringify(authData))
        } catch (error) {
          console.error('Error storing auth data:', error)
        }
      }
    },
    logout: () => {
      const authData = {
        isAuthenticated: false,
        user: null,
        token: null,
        isCreator: false
      }
      set(authData)
      if (typeof localStorage !== 'undefined') {
        localStorage.removeItem('auth')
      }
    },
    update
  }
}

export const authStore = createAuthStore()

// Creator store for managing creator content
export const creatorStore = writable({
  events: [],
  packages: [],
  venues: [],
  analytics: {
    totalViews: 0,
    totalBookings: 0,
    revenue: 0
  }
})