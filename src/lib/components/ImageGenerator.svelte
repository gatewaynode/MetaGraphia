<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { generationStore, generationActions, type GenerationParams } from '../stores/imageGeneration';
  import { onMount } from 'svelte';

  // Local state
  let prompt = '';
  let isGenerating = false;
  let error: string | null = null;
  let progress = { current_step: 0, total_steps: 20, status: 'Ready' };
  let currentResult: any = null;

  // Subscribe to the store
  generationStore.subscribe(store => {
    isGenerating = store.isGenerating;
    error = store.error;
    progress = store.progress;
    currentResult = store.currentResult;
  });

  // Handle prompt input
  function handlePromptInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    prompt = target.value;
    generationActions.updateParams({ prompt });
  }

  // Handle generation
  async function handleGenerate() {
    if (!prompt.trim()) {
      generationActions.setError('Please enter a prompt');
      return;
    }

    try {
      generationActions.startGeneration();
      
      // Call the Tauri backend
      const result = await invoke('generate_image', {
        request: {
          prompt: prompt,
          img_width: 512,
          img_height: 512,
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

<div class="image-generator">
  <div class="generator-panel">
    <h2>Text-to-Image Generation</h2>
    
    <!-- Prompt Input -->
    <div class="input-group">
      <label for="prompt">Prompt</label>
      <textarea
        id="prompt"
        bind:value={prompt}
        on:input={handlePromptInput}
        placeholder="Enter your image description..."
        rows="3"
        disabled={isGenerating}
      ></textarea>
    </div>

    <!-- Generation Controls -->
    <div class="controls">
      <button 
        class="generate-btn"
        on:click={handleGenerate}
        disabled={isGenerating || !prompt.trim()}
      >
        {isGenerating ? 'Generating...' : 'Generate Image'}
      </button>
      
      {#if isGenerating}
        <button class="cancel-btn" on:click={handleCancel}>
          Cancel
        </button>
      {/if}
    </div>

    <!-- Progress -->
    {#if isGenerating}
      <div class="progress-container">
        <div class="progress-bar">
          <div 
            class="progress-fill" 
            style="width: {(progress.current_step / progress.total_steps) * 100}%"
          ></div>
        </div>
        <div class="progress-text">
          {progress.status} - Step {progress.current_step} of {progress.total_steps}
        </div>
      </div>
    {/if}

    <!-- Error Display -->
    {#if error}
      <div class="error-message">
        {error}
      </div>
    {/if}
  </div>

  <!-- Result Display -->
  {#if currentResult}
    <div class="result-panel">
      <h3>Generated Image</h3>
      <div class="image-container">
        <img 
          src={currentResult.generated_img_path} 
          alt="Generated image"
          class="generated-image"
        />
      </div>
      <div class="image-actions">
        <button class="action-btn">Save</button>
        <button class="action-btn">Share</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .image-generator {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }

  .generator-panel {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 24px;
    margin-bottom: 24px;
  }

  h2 {
    margin: 0 0 20px 0;
    color: #333;
    font-size: 1.5rem;
  }

  .input-group {
    margin-bottom: 20px;
  }

  label {
    display: block;
    margin-bottom: 8px;
    font-weight: 500;
    color: #555;
  }

  textarea {
    width: 100%;
    padding: 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 14px;
    font-family: inherit;
    resize: vertical;
    min-height: 80px;
  }

  textarea:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
  }

  textarea:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  .controls {
    display: flex;
    gap: 12px;
    margin-bottom: 20px;
  }

  .generate-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .generate-btn:hover:not(:disabled) {
    background: #0056b3;
  }

  .generate-btn:disabled {
    background: #6c757d;
    cursor: not-allowed;
  }

  .cancel-btn {
    background: #dc3545;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .cancel-btn:hover {
    background: #c82333;
  }

  .progress-container {
    margin-bottom: 20px;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: #e9ecef;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 8px;
  }

  .progress-fill {
    height: 100%;
    background: #007bff;
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 14px;
    color: #666;
    text-align: center;
  }

  .error-message {
    background: #f8d7da;
    color: #721c24;
    padding: 12px;
    border-radius: 6px;
    border: 1px solid #f5c6cb;
    margin-bottom: 20px;
  }

  .result-panel {
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 24px;
  }

  .result-panel h3 {
    margin: 0 0 16px 0;
    color: #333;
  }

  .image-container {
    text-align: center;
    margin-bottom: 16px;
  }

  .generated-image {
    max-width: 100%;
    max-height: 400px;
    border-radius: 6px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .image-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .action-btn {
    background: #6c757d;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .action-btn:hover {
    background: #5a6268;
  }
</style> 