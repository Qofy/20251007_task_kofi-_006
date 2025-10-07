import { writable, derived } from 'svelte/store';
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

// Derived stores for easy access
export const user = derived(authStore, $authStore => $authStore.user);
export const isAuthenticated = derived(authStore, $authStore => $authStore.isAuthenticated);

// Other stores
export const eventsStore = writable([]);
export const packagesStore = writable([]);
export const venuesStore = writable([]);
export const loadingStore = writable(false);
export const errorStore = writable(null);

// Auth helpers
export const authHelpers = {
  login: (userData, token) => {
    authStore.set({
      isAuthenticated: true,
      user: userData,
      token
    });
  },
  
  logout: () => {
    authStore.clear();
    if (browser) {
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
    }
  }
};