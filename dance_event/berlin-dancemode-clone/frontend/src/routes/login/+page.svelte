<script>
  import { authAPI } from '$lib/api.js';
  import { authHelpers } from '$lib/stores.js';
  import { goto } from '$app/navigation';
  
  let email = '';
  let password = '';
  let isLoading = false;
  let error = null;
  
  async function handleLogin() {
    if (!email || !password) {
      error = 'Please fill in all fields';
      return;
    }
    
    isLoading = true;
    error = null;
    
    try {
      const response = await authAPI.login({ email, password });
      
      if (response.success) {
        authHelpers.login(response.data.user, response.data.token);
        goto('/dashboard');
      } else {
        error = response.message || 'Login failed';
      }
    } catch (err) {
      error = 'Network error. Please try again.';
      console.error('Login error:', err);
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>Login - Berlin DanceMode</title>
</svelte:head>

<div class="login-page">
  <div class="container">
    <div class="login-form">
      <h1>Welcome Back</h1>
      <p>Sign in to your Berlin DanceMode account</p>
      
      <form on:submit|preventDefault={handleLogin}>
        {#if error}
          <div class="error-message">{error}</div>
        {/if}
        
        <div class="form-group">
          <label for="email">Email</label>
          <input
            type="email"
            id="email"
            bind:value={email}
            required
            disabled={isLoading}
          />
        </div>
        
        <div class="form-group">
          <label for="password">Password</label>
          <input
            type="password"
            id="password"
            bind:value={password}
            required
            disabled={isLoading}
          />
        </div>
        
        <button type="submit" class="btn btn-primary" disabled={isLoading}>
          {#if isLoading}
            Signing in...
          {:else}
            Sign In
          {/if}
        </button>
      </form>
      
      <p class="login-footer">
        Don't have an account? <a href="/register">Create one here</a>
      </p>
    </div>
  </div>
</div>

<style>
  .login-page {
    min-height: 80vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 0;
  }
  
  .container {
    max-width: 400px;
    width: 100%;
    padding: 0 1rem;
  }
  
  .login-form {
    background: white;
    padding: 3rem 2rem;
    border-radius: 10px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
    text-align: center;
  }
  
  h1 {
    color: #333;
    margin-bottom: 0.5rem;
  }
  
  p {
    color: #666;
    margin-bottom: 2rem;
  }
  
  .form-group {
    text-align: left;
    margin-bottom: 1.5rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #333;
    font-weight: 500;
  }
  
  input {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #e1e1e1;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.3s ease;
  }
  
  input:focus {
    outline: none;
    border-color: #667eea;
  }
  
  .btn {
    width: 100%;
    padding: 0.75rem;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .btn-primary {
    background: #667eea;
    color: white;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: #5a6fd8;
  }
  
  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .error-message {
    background: #fee2e2;
    color: #dc2626;
    padding: 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
  }
  
  .login-footer {
    margin-top: 2rem;
    color: #666;
  }
  
  .login-footer a {
    color: #667eea;
    text-decoration: none;
  }
  
  .login-footer a:hover {
    text-decoration: underline;
  }
</style>