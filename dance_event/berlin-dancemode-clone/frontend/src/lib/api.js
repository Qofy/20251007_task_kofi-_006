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

// Data management API
export const dataAPI = {
  exportAll: async () => {
    const response = await api.get('/api/data/export');
    return response.data;
  },
  
  importAll: async (data) => {
    const response = await api.post('/api/data/import', data);
    return response.data;
  },
  
  getStatistics: async () => {
    const response = await api.get('/api/data/statistics');
    return response.data;
  },
  
  clearAll: async () => {
    const response = await api.delete('/api/data/clear');
    return response.data;
  },
  
  bulkCreateEvents: async (events, replaceExisting = false) => {
    const response = await api.post('/api/data/bulk/events', {
      data: events,
      replace_existing: replaceExisting
    });
    return response.data;
  },
  
  bulkCreateVenues: async (venues, replaceExisting = false) => {
    const response = await api.post('/api/data/bulk/venues', {
      data: venues,
      replace_existing: replaceExisting
    });
    return response.data;
  },
  
  bulkCreatePackages: async (packages, replaceExisting = false) => {
    const response = await api.post('/api/data/bulk/packages', {
      data: packages,
      replace_existing: replaceExisting
    });
    return response.data;
  }
};

// Comprehensive data loading for all frontend elements
export const frontendDataAPI = {
  loadAllData: async () => {
    try {
      const [eventsRes, venuesRes, packagesRes, statsRes] = await Promise.all([
        eventsAPI.getAll(),
        venuesAPI.getAll(),
        packagesAPI.getAll(),
        dataAPI.getStatistics()
      ]);

      return {
        events: eventsRes.data || [],
        venues: venuesRes.data || [],
        packages: packagesRes.data || [],
        statistics: statsRes.data || {},
        success: true
      };
    } catch (error) {
      console.error('Error loading all frontend data:', error);
      return {
        events: [],
        venues: [],
        packages: [],
        statistics: {},
        success: false,
        error: error.message
      };
    }
  },

  saveAllData: async (data) => {
    try {
      const results = {
        events: { success: 0, errors: 0 },
        venues: { success: 0, errors: 0 },
        packages: { success: 0, errors: 0 }
      };

      // Save venues first (events might depend on venues)
      if (data.venues && data.venues.length > 0) {
        const venueResult = await dataAPI.bulkCreateVenues(data.venues, data.replaceExisting);
        if (venueResult.success) {
          results.venues = {
            success: venueResult.data.success_count,
            errors: venueResult.data.error_count
          };
        }
      }

      // Save packages
      if (data.packages && data.packages.length > 0) {
        const packageResult = await dataAPI.bulkCreatePackages(data.packages, data.replaceExisting);
        if (packageResult.success) {
          results.packages = {
            success: packageResult.data.success_count,
            errors: packageResult.data.error_count
          };
        }
      }

      // Save events
      if (data.events && data.events.length > 0) {
        const eventResult = await dataAPI.bulkCreateEvents(data.events, data.replaceExisting);
        if (eventResult.success) {
          results.events = {
            success: eventResult.data.success_count,
            errors: eventResult.data.error_count
          };
        }
      }

      return {
        success: true,
        results
      };
    } catch (error) {
      console.error('Error saving all data:', error);
      return {
        success: false,
        error: error.message
      };
    }
  }
};

// Health check
export const healthAPI = {
  check: async () => {
    const response = await api.get('/health');
    return response.data;
  }
};

export default api;