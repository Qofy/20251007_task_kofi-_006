import { writable } from 'svelte/store'

// Simple hash function for Gravatar (simulating MD5)
function simpleHash(str) {
  let hash = 0
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash // Convert to 32-bit integer
  }
  return Math.abs(hash).toString(16)
}

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
    login: async (email, password) => {
      // For demo purposes, simulate login with mock user data
      // In a real app, this would make an API call
      const userData = {
        id: '1',
        firstName: email.split('@')[0].charAt(0).toUpperCase() + email.split('@')[0].slice(1),
        lastName: 'User',
        username: email.split('@')[0],
        email: email,
        isCreator: email.includes('creator'),
        role: email.includes('creator') ? 'creator' : 'user',
        avatar: `https://www.gravatar.com/avatar/${simpleHash(username.toLowerCase().trim())}?s=200&d=identicon`
      }
      
      const authData = {
        isAuthenticated: true,
        user: userData,
        token: 'demo-token-' + Date.now(),
        isCreator: userData.isCreator
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
    setUserData: (userData, token) => {
      // This method is for when you already have user data (legacy compatibility)
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
    register: async (firstName, lastName, username, email, password, isCreator) => {
      // For demo purposes, we'll simulate registration
      // In a real app, this would make an API call
      
      // Create a hash for Gravatar
      const emailHash = simpleHash(email.toLowerCase().trim())
      
      const userData = {
        id: Date.now().toString(),
        firstName,
        lastName,
        username,
        email,
        isCreator,
        role: isCreator ? 'creator' : 'user',
        avatar: `https://www.gravatar.com/avatar/${emailHash}?s=200&d=identicon`
      }
      
      const authData = {
        isAuthenticated: true,
        user: userData,
        token: 'demo-token-' + Date.now(),
        isCreator
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