
// Run this in browser console to clear corrupted auth data:

localStorage.removeItem('auth');
sessionStorage.clear();
console.log('Auth data cleared. Please refresh the page.');
location.reload();


