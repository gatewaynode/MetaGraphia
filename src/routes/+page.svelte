<script>
  import ImageGenerator from '../lib/components/ImageGenerator.svelte';
  import Settings from '../lib/components/Settings.svelte';
  import '../app.css';
</script>

<main class="app-container">
  <!-- Animated Background -->
  <div class="background">
    <div class="gradient-orb orb-1"></div>
    <div class="gradient-orb orb-2"></div>
    <div class="gradient-orb orb-3"></div>
    <div class="noise-overlay"></div>
  </div>

  <!-- Header -->
  <header class="app-header fade-in">
    <div class="container">
      <div class="header-content">
        <div class="logo-section">
          <h1 class="app-title">
            <span class="text-gradient">DiffusionBee</span>
            <span class="text-muted">Tauri</span>
          </h1>
          <p class="app-subtitle">AI Image Generation with Stable Diffusion</p>
        </div>
        <div class="header-actions">
          <div class="status-indicator">
            <div class="status-dot"></div>
            <span class="status-text">Ready</span>
          </div>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <div class="main-content slide-up">
    <div class="container">
      <div class="header-section">
        <h1 class="app-title">
          <span class="text-gradient">DiffusionBee</span> Tauri
        </h1>
        <Settings />
      </div>
      <ImageGenerator />
    </div>
  </div>
</main>

<style>
  .app-container {
    min-height: 100vh;
    position: relative;
    overflow-x: hidden;
  }

  /* Animated Background */
  .background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    background: linear-gradient(135deg, #0a0a0f 0%, #1a1a2e 50%, #16213e 100%);
  }

  .gradient-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.3;
    animation: float 20s ease-in-out infinite;
  }

  .orb-1 {
    width: 400px;
    height: 400px;
    background: radial-gradient(circle, rgba(100, 255, 218, 0.4) 0%, transparent 70%);
    top: -100px;
    left: -100px;
    animation-delay: 0s;
  }

  .orb-2 {
    width: 300px;
    height: 300px;
    background: radial-gradient(circle, rgba(0, 212, 170, 0.3) 0%, transparent 70%);
    top: 50%;
    right: -50px;
    animation-delay: -7s;
  }

  .orb-3 {
    width: 250px;
    height: 250px;
    background: radial-gradient(circle, rgba(100, 255, 218, 0.2) 0%, transparent 70%);
    bottom: -50px;
    left: 50%;
    animation-delay: -14s;
  }

  @keyframes float {
    0%, 100% {
      transform: translateY(0px) rotate(0deg);
    }
    33% {
      transform: translateY(-30px) rotate(120deg);
    }
    66% {
      transform: translateY(20px) rotate(240deg);
    }
  }

  .noise-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
    opacity: 0.02;
    pointer-events: none;
  }

  /* Header Styles */
  .app-header {
    padding: var(--spacing-xl) 0;
    position: relative;
    z-index: 10;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .logo-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .app-title {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    margin: 0;
    letter-spacing: -0.02em;
    display: flex;
    align-items: baseline;
    gap: var(--spacing-sm);
  }

  .app-subtitle {
    font-size: var(--font-size-lg);
    color: var(--text-secondary);
    margin: 0;
    font-weight: 400;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--spacing-lg);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    backdrop-filter: var(--glass-backdrop);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    background: var(--accent-primary);
    border-radius: 50%;
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.7;
      transform: scale(1.1);
    }
  }

  .status-text {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    font-weight: 500;
  }

  /* Main Content */
  .main-content {
    padding: var(--spacing-2xl) 0;
    position: relative;
    z-index: 10;
  }

  .header-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-2xl);
    padding-bottom: var(--spacing-lg);
    border-bottom: 1px solid var(--border-primary);
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .header-content {
      flex-direction: column;
      gap: var(--spacing-lg);
      text-align: center;
    }

    .app-title {
      font-size: var(--font-size-2xl);
    }

    .app-subtitle {
      font-size: var(--font-size-md);
    }

    .main-content {
      padding: var(--spacing-xl) 0;
    }
  }
</style>
