<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import api from '$lib/api.js';
  
  let selectedPackage = null;
  let loading = true;
  let paymentForm = {
    cardNumber: '',
    expiryDate: '',
    cvv: '',
    cardholderName: '',
    billingAddress: {
      street: '',
      city: '',
      zipCode: '',
      country: 'Germany'
    }
  };
  let processing = false;
  let paymentSuccess = false;
  let error = null;

  onMount(async () => {
    const packageId = $page.url.searchParams.get('package');
    
    if (!packageId) {
      goto('/packages');
      return;
    }

    try {
      const response = await api.get('/api/packages');
      const packages = Array.isArray(response.data) ? response.data : [];
      selectedPackage = packages.find(pkg => pkg.id === packageId);
      
      if (!selectedPackage) {
        error = 'Package not found';
        setTimeout(() => goto('/packages'), 2000);
      }
    } catch (err) {
      error = 'Failed to load package details';
      console.error('Error loading package:', err);
    } finally {
      loading = false;
    }
  });

  function formatCardNumber(value) {
    // Remove all non-digits and add spaces every 4 digits
    const digits = value.replace(/\D/g, '');
    const formatted = digits.replace(/(\d{4})(?=\d)/g, '$1 ');
    paymentForm.cardNumber = formatted.slice(0, 19); // Max 16 digits + 3 spaces
  }

  function formatExpiryDate(value) {
    // Format as MM/YY
    const digits = value.replace(/\D/g, '');
    if (digits.length >= 2) {
      paymentForm.expiryDate = digits.slice(0, 2) + '/' + digits.slice(2, 4);
    } else {
      paymentForm.expiryDate = digits;
    }
  }

  function formatCVV(value) {
    // Only allow 3-4 digits
    paymentForm.cvv = value.replace(/\D/g, '').slice(0, 4);
  }

  async function processPayment() {
    if (!validateForm()) return;
    
    processing = true;
    error = null;

    try {
      // Simulate payment processing
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // In a real app, you'd integrate with a payment processor like Stripe
      const paymentData = {
        packageId: selectedPackage.id,
        amount: selectedPackage.price,
        currency: 'EUR',
        paymentMethod: {
          type: 'card',
          cardNumber: paymentForm.cardNumber.replace(/\s/g, ''),
          expiryDate: paymentForm.expiryDate,
          cardholderName: paymentForm.cardholderName
        }
      };

      // For demo purposes, we'll just show success
      paymentSuccess = true;
      
      // Reset form
      paymentForm = {
        cardNumber: '',
        expiryDate: '',
        cvv: '',
        cardholderName: '',
        billingAddress: {
          street: '',
          city: '',
          zipCode: '',
          country: 'Germany'
        }
      };
    } catch (err) {
      error = 'Payment processing failed. Please try again.';
      console.error('Payment error:', err);
    } finally {
      processing = false;
    }
  }

  function validateForm() {
    if (!paymentForm.cardNumber || paymentForm.cardNumber.replace(/\s/g, '').length < 13) {
      error = 'Please enter a valid card number';
      return false;
    }
    
    if (!paymentForm.expiryDate || !paymentForm.expiryDate.match(/^\d{2}\/\d{2}$/)) {
      error = 'Please enter a valid expiry date (MM/YY)';
      return false;
    }
    
    if (!paymentForm.cvv || paymentForm.cvv.length < 3) {
      error = 'Please enter a valid CVV';
      return false;
    }
    
    if (!paymentForm.cardholderName.trim()) {
      error = 'Please enter the cardholder name';
      return false;
    }

    return true;
  }

  function formatPrice(price) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR'
    }).format(price);
  }
</script>

<svelte:head>
  <title>Payment - Berlin DanceMode</title>
</svelte:head>

