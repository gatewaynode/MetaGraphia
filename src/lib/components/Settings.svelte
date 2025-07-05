<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { settingsStore, settingsActions } from '../stores/imageGeneration';
  import { onMount } from 'svelte';

  // Local state
  let outputDirectory = '';
  let modelPath = '';
  let availableModels: string[] = [];
  let isOpen = false;
  let isSaving = false;
  let error: string | null = null;
  let success: string | null = null;

  // Subscribe to settings
  settingsStore.subscribe(store => {
    outputDirectory = store.output_directory || '';
    modelPath = store.model_path || '';
  });

  onMount(async () => {
    settingsActions.loadSettings();
    try {
      availableModels = await invoke<string[]>('get_models');
    } catch (error) {
      console.error('Failed to load models:', error);
    }
  });

  async function handleSaveSettings() {
    if (!outputDirectory.trim()) {
      error = 'Output directory cannot be empty';
      return;
    }

    isSaving = true;
    error = null;
    success = null;

    try {
      await settingsActions.saveSettings({
        ...$settingsStore,
        output_directory: outputDirectory.trim(),
        model_path: modelPath.trim()
      });
      success = 'Settings saved successfully!';
    } catch (err) {
      error = err as string;
    } finally {
      isSaving = false;
    }
  }

  async function handleResetToDefaults() {
    isSaving = true;
    error = null;
    success = null;

    try {
      await settingsActions.resetToDefaults();
      success = 'Settings reset to defaults!';
    } catch (err) {
      error = err as string;
    } finally {
      isSaving = false;
    }
  }

  function handleBrowseDirectory() {
    // For now, we'll use a simple prompt
    // In a full implementation, you'd use Tauri's dialog API
    const newDir = prompt('Enter output directory path:', outputDirectory);
    if (newDir) {
      outputDirectory = newDir;
    }
  }
</script>

