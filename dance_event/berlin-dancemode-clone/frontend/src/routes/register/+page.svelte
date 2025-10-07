<script>
  import { goto } from '$app/navigation';
  import api from '$lib/api.js';
  import { authHelpers } from '$lib/stores.js';
  
  let formData = {
    username: '',
    email: '',
    password: '',
    confirmPassword: ''
  };
  
  let errors = {};
  let loading = false;
  let showPassword = false;
  let showConfirmPassword = false;
  
  function validateForm() {
    errors = {};
    
    if (!formData.username.trim()) {
      errors.username = 'Username is required';
    } else if (formData.username.length < 3) {
      errors.username = 'Username must be at least 3 characters';
    }
    
    if (!formData.email.trim()) {
      errors.email = 'Email is required';
    } else if (!/\S+@\S+\.\S+/.test(formData.email)) {
      errors.email = 'Please enter a valid email address';
    }
    
    if (!formData.password) {
      errors.password = 'Password is required';
    } else if (formData.password.length < 6) {
      errors.password = 'Password must be at least 6 characters';
    }
    
    if (formData.password !== formData.confirmPassword) {
      errors.confirmPassword = 'Passwords do not match';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit(event) {
    event.preventDefault();
    
    if (!validateForm()) {
      return;
    }
    
    loading = true;
    
    try {
      const response = await api.post('/api/auth/register', {
        username: formData.username,
        email: formData.email,
        password: formData.password
      });
      
      if (response.data.token) {
        authHelpers.login(response.data.token, response.data.user);
        goto('/dashboard');
      }
    } catch (error) {
      if (error.response?.status === 409) {
        errors.general = 'Username or email already exists';
      } else {
        errors.general = 'Registration failed. Please try again.';
      }
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Register - Berlin DanceMode</title>
  <meta name="description" content="Join Berlin DanceMode and unlock exclusive access to Berlin's underground electronic music scene" />
</svelte:head>

<div class="register-page">
  <div class="register-container">
    <div class="register-content">
      <!-- Left Side - Form -->
      <div class="register-form-section">
        <div class="form-header">
          <h1>Join Berlin DanceMode</h1>
          <p>Unlock exclusive access to Berlin's underground electronic music scene</p>
        </div>
        
        <form on:submit={handleSubmit} class="register-form">
          {#if errors.general}
            <div class="error-message general-error">
              {errors.general}
            </div>
          {/if}
          
          <div class="form-group">
            <label for="username">Username</label>
            <input
              id="username"
              type="text"
              bind:value={formData.username}
              placeholder="Choose a username"
              class:error={errors.username}
              disabled={loading}
            />
            {#if errors.username}
              <span class="error-message">{errors.username}</span>
            {/if}
          </div>
          
          <div class="form-group">
            <label for="email">Email Address</label>
            <input
              id="email"
              type="email"
              bind:value={formData.email}
              placeholder="your.email@example.com"
              class:error={errors.email}
              disabled={loading}
            />
            {#if errors.email}
              <span class="error-message">{errors.email}</span>
            {/if}
          </div>
          
          <div class="form-group">
            <label for="password">Password</label>
            <div class="password-input">
              <input
                id="password"
                type={showPassword ? 'text' : 'password'}
                bind:value={formData.password}
                placeholder="Create a secure password"
                class:error={errors.password}
                disabled={loading}
              />
              <button
                type="button"
                class="password-toggle"
                on:click={() => showPassword = !showPassword}
              >
                {#if showPassword}
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd" />
                    <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z" />
                  </svg>
                {:else}
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                    <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
                  </svg>
                {/if}
              </button>
            </div>
            {#if errors.password}
              <span class="error-message">{errors.password}</span>
            {/if}
          </div>
          
          <div class="form-group">
            <label for="confirmPassword">Confirm Password</label>
            <div class="password-input">
              <input
                id="confirmPassword"
                type={showConfirmPassword ? 'text' : 'password'}
                bind:value={formData.confirmPassword}
                placeholder="Confirm your password"
                class:error={errors.confirmPassword}
                disabled={loading}
              />
              <button
                type="button"
                class="password-toggle"
                on:click={() => showConfirmPassword = !showConfirmPassword}
              >
                {#if showConfirmPassword}
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd" />
                    <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z" />
                  </svg>
                {:else}
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                    <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
                  </svg>
                {/if}
              </button>
            </div>
            {#if errors.confirmPassword}
              <span class="error-message">{errors.confirmPassword}</span>
            {/if}
          </div>
          
          <button type="submit" class="register-btn" disabled={loading}>
            {#if loading}
              <div class="spinner"></div>
              Creating Account...
            {:else}
              Create Account
            {/if}
          </button>
        </form>
        
        <div class="form-footer">
          <p>Already have an account? <a href="/login">Sign in here</a></p>
        </div>
        
        <div class="social-register">
          <div class="divider">
            <span>Or register with</span>
          </div>
          
          <div class="social-buttons">
            <button class="social-btn google" disabled title="Coming Soon">
              <svg viewBox="0 0 24 24">
                <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
              </svg>
              Google (Coming Soon)
            </button>
            
            <button class="social-btn facebook" disabled title="Coming Soon">
              <svg viewBox="0 0 24 24">
                <path fill="#1877F2" d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/>
              </svg>
              Facebook (Coming Soon)
            </button>
          </div>
        </div>
      </div>
      
      <!-- Right Side - Benefits -->
      <div class="register-benefits-section">
        <div class="benefits-content">
          <h2>Why Join Berlin DanceMode?</h2>
          
          <div class="benefits-list">
            <div class="benefit-item">
              <div class="benefit-icon">🎫</div>
              <div class="benefit-text">
                <h3>Exclusive Access</h3>
                <p>Skip the line at partner venues and get access to members-only events</p>
              </div>
            </div>
            
            <div class="benefit-item">
              <div class="benefit-icon">🎵</div>
              <div class="benefit-text">
                <h3>Curated Events</h3>
                <p>Handpicked events from Berlin's underground electronic music scene</p>
              </div>
            </div>
            
            <div class="benefit-item">
              <div class="benefit-icon">👥</div>
              <div class="benefit-text">
                <h3>Community</h3>
                <p>Connect with like-minded electronic music enthusiasts from around the world</p>
              </div>
            </div>
            
            <div class="benefit-item">
              <div class="benefit-icon">💰</div>
              <div class="benefit-text">
                <h3>Special Pricing</h3>
                <p>Member discounts on events, packages, and exclusive merchandise</p>
              </div>
            </div>
            
            <div class="benefit-item">
              <div class="benefit-icon">📧</div>
              <div class="benefit-text">
                <h3>Inside Info</h3>
                <p>Weekly newsletter with insider tips, venue recommendations, and scene updates</p>
              </div>
            </div>
            
            <div class="benefit-item">
              <div class="benefit-icon">🎯</div>
              <div class="benefit-text">
                <h3>Personalized</h3>
                <p>Tailored recommendations based on your music preferences and past events</p>
              </div>
            </div>
          </div>
          
          <div class="testimonial">
            <blockquote>
              "Berlin DanceMode opened doors I never knew existed. The underground scene here is incredible, and they're the key to experiencing it authentically."
            </blockquote>
            <cite>— Maria K., Barcelona</cite>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .register-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .register-container {
    width: 100%;
    max-width: 1200px;
  }

  .register-content {
    display: grid;
    grid-template-columns: 1fr 1fr;
    background: white;
    border-radius: 24px;
    overflow: hidden;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
  }

  /* Form Section */
  .register-form-section {
    padding: 3rem;
  }

  .form-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .form-header h1 {
    font-size: 2.5rem;
    color: #1a202c;
    margin-bottom: 0.5rem;
    font-weight: 700;
  }

  .form-header p {
    color: #4a5568;
    font-size: 1.1rem;
  }

  .register-form {
    margin-bottom: 2rem;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    font-weight: 600;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .form-group input {
    width: 100%;
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    font-size: 1rem;
    transition: border-color 0.3s;
    background: white;
  }

  .form-group input:focus {
    outline: none;
    border-color: #667eea;
  }

  .form-group input.error {
    border-color: #f56565;
  }

  .form-group input:disabled {
    background: #f7fafc;
    cursor: not-allowed;
  }

  .password-input {
    position: relative;
  }

  .password-toggle {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    cursor: pointer;
    color: #4a5568;
    padding: 0.25rem;
  }

  .password-toggle svg {
    width: 1.25rem;
    height: 1.25rem;
  }

  .error-message {
    color: #f56565;
    font-size: 0.875rem;
    margin-top: 0.25rem;
    display: block;
  }

  .general-error {
    background: #fed7d7;
    color: #c53030;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    text-align: center;
  }

  .register-btn {
    width: 100%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    padding: 1rem;
    border-radius: 12px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.3s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .register-btn:hover:not(:disabled) {
    transform: translateY(-2px);
  }

  .register-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid transparent;
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .form-footer {
    text-align: center;
    margin-bottom: 2rem;
  }

  .form-footer p {
    color: #4a5568;
  }

  .form-footer a {
    color: #667eea;
    text-decoration: none;
    font-weight: 600;
  }

  .form-footer a:hover {
    text-decoration: underline;
  }

  /* Social Register */
  .social-register {
    margin-top: 2rem;
  }

  .divider {
    text-align: center;
    margin-bottom: 1rem;
    position: relative;
  }

  .divider::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 0;
    right: 0;
    height: 1px;
    background: #e2e8f0;
    z-index: 1;
  }

  .divider span {
    background: white;
    padding: 0 1rem;
    color: #4a5568;
    position: relative;
    z-index: 2;
  }

  .social-buttons {
    display: flex;
    gap: 1rem;
  }

  .social-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    background: white;
    cursor: pointer;
    transition: all 0.3s;
    font-weight: 500;
  }

  .social-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .social-btn:hover:not(:disabled) {
    border-color: #cbd5e0;
    transform: translateY(-2px);
  }

  .social-btn svg {
    width: 1.25rem;
    height: 1.25rem;
  }

  /* Benefits Section */
  .register-benefits-section {
    background: linear-gradient(135deg, #1a202c 0%, #2d3748 100%);
    color: white;
    padding: 3rem;
    display: flex;
    align-items: center;
  }

  .benefits-content h2 {
    font-size: 2rem;
    margin-bottom: 2rem;
    text-align: center;
  }

  .benefits-list {
    margin-bottom: 3rem;
  }

  .benefit-item {
    display: flex;
    align-items: flex-start;
    margin-bottom: 1.5rem;
  }

  .benefit-icon {
    font-size: 2rem;
    margin-right: 1rem;
    flex-shrink: 0;
  }

  .benefit-text h3 {
    font-size: 1.1rem;
    margin-bottom: 0.25rem;
    color: #667eea;
  }

  .benefit-text p {
    color: #cbd5e0;
    line-height: 1.5;
    font-size: 0.9rem;
  }

  .testimonial {
    background: rgba(255, 255, 255, 0.1);
    padding: 1.5rem;
    border-radius: 12px;
    border-left: 4px solid #667eea;
  }

  .testimonial blockquote {
    font-style: italic;
    margin-bottom: 1rem;
    line-height: 1.6;
  }

  .testimonial cite {
    font-size: 0.875rem;
    opacity: 0.8;
  }

  @media (max-width: 968px) {
    .register-content {
      grid-template-columns: 1fr;
    }

    .register-benefits-section {
      order: -1;
    }

    .register-form-section,
    .register-benefits-section {
      padding: 2rem;
    }

    .form-header h1 {
      font-size: 2rem;
    }

    .social-buttons {
      flex-direction: column;
    }
  }

  @media (max-width: 640px) {
    .register-page {
      padding: 1rem;
    }

    .register-form-section,
    .register-benefits-section {
      padding: 1.5rem;
    }

    .benefits-content h2 {
      font-size: 1.5rem;
    }

    .benefit-item {
      flex-direction: column;
      align-items: center;
      text-align: center;
    }

    .benefit-icon {
      margin-right: 0;
      margin-bottom: 0.5rem;
    }
  }
</style>