<script>
  import { authStore } from '../stores/auth.js'
  import { url, goto } from '@roxi/routify'

  let firstName = ''
  let lastName = ''
  let username = ''
  let email = ''
  let password = ''
  let confirmPassword = ''
  let isCreator = false
  let loading = false
  let error = ''

  async function handleRegister() {
    if (!firstName || !lastName || !username || !email || !password || !confirmPassword) {
      error = 'Please fill in all fields'
      return
    }

    if (password !== confirmPassword) {
      error = 'Passwords do not match'
      return
    }

    if (password.length < 6) {
      error = 'Password must be at least 6 characters long'
      return
    }

    loading = true
    error = ''

    try {
      await authStore.register(firstName, lastName, username, email, password, isCreator)
      $goto('/')
    } catch (err) {
      error = err.message || 'Registration failed'
    } finally {
      loading = false
    }
  }

  function handleKeydown(event) {
    if (event.key === 'Enter') {
      handleRegister()
    }
  }
</script>

<main class="register-container">
  <div class="register-card">
    <div class="register-header">
      <h1>Join Berlin DanceMode</h1>
      <p>Create your account and start exploring the Berlin nightlife scene</p>
    </div>

    <form on:submit|preventDefault={handleRegister} class="register-form">
      {#if error}
        <div class="error-message">
          {error}
        </div>
      {/if}

      <div class="form-row">
        <div class="form-group">
          <label for="firstName">First Name</label>
          <input
            id="firstName"
            type="text"
            bind:value={firstName}
            on:keydown={handleKeydown}
            placeholder="Your first name"
            required
            disabled={loading}
          />
        </div>

        <div class="form-group">
          <label for="lastName">Last Name</label>
          <input
            id="lastName"
            type="text"
            bind:value={lastName}
            on:keydown={handleKeydown}
            placeholder="Your last name"
            required
            disabled={loading}
          />
        </div>
      </div>

      <div class="form-group">
        <label for="username">Username</label>
        <input
          id="username"
          type="text"
          bind:value={username}
          on:keydown={handleKeydown}
          placeholder="Choose a username"
          required
          disabled={loading}
        />
      </div>

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
          placeholder="Create a password"
          required
          disabled={loading}
        />
      </div>

      <div class="form-group">
        <label for="confirmPassword">Confirm Password</label>
        <input
          id="confirmPassword"
          type="password"
          bind:value={confirmPassword}
          on:keydown={handleKeydown}
          placeholder="Confirm your password"
          required
          disabled={loading}
        />
      </div>

      <div class="form-group checkbox-group">
        <label class="checkbox-label">
          <input
            type="checkbox"
            bind:checked={isCreator}
            disabled={loading}
          />
          <span class="checkbox-text">
            Register as a Creator
            <small>Get access to event management tools and analytics</small>
          </span>
        </label>
      </div>

      <button type="submit" class="register-btn" disabled={loading}>
        {#if loading}
          <span class="spinner"></span>
          Creating Account...
        {:else}
          Create Account
        {/if}
      </button>
    </form>

    <div class="register-footer">
      <p>Already have an account?</p>
      <a href={$url('/login')} class="login-link">Sign in here</a>
    </div>

    <div class="back-home">
      <a href={$url('/')} class="back-link">← Back to Home</a>
    </div>
  </div>
</main>

<style>
  .register-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 2rem;
  }

  .register-card {
    background: white;
    border-radius: 16px;
    padding: 3rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 450px;
  }

  .register-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .register-header h1 {
    font-size: 2rem;
    font-weight: bold;
    color: #2d3748;
    margin-bottom: 0.5rem;
  }

  .register-header p {
    color: #718096;
    font-size: 0.9rem;
    line-height: 1.4;
  }

  .register-form {
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

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-row .form-group {
    margin-bottom: 0;
  }

  .form-group label {
    font-weight: 600;
    color: #4a5568;
    font-size: 0.9rem;
  }

  .form-group input[type="text"],
  .form-group input[type="email"],
  .form-group input[type="password"] {
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

  .checkbox-group {
    margin: 0.5rem 0;
  }

  .checkbox-label {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    cursor: pointer;
    font-weight: normal;
  }

  .checkbox-label input[type="checkbox"] {
    width: 18px;
    height: 18px;
    margin-top: 2px;
    accent-color: #667eea;
  }

  .checkbox-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .checkbox-text small {
    color: #718096;
    font-size: 0.8rem;
    line-height: 1.3;
  }

  .register-btn {
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

  .register-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
  }

  .register-btn:disabled {
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

  .register-footer {
    text-align: center;
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 1px solid #e2e8f0;
  }

  .register-footer p {
    color: #718096;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
  }

  .login-link {
    color: #667eea;
    text-decoration: none;
    font-weight: 600;
    transition: color 0.2s ease;
  }

  .login-link:hover {
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
    .register-container {
      padding: 1rem;
    }

    .register-card {
      padding: 2rem;
    }

    .register-header h1 {
      font-size: 1.75rem;
    }

    .form-row {
      grid-template-columns: 1fr;
      gap: 0;
    }

    .form-row .form-group {
      margin-bottom: 1.5rem;
    }
  }
</style>