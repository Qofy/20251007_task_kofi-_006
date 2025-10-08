import axios from 'axios'
import { authStore } from '../stores/auth.js'

const API_BASE_URL = 'http://localhost:8081'

// Create axios instance
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Add auth token to requests
api.interceptors.request.use((config) => {
  try {
    const auth = JSON.parse(localStorage.getItem('auth') || '{}')
    if (auth.token) {
      // Ensure token contains only valid ASCII characters
      const cleanToken = auth.token.replace(/[^\x00-\x7F]/g, "")
      if (cleanToken && cleanToken.length > 0) {
        config.headers.Authorization = `Bearer ${cleanToken}`
      }
    }
  } catch (error) {
    console.error('Error processing auth token:', error)
    // Clear corrupted auth data
    localStorage.removeItem('auth')
  }
  return config
})

// API methods
export const apiService = {
  // Authentication
  auth: {
    login: (credentials) => api.post('/api/auth/login', credentials),
    register: (userData) => api.post('/api/auth/register', userData),
    logout: () => api.post('/api/auth/logout'),
  },

  // Events
  events: {
    getAll: () => api.get('/api/events'),
    getById: (id) => api.get(`/api/events/${id}`),
    create: (eventData) => api.post('/api/events', eventData),
    update: (id, eventData) => api.put(`/api/events/${id}`, eventData),
    delete: (id) => api.delete(`/api/events/${id}`),
  },

  // Packages
  packages: {
    getAll: () => api.get('/api/packages'),
    getById: (id) => api.get(`/api/packages/${id}`),
    create: (packageData) => api.post('/api/packages', packageData),
    update: (id, packageData) => api.put(`/api/packages/${id}`, packageData),
    delete: (id) => api.delete(`/api/packages/${id}`),
  },

  // Venues
  venues: {
    getAll: () => api.get('/api/venues'),
    getById: (id) => api.get(`/api/venues/${id}`),
    create: (venueData) => api.post('/api/venues', venueData),
    update: (id, venueData) => api.put(`/api/venues/${id}`, venueData),
    delete: (id) => api.delete(`/api/venues/${id}`),
  },

  // Creator specific
  creator: {
    getMyContent: () => api.get('/api/creator/content'),
    getAnalytics: () => api.get('/api/creator/analytics'),
    saveElement: (elementData) => api.post('/api/creator/elements', elementData),
    loadElements: () => api.get('/api/creator/elements'),
    updateElement: (id, elementData) => api.put(`/api/creator/elements/${id}`, elementData),
    deleteElement: (id) => api.delete(`/api/creator/elements/${id}`),
  },

  // Frontend elements management
  elements: {
    save: (elementData) => api.post('/api/elements', elementData),
    load: () => api.get('/api/elements'),
    update: (id, elementData) => api.put(`/api/elements/${id}`, elementData),
    delete: (id) => api.delete(`/api/elements/${id}`),
  }
}

export default apiService