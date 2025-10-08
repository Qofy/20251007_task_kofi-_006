<script>
  import { authAPI } from '../lib/api.js';
  
  let email = '';
  let password = '';
  let loading = false;
  let error = null;
  
  async function handleLogin() {
    if (!email || !password) {
      error = 'Please fill in all fields';
      return;
    }
    
    loading = true;
    error = null;
    
    try {
      const result = await authAPI.login({ email, password });
      if (result.success) {
        // Redirect to dashboard or home
        window.location.hash = '#/';
      } else {
        error = result.message || 'Login failed';
      }
    } catch (err) {
      error = 'Login failed. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<div class="login-page">
  <div class="login-container">
    <div class="login-form">
      <h1>Login</h1>
      <p>Welcome back to Berlin DanceMode</p>
      
      {#if error}
        <div class="error-message">{error}</div>
      {/if}
      
      <form on:submit|preventDefault={handleLogin}>
        <div class="form-group">
          <label for="email">Email</label>
          <input 
            type="email" 
            id="email" 
            bind:value={email} 
            required 
            disabled={loading}
          />
        </div>
        
        <div class="form-group">
          <label for="password">Password</label>
          <input 
            type="password" 
            id="password" 
            bind:value={password} 
            required 
            disabled={loading}
          />
        </div>
        
        <button type="submit" class="btn btn-primary" disabled={loading}>
          {loading ? 'Logging in...' : 'Login'}
        </button>
      </form>
      
      <p class="signup-link">
        Don't have an account? <a href="#/register">Sign up</a>
      </p>
    </div>
  </div>
</div>

<style>
  .login-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 50%, #667eea 100%);
  }

  .login-container {
    width: 100%;
    max-width: 400px;
    padding: 2rem;
  }

  .login-form {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  }

  .login-form h1 {
    text-align: center;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .login-form p {
    text-align: center;
    color: #4a5568;
    margin-bottom: 2rem;
  }

  .error-message {
    background: #fed7d7;
    color: #c53030;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    text-align: center;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    color: #4a5568;
    font-weight: 600;
  }

  .form-group input {
    width: 100%;
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.3s;
  }

  .form-group input:focus {
    outline: none;
    border-color: #667eea;
  }

  .form-group input:disabled {
    background: #f7fafc;
    cursor: not-allowed;
  }

  .btn {
    width: 100%;
    padding: 1rem 2rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.3s;
    margin-bottom: 1rem;
  }

  .btn:hover:not(:disabled) {
    background: #5a67d8;
  }

  .btn:disabled {
    background: #a0aec0;
    cursor: not-allowed;
  }

  .signup-link {
    text-align: center;
    color: #4a5568;
  }

  .signup-link a {
    color: #667eea;
    text-decoration: none;
    font-weight: 600;
  }

  .signup-link a:hover {
    text-decoration: underline;
  }
</style>