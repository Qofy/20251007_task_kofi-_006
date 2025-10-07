import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:8080';

// Create axios instance with default config
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor to handle errors
api.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    if (error.response?.status === 401) {
      // Clear token on unauthorized access
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authAPI = {
  register: async (userData) => {
    const response = await api.post('/api/auth/register', userData);
    return response.data;
  },
  
  login: async (credentials) => {
    const response = await api.post('/api/auth/login', credentials);
    if (response.data.success && response.data.data.token) {
      localStorage.setItem('auth_token', response.data.data.token);
      localStorage.setItem('user', JSON.stringify(response.data.data.user));
    }
    return response.data;
  },
  
  logout: () => {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
  }
};

// Events API
export const eventsAPI = {
  getAll: async () => {
    const response = await api.get('/api/events');
    return response.data;
  },
  
  getById: async (id) => {
    const response = await api.get(`/api/events/${id}`);
    return response.data;
  },
  
  create: async (eventData) => {
    const response = await api.post('/api/events', eventData);
    return response.data;
  }
};

// Venues API
export const venuesAPI = {
  getAll: async () => {
    const response = await api.get('/api/venues');
    return response.data;
  },
  
  create: async (venueData) => {
    const response = await api.post('/api/venues', venueData);
    return response.data;
  }
};

// Packages API
export const packagesAPI = {
  getAll: async () => {
    const response = await api.get('/api/packages');
    return response.data;
  },
  
  create: async (packageData) => {
    const response = await api.post('/api/packages', packageData);
    return response.data;
  }
};

// Registrations API
export const registrationsAPI = {
  create: async (registrationData) => {
    const response = await api.post('/api/registrations', registrationData);
    return response.data;
  },
  
  getUserRegistrations: async () => {
    const response = await api.get('/api/users/registrations');
    return response.data;
  },
  
  processPayment: async (registrationId, paymentData) => {
    const response = await api.post(`/api/registrations/${registrationId}/payment`, paymentData);
    return response.data;
  }
};

// Users API
export const usersAPI = {
  getProfile: async () => {
    const response = await api.get('/api/users/profile');
    return response.data;
  },
  
  updateProfile: async (userData) => {
    const response = await api.put('/api/users/profile', userData);
    return response.data;
  }
};

// Health check
export const healthAPI = {
  check: async () => {
    const response = await api.get('/health');
    return response.data;
  }
};

// Utility functions
export const isAuthenticated = () => {
  return !!localStorage.getItem('auth_token');
};

export const getCurrentUser = () => {
  const user = localStorage.getItem('user');
  return user ? JSON.parse(user) : null;
};

export default api;