<div class="settings-container">
  <button 
    class="settings-toggle btn btn-secondary"
    on:click={() => isOpen = !isOpen}
  >
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="3"/>
      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1 1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
    </svg>
    Settings
  </button>

  {#if isOpen}
    <div class="settings-panel card glass fade-in">
      <div class="settings-header">
        <h3 class="settings-title">Application Settings</h3>
        <button 
          class="close-btn btn btn-icon"
          on:click={() => isOpen = false}
          aria-label="Close settings"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="settings-content">
        <!-- Model Selection Setting -->
        <div class="setting-group">
          <label for="model-path" class="setting-label">
            <span class="label-text">Stable Diffusion Model</span>
            <span class="label-hint">Select the model file for image generation</span>
          </label>
          <div class="input-group">
            <select
              id="model-path"
              bind:value={modelPath}
              class="setting-input"
              disabled={isSaving}
            >
              <option value="">No model selected</option>
              {#each availableModels as model}
                <option value={model}>{model}</option>
              {/each}
            </select>
          </div>
          <p class="setting-help">
            {#if modelPath}
              Selected model: {modelPath}
            {:else}
              No model selected. Please download a Stable Diffusion model (.tdict file) to generate images.
            {/if}
          </p>
        </div>

        <!-- Output Directory Setting -->
        <div class="setting-group">
          <label for="output-directory" class="setting-label">
            <span class="label-text">Output Directory</span>
            <span class="label-hint">Where generated images will be saved</span>
          </label>
          <div class="input-group">
            <input
              id="output-directory"
              type="text"
              bind:value={outputDirectory}
              placeholder="~/Desktop"
              class="setting-input"
              disabled={isSaving}
            />
            <button 
              class="browse-btn btn btn-secondary"
              on:click={handleBrowseDirectory}
              disabled={isSaving}
            >
              Browse
            </button>
          </div>
          <p class="setting-help">
            Generated images will be saved to this directory with timestamps and dimensions in the filename.
          </p>
        </div>

        <!-- Default Parameters Display -->
        <div class="setting-group">
          <h4 class="setting-subtitle">Default Generation Parameters</h4>
          <div class="params-grid">
            <div class="param-item">
              <span class="param-label">Width:</span>
              <span class="param-value">{$settingsStore.default_width}px</span>
            </div>
            <div class="param-item">
              <span class="param-label">Height:</span>
              <span class="param-value">{$settingsStore.default_height}px</span>
            </div>
            <div class="param-item">
              <span class="param-label">Inference Steps:</span>
              <span class="param-value">{$settingsStore.default_inference_steps}</span>
            </div>
            <div class="param-item">
              <span class="param-label">Guidance Scale:</span>
              <span class="param-value">{$settingsStore.default_guidance_scale}</span>
            </div>
          </div>
        </div>

        <!-- Error/Success Messages -->
        {#if error}
          <div class="error-message">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="15" y1="9" x2="9" y2="15"/>
              <line x1="9" y1="9" x2="15" y2="15"/>
            </svg>
            <span>{error}</span>
          </div>
        {/if}

        {#if success}
          <div class="success-message">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20,6 9,17 4,12"/>
            </svg>
            <span>{success}</span>
          </div>
        {/if}

        <!-- Action Buttons -->
        <div class="settings-actions">
          <button 
            class="save-btn btn btn-primary"
            on:click={handleSaveSettings}
            disabled={isSaving}
          >
            {#if isSaving}
              <div class="loading-spinner"></div>
              Saving...
            {:else}
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                <polyline points="17,21 17,13 7,13 7,21"/>
                <polyline points="7,3 7,8 15,8"/>
              </svg>
              Save Settings
            {/if}
          </button>
          
          <button 
            class="reset-btn btn btn-secondary"
            on:click={handleResetToDefaults}
            disabled={isSaving}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="1,4 1,10 7,10"/>
              <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1,10"/>
            </svg>
            Reset to Defaults
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .settings-container {
    position: relative;
  }

  .settings-toggle {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
  }

  .settings-toggle svg {
    width: 16px;
    height: 16px;
  }

  .settings-panel {
    position: absolute;
    top: 100%;
    right: 0;
    width: 500px;
    max-width: 90vw;
    z-index: 1000;
    margin-top: var(--spacing-sm);
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-lg);
  }

  .settings-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .close-btn {
    padding: var(--spacing-xs);
  }

  .close-btn svg {
    width: 16px;
    height: 16px;
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .setting-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .setting-label {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .label-text {
    font-size: var(--font-size-md);
    font-weight: 600;
    color: var(--text-primary);
  }

  .label-hint {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .input-group {
    display: flex;
    gap: var(--spacing-sm);
  }

  .setting-input {
    flex: 1;
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    transition: all var(--transition-normal);
  }

  .setting-input:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px rgba(147, 51, 234, 0.1);
  }

  .setting-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .browse-btn {
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
    white-space: nowrap;
  }

  .setting-help {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin: 0;
  }

  .setting-subtitle {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--spacing-sm) 0;
  }

  .params-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
    background: var(--bg-tertiary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
  }

  .param-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .param-label {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .param-value {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .error-message,
  .success-message {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
  }

  .error-message {
    background: rgba(255, 107, 107, 0.1);
    border: 1px solid rgba(255, 107, 107, 0.3);
    color: #ff6b6b;
  }

  .success-message {
    background: rgba(0, 212, 170, 0.1);
    border: 1px solid rgba(0, 212, 170, 0.3);
    color: #00d4aa;
  }

  .error-message svg,
  .success-message svg {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .settings-actions {
    display: flex;
    gap: var(--spacing-md);
    justify-content: flex-end;
  }

  .save-btn,
  .reset-btn {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-lg);
    font-size: var(--font-size-sm);
  }

  .save-btn svg,
  .reset-btn svg {
    width: 16px;
    height: 16px;
  }

  .loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .settings-panel {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      width: 100%;
      max-width: none;
      margin: 0;
      border-radius: 0;
      z-index: 10000;
    }

    .settings-actions {
      flex-direction: column;
    }

    .params-grid {
      grid-template-columns: 1fr;
    }
  }
</style> 