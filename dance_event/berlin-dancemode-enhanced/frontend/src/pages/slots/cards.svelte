<div class="card">
  <div class="card-inner">
    <slot/>
    <div class="header">
      <slot name="header"/>
    </div>
    <div class="content">
      <slot name="content"/>
    </div>
    <div class="footer">
      <slot name="footer"/>
    </div>
  </div>
</div>

<style>
  .card {
    width: 280px;
    height: 26rem;
    margin: 1rem;
    perspective: 1000px;
    cursor: pointer;
  }

  .card-inner {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 16px;
    box-shadow: 
      0 8px 32px rgba(0, 0, 0, 0.1),
      0 2px 8px rgba(0, 0, 0, 0.08);
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
  }

  .card-inner::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, 
      rgba(255, 255, 255, 0.1) 0%, 
      rgba(255, 255, 255, 0.05) 50%, 
      rgba(0, 0, 0, 0.1) 100%);
    pointer-events: none;
    z-index: 1;
  }

  .card:hover .card-inner {
    transform: translateY(-8px) rotateX(5deg);
    box-shadow: 
      0 20px 60px rgba(0, 0, 0, 0.2),
      0 8px 20px rgba(0, 0, 0, 0.15);
  }

  .header {
    position: relative;
    z-index: 2;
    padding: 1.5rem 1.5rem 0.5rem;
    color: white;
    font-weight: 600;
    font-size: 1.2rem;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .content {
    position: relative;
    z-index: 2;
    flex: 1;
    padding: 0.5rem 1.5rem;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.6;
    overflow: hidden;
  }

  .footer {
    position: relative;
    z-index: 2;
    padding: 0.5rem 1.5rem 1.5rem;
    margin-top: auto;
    color: rgba(255, 255, 255, 0.8);
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(0, 0, 0, 0.1);
  }

  /* Alternative card styles */
  .card:nth-child(2n) .card-inner {
    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  }

  .card:nth-child(3n) .card-inner {
    background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  }

  .card:nth-child(4n) .card-inner {
    background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  }

  .card:nth-child(5n) .card-inner {
    background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .card {
      width: 100%;
      max-width: 320px;
      margin: 0.5rem auto;
    }
    
    .card-inner {
      height: 22rem;
    }
  }

  /* Animation for smooth loading */
  @keyframes cardFadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .card {
    animation: cardFadeIn 0.6s ease-out forwards;
  }
</style>


