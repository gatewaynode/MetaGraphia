<script>
  import ImageGenerator from '../lib/components/ImageGenerator.svelte';
</script>

<main class="container">
  <header class="app-header">
    <h1>DiffusionBee Tauri</h1>
    <p>AI Image Generation with Stable Diffusion</p>
  </header>

  <ImageGenerator />
</main>

<style>
  .container {
    margin: 0;
    padding: 20px;
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  .app-header {
    text-align: center;
    margin-bottom: 40px;
    color: white;
  }

  .app-header h1 {
    font-size: 2.5rem;
    margin: 0 0 10px 0;
    font-weight: 700;
  }

  .app-header p {
    font-size: 1.1rem;
    margin: 0;
    opacity: 0.9;
  }
</style>