<div class="payment-page">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading payment details...</p>
    </div>
  {:else if paymentSuccess}
    <div class="success-container">
      <div class="success-card">
        <div class="success-icon">✅</div>
        <h1>Payment Successful!</h1>
        <p>Thank you for purchasing <strong>{selectedPackage.name}</strong></p>
        <p class="amount">Amount paid: {formatPrice(selectedPackage.price)}</p>
        <p>You will receive a confirmation email shortly with your booking details.</p>
        
        <div class="success-actions">
          <button class="primary-btn" on:click={() => goto('/')}>
            Return to Home
          </button>
          <button class="secondary-btn" on:click={() => goto('/packages')}>
            View More Packages
          </button>
        </div>
      </div>
    </div>
  {:else if selectedPackage}
    <div class="container">
      <div class="payment-header">
        <button class="back-btn" on:click={() => goto('/packages')}>
          ← Back to Packages
        </button>
        <h1>Complete Your Purchase</h1>
      </div>

      <div class="payment-content">
        <!-- Package Summary -->
        <div class="package-summary">
          <h2>Order Summary</h2>
          <div class="package-details">
            <h3>{selectedPackage.name}</h3>
            <p class="package-description">{selectedPackage.description}</p>
            <div class="package-features">
              <p><strong>Duration:</strong> {selectedPackage.duration_days} day{selectedPackage.duration_days > 1 ? 's' : ''}</p>
              <p><strong>Max Participants:</strong> {selectedPackage.max_participants}</p>
            </div>
            <div class="price-breakdown">
              <div class="price-row">
                <span>Package Price</span>
                <span>{formatPrice(selectedPackage.price)}</span>
              </div>
              <div class="price-row">
                <span>Processing Fee</span>
                <span>{formatPrice(selectedPackage.price * 0.03)}</span>
              </div>
              <div class="price-total">
                <span>Total</span>
                <span>{formatPrice(selectedPackage.price * 1.03)}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Payment Form -->
        <div class="payment-form">
          <h2>Payment Information</h2>
          
          {#if error}
            <div class="error-message">
              {error}
            </div>
          {/if}

          <form on:submit|preventDefault={processPayment}>
            <div class="form-group">
              <label for="cardNumber">Card Number</label>
              <input
                type="text"
                id="cardNumber"
                bind:value={paymentForm.cardNumber}
                on:input={(e) => formatCardNumber(e.target.value)}
                placeholder="1234 5678 9012 3456"
                maxlength="19"
                required
              />
            </div>

            <div class="form-row">
              <div class="form-group">
                <label for="expiryDate">Expiry Date</label>
                <input
                  type="text"
                  id="expiryDate"
                  bind:value={paymentForm.expiryDate}
                  on:input={(e) => formatExpiryDate(e.target.value)}
                  placeholder="MM/YY"
                  maxlength="5"
                  required
                />
              </div>

              <div class="form-group">
                <label for="cvv">CVV</label>
                <input
                  type="text"
                  id="cvv"
                  bind:value={paymentForm.cvv}
                  on:input={(e) => formatCVV(e.target.value)}
                  placeholder="123"
                  maxlength="4"
                  required
                />
              </div>
            </div>

            <div class="form-group">
              <label for="cardholderName">Cardholder Name</label>
              <input
                type="text"
                id="cardholderName"
                bind:value={paymentForm.cardholderName}
                placeholder="John Doe"
                required
              />
            </div>

            <h3>Billing Address</h3>
            
            <div class="form-group">
              <label for="street">Street Address</label>
              <input
                type="text"
                id="street"
                bind:value={paymentForm.billingAddress.street}
                placeholder="123 Main Street"
                required
              />
            </div>

            <div class="form-row">
              <div class="form-group">
                <label for="city">City</label>
                <input
                  type="text"
                  id="city"
                  bind:value={paymentForm.billingAddress.city}
                  placeholder="Berlin"
                  required
                />
              </div>

              <div class="form-group">
                <label for="zipCode">ZIP Code</label>
                <input
                  type="text"
                  id="zipCode"
                  bind:value={paymentForm.billingAddress.zipCode}
                  placeholder="10115"
                  required
                />
              </div>
            </div>

            <div class="form-group">
              <label for="country">Country</label>
              <select id="country" bind:value={paymentForm.billingAddress.country}>
                <option value="Germany">Germany</option>
                <option value="Austria">Austria</option>
                <option value="Switzerland">Switzerland</option>
                <option value="Netherlands">Netherlands</option>
                <option value="France">France</option>
                <option value="Other">Other</option>
              </select>
            </div>

            <button type="submit" class="pay-btn" disabled={processing}>
              {#if processing}
                <div class="btn-spinner"></div>
                Processing Payment...
              {:else}
                Pay {formatPrice(selectedPackage.price * 1.03)}
              {/if}
            </button>
          </form>

          <div class="payment-security">
            <p>🔒 Your payment information is secure and encrypted</p>
            <p>💳 We accept Visa, Mastercard, and American Express</p>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="error-container">
      <h1>Package Not Found</h1>
      <p>The requested package could not be found.</p>
      <button class="primary-btn" on:click={() => goto('/packages')}>
        View All Packages
      </button>
    </div>
  {/if}
</div>

<style>
  .payment-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 2rem 0;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 50vh;
    color: white;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid rgba(255, 255, 255, 0.3);
    border-top: 4px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .payment-header {
    margin-bottom: 2rem;
  }

  .back-btn {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 5px;
    cursor: pointer;
    margin-bottom: 1rem;
    transition: background-color 0.3s;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.3);
  }

  .payment-header h1 {
    color: white;
    font-size: 2.5rem;
    margin: 0;
  }

  .payment-content {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 3rem;
  }

  .package-summary {
    background: white;
    padding: 2rem;
    border-radius: 15px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
    height: fit-content;
  }

  .package-summary h2 {
    margin-top: 0;
    color: #333;
    border-bottom: 2px solid #667eea;
    padding-bottom: 0.5rem;
  }

  .package-details h3 {
    color: #667eea;
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .package-description {
    color: #666;
    margin-bottom: 1.5rem;
  }

  .package-features {
    margin-bottom: 2rem;
  }

  .package-features p {
    margin: 0.5rem 0;
    color: #555;
  }

  .price-breakdown {
    border-top: 1px solid #eee;
    padding-top: 1rem;
  }

  .price-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    color: #555;
  }

  .price-total {
    display: flex;
    justify-content: space-between;
    font-weight: bold;
    font-size: 1.2rem;
    color: #333;
    border-top: 2px solid #667eea;
    padding-top: 0.5rem;
    margin-top: 1rem;
  }

  .payment-form {
    background: white;
    padding: 2rem;
    border-radius: 15px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  }

  .payment-form h2 {
    margin-top: 0;
    color: #333;
    border-bottom: 2px solid #667eea;
    padding-bottom: 0.5rem;
  }

  .payment-form h3 {
    margin-top: 2rem;
    margin-bottom: 1rem;
    color: #667eea;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: bold;
    color: #333;
  }

  input, select {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.3s;
  }

  input:focus, select:focus {
    outline: none;
    border-color: #667eea;
  }

  .error-message {
    background: #ffebee;
    color: #c62828;
    padding: 1rem;
    border-radius: 5px;
    margin-bottom: 1rem;
    border-left: 4px solid #c62828;
  }

  .pay-btn {
    width: 100%;
    padding: 1rem;
    background: linear-gradient(45deg, #667eea, #764ba2);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: bold;
    cursor: pointer;
    transition: transform 0.3s, box-shadow 0.3s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 2rem;
  }

  .pay-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(102, 126, 234, 0.4);
  }

  .pay-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .payment-security {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid #eee;
    text-align: center;
  }

  .payment-security p {
    margin: 0.5rem 0;
    color: #666;
    font-size: 0.9rem;
  }

  .success-container {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 70vh;
  }

  .success-card {
    background: white;
    padding: 3rem;
    border-radius: 20px;
    box-shadow: 0 15px 40px rgba(0, 0, 0, 0.2);
    text-align: center;
    max-width: 500px;
  }

  .success-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .success-card h1 {
    color: #4caf50;
    margin-bottom: 1rem;
  }

  .amount {
    font-size: 1.2rem;
    font-weight: bold;
    color: #667eea;
    margin: 1rem 0;
  }

  .success-actions {
    margin-top: 2rem;
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .primary-btn, .secondary-btn {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.3s;
  }

  .primary-btn {
    background: linear-gradient(45deg, #667eea, #764ba2);
    color: white;
    border: none;
  }

  .secondary-btn {
    background: transparent;
    color: #667eea;
    border: 2px solid #667eea;
  }

  .primary-btn:hover, .secondary-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
  }

  .error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 50vh;
    color: white;
    text-align: center;
  }

  @media (max-width: 768px) {
    .payment-content {
      grid-template-columns: 1fr;
      gap: 2rem;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .success-actions {
      flex-direction: column;
    }

    .payment-header h1 {
      font-size: 2rem;
    }
  }
</style>