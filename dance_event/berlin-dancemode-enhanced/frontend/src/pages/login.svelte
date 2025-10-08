<script>
  import { authStore } from '../stores/auth.js'
  import { url, goto } from '@roxi/routify'

  let email = ''
  let password = ''
  let loading = false
  let error = ''

  async function handleLogin() {
    if (!email || !password) {
      error = 'Please fill in all fields'
      return
    }

    loading = true
    error = ''

    try {
      await authStore.login(email, password)
      $goto('/')
    } catch (err) {
      error = err.message || 'Login failed'
    } finally {
      loading = false
    }
  }

  function handleKeydown(event) {
    if (event.key === 'Enter') {
      handleLogin()
    }
  }
</script>

<main class="login-container">
  <div class="login-card">
    <div class="login-header">
      <h1>Welcome Back</h1>
      <p>Sign in to your Berlin DanceMode account</p>
    </div>

    <form on:submit|preventDefault={handleLogin} class="login-form">
      {#if error}
        <div class="error-message">
          {error}
        </div>
      {/if}

      <div class="form-group">
        <label for="email">Email</label>
        <input
          id="email"
          type="email"
          bind:value={email}
          on:keydown={handleKeydown}
          placeholder="Enter your email"
          required
          disabled={loading}
        />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input
          id="password"
          type="password"
          bind:value={password}
          on:keydown={handleKeydown}
          placeholder="Enter your password"
          required
          disabled={loading}
        />
      </div>

      <button type="submit" class="login-btn" disabled={loading}>
        {#if loading}
          <span class="spinner"></span>
          Signing in...
        {:else}
          Sign In
        {/if}
      </button>
    </form>

    <div class="login-footer">
      <p>Don't have an account?</p>
      <a href={$url('/register')} class="register-link">Create one here</a>
    </div>

    <div class="back-home">
      <a href={$url('/')} class="back-link">← Back to Home</a>
    </div>
  </div>
</main>

<style>
  .login-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 2rem;
  }

  .login-card {
    background: white;
    border-radius: 16px;
    padding: 3rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 400px;
  }

  .login-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .login-header h1 {
    font-size: 2rem;
    font-weight: bold;
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .login-header p {
    color: #718096;
    font-size: 0.9rem;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .error-message {
    background: #fed7d7;
    color: #c53030;
    padding: 0.75rem;
    border-radius: 8px;
    font-size: 0.9rem;
    text-align: center;
    border: 1px solid #feb2b2;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-weight: 600;
    color: #4a5568;
    font-size: 0.9rem;
  }

  .form-group input {
    padding: 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 1rem;
    transition: all 0.2s ease;
    background: white;
  }

  .form-group input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .form-group input:disabled {
    background: #f7fafc;
    cursor: not-allowed;
  }

  .login-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    padding: 0.875rem;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
  }

  .login-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .login-footer {
    text-align: center;
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 1px solid #e2e8f0;
  }

  .login-footer p {
    color: #718096;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
  }

  .register-link {
    color: #667eea;
    text-decoration: none;
    font-weight: 600;
    transition: color 0.2s ease;
  }

  .register-link:hover {
    color: #764ba2;
    text-decoration: underline;
  }

  .back-home {
    text-align: center;
    margin-top: 1rem;
  }

  .back-link {
    color: #718096;
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.2s ease;
  }

  .back-link:hover {
    color: #4a5568;
  }

  @media (max-width: 480px) {
    .login-container {
      padding: 1rem;
    }

    .login-card {
      padding: 2rem;
    }

    .login-header h1 {
      font-size: 1.75rem;
    }
  }
</style>