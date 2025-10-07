import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Create a custom store that persists to localStorage
function createPersistedStore(key, initialValue) {
  let stored = initialValue;
  
  if (browser && localStorage.getItem(key)) {
    try {
      stored = JSON.parse(localStorage.getItem(key));
    } catch (e) {
      console.error(`Error parsing stored value for ${key}:`, e);
    }
  }
  
  const { subscribe, set, update } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      if (browser) {
        localStorage.setItem(key, JSON.stringify(value));
      }
      set(value);
    },
    update: (updater) => {
      update((current) => {
        const newValue = updater(current);
        if (browser) {
          localStorage.setItem(key, JSON.stringify(newValue));
        }
        return newValue;
      });
    },
    clear: () => {
      if (browser) {
        localStorage.removeItem(key);
      }
      set(initialValue);
    }
  };
}

// Auth store
export const authStore = createPersistedStore('auth', {
  isAuthenticated: false,
  user: null,
  token: null
});

// Theme store
export const themeStore = createPersistedStore('theme', 'light');

// Navigation store
export const navigationStore = writable({
  isMenuOpen: false,
  currentPage: 'home'
});

// Events store
export const eventsStore = writable([]);

// Packages store
export const packagesStore = writable([]);

// Venues store
export const venuesStore = writable([]);

// Loading states
export const loadingStore = writable({
  events: false,
  packages: false,
  venues: false,
  auth: false
});

// Error store
export const errorStore = writable(null);

// Success message store
export const successStore = writable(null);

// Cart store for event/package registrations
export const cartStore = createPersistedStore('cart', {
  items: [],
  total: 0
});

// Helper functions for auth
export const authHelpers = {
  login: (user, token) => {
    authStore.set({
      isAuthenticated: true,
      user,
      token
    });
  },
  
  logout: () => {
    authStore.clear();
    cartStore.clear();
  },
  
  updateUser: (userData) => {
    authStore.update(current => ({
      ...current,
      user: { ...current.user, ...userData }
    }));
  }
};

// Helper functions for cart
export const cartHelpers = {
  addItem: (item) => {
    cartStore.update(current => {
      const existingItem = current.items.find(i => 
        i.id === item.id && i.type === item.type
      );
      
      if (existingItem) {
        return current; // Item already in cart
      }
      
      const newItems = [...current.items, item];
      const newTotal = newItems.reduce((sum, item) => sum + item.price, 0);
      
      return {
        items: newItems,
        total: newTotal
      };
    });
  },
  
  removeItem: (itemId, itemType) => {
    cartStore.update(current => {
      const newItems = current.items.filter(item => 
        !(item.id === itemId && item.type === itemType)
      );
      const newTotal = newItems.reduce((sum, item) => sum + item.price, 0);
      
      return {
        items: newItems,
        total: newTotal
      };
    });
  },
  
  clearCart: () => {
    cartStore.clear();
  }
};

// Notification helpers
export const notifications = {
  showError: (message) => {
    errorStore.set(message);
    setTimeout(() => errorStore.set(null), 5000);
  },
  
  showSuccess: (message) => {
    successStore.set(message);
    setTimeout(() => successStore.set(null), 5000);
  },
  
  clearAll: () => {
    errorStore.set(null);
    successStore.set(null);
  }
};