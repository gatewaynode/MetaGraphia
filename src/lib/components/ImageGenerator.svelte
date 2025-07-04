<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { generationStore, generationActions, settingsActions, type GenerationParams } from '../stores/imageGeneration';
  import { onMount } from 'svelte';
  import DimensionInput from './DimensionInput.svelte';

  // Local state
  let prompt = '';
  let isGenerating = false;
  let error: string | null = null;
  let progress = { current_step: 0, total_steps: 20, status: 'Ready' };
  let currentResult: any = null;
  let width = 512;
  let height = 512;

  // Subscribe to the store
  generationStore.subscribe(store => {
    isGenerating = store.isGenerating;
    error = store.error;
    progress = store.progress;
    currentResult = store.currentResult;
    width = store.params.img_width;
    height = store.params.img_height;
  });

  // Load settings on mount
  onMount(() => {
    settingsActions.loadSettings();
  });

  // Handle prompt input
  function handlePromptInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    prompt = target.value;
    generationActions.updateParams({ prompt });
  }

  // Handle dimension change
  function handleDimensionChange(event: CustomEvent) {
    const { width: newWidth, height: newHeight } = event.detail;
    width = newWidth;
    height = newHeight;
  }

  // Handle generation
  async function handleGenerate() {
    if (!prompt.trim()) {
      generationActions.setError('Please enter a prompt');
      return;
    }

    try {
      generationActions.startGeneration();
      
      // Call the Tauri backend with current parameters
      const result = await invoke('generate_image', {
        request: {
          prompt: prompt,
          img_width: width,
          img_height: height,
          num_imgs: 1,
          num_inference_steps: 20,
          guidance_scale: 7.5
        }
      });

      generationActions.setResult(result as any);
    } catch (err) {
      generationActions.setError(err as string);
    }
  }

  // Handle cancel
  async function handleCancel() {
    try {
      await invoke('cancel_generation');
      generationActions.setError('Generation cancelled');
    } catch (err) {
      generationActions.setError(err as string);
    }
  }
</script>

<div class="image-generator card glass fade-in">
  <div class="generator-header">
    <h2 class="section-title">
      <span class="text-gradient">Text-to-Image</span> Generation
    </h2>
    <p class="section-description">
      Transform your ideas into stunning visuals with AI-powered image generation
    </p>
  </div>

  <div class="generator-content">
    <!-- Prompt Input Section -->
    <div class="input-section">
      <label for="prompt-input" class="input-label">
        <span class="label-text">Prompt</span>
        <span class="label-hint">Describe the image you want to generate</span>
      </label>
      <textarea
        id="prompt-input"
        bind:value={prompt}
        placeholder="A serene landscape with mountains at sunset, digital art style..."
        class="prompt-input"
        rows="3"
        disabled={isGenerating}
      ></textarea>
    </div>

    <!-- Dimension Controls -->
    <div class="controls-section">
      <DimensionInput />
    </div>

    <!-- Generation Button -->
    <div class="action-section">
      <button
        on:click={handleGenerate}
        disabled={isGenerating || !prompt.trim()}
        class="generate-btn btn btn-primary"
      >
        {#if isGenerating}
          <div class="loading-spinner"></div>
          <span>Generating...</span>
        {:else}
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
          <span>Generate Image</span>
        {/if}
      </button>
    </div>

    <!-- Error Display -->
    {#if error}
      <div class="error-message">
        <svg class="error-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="15" y1="9" x2="9" y2="15"/>
          <line x1="9" y1="9" x2="15" y2="15"/>
        </svg>
        <span>{error}</span>
      </div>
    {/if}

    <!-- Progress Bar -->
    {#if isGenerating}
      <div class="progress-section">
        <div class="progress-bar">
          <div class="progress-fill"></div>
        </div>
        <p class="progress-text">Creating your masterpiece...</p>
      </div>
    {/if}

    <!-- Generated Image Display -->
    {#if currentResult}
      <div class="result-section slide-up">
        <h3 class="result-title">Generated Image</h3>
        <div class="image-container">
          <img
            src={currentResult.generated_img_path}
            alt="Generated image from prompt"
            class="generated-image"
          />
          <div class="image-overlay">
            <button class="btn btn-secondary overlay-btn">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7,10 12,15 17,10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              Download
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .image-generator {
    padding: var(--spacing-2xl);
    max-width: 800px;
    margin: 0 auto;
  }

  .generator-header {
    text-align: center;
    margin-bottom: var(--spacing-2xl);
  }

  .section-title {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    margin: 0 0 var(--spacing-sm) 0;
    letter-spacing: -0.02em;
  }

  .section-description {
    font-size: var(--font-size-lg);
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.5;
  }

  .generator-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .input-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .input-label {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .label-text {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .label-hint {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  .prompt-input {
    width: 100%;
    padding: var(--spacing-md);
    background: var(--glass-bg);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
    color: var(--text-primary);
    font-family: var(--font-family);
    font-size: var(--font-size-md);
    line-height: 1.5;
    resize: vertical;
    min-height: 80px;
    backdrop-filter: var(--glass-backdrop);
    transition: all var(--transition-normal);
  }

  .prompt-input:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(100, 255, 218, 0.1);
  }

  .prompt-input::placeholder {
    color: var(--text-muted);
  }

  .prompt-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .controls-section {
    display: flex;
    justify-content: center;
  }

  .action-section {
    display: flex;
    justify-content: center;
    padding: var(--spacing-lg) 0;
  }

  .generate-btn {
    padding: var(--spacing-md) var(--spacing-2xl);
    font-size: var(--font-size-lg);
    font-weight: 600;
    min-width: 200px;
    position: relative;
    overflow: hidden;
  }

  .generate-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none !important;
  }

  .btn-icon {
    width: 20px;
    height: 20px;
    margin-right: var(--spacing-sm);
  }

  .loading-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-right: var(--spacing-sm);
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background: rgba(255, 107, 107, 0.1);
    border: 1px solid rgba(255, 107, 107, 0.3);
    border-radius: var(--radius-md);
    color: #ff6b6b;
    font-size: var(--font-size-sm);
  }

  .error-icon {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    align-items: center;
  }

  .progress-bar {
    width: 100%;
    max-width: 400px;
    height: 6px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent-gradient);
    border-radius: var(--radius-sm);
    animation: progress 2s ease-in-out infinite;
  }

  @keyframes progress {
    0% {
      width: 0%;
    }
    50% {
      width: 70%;
    }
    100% {
      width: 100%;
    }
  }

  .progress-text {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0;
  }

  .result-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .result-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    text-align: center;
  }

  .image-container {
    position: relative;
    border-radius: var(--radius-lg);
    overflow: hidden;
    background: var(--bg-secondary);
    box-shadow: var(--shadow-lg);
  }

  .generated-image {
    width: 100%;
    height: auto;
    display: block;
    transition: transform var(--transition-normal);
  }

  .image-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity var(--transition-normal);
  }

  .image-container:hover .image-overlay {
    opacity: 1;
  }

  .image-container:hover .generated-image {
    transform: scale(1.02);
  }

  .overlay-btn {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
  }

  .overlay-btn svg {
    width: 16px;
    height: 16px;
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .image-generator {
      padding: var(--spacing-lg);
      margin: 0 var(--spacing-md);
    }

    .section-title {
      font-size: var(--font-size-xl);
    }

    .section-description {
      font-size: var(--font-size-md);
    }

    .generate-btn {
      min-width: 100%;
      padding: var(--spacing-md) var(--spacing-lg);
    }
  }
</style> 