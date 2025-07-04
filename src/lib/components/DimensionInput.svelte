<script lang="ts">
  import { generationStore, generationActions } from '../stores/imageGeneration';

  // Subscribe to store values
  $: ({ params } = $generationStore);
  $: width = params.img_width;
  $: height = params.img_height;

  // Aspect ratio presets
  const aspectRatios = [
    { name: 'Square', ratio: 1, width: 512, height: 512 },
    { name: 'Portrait', ratio: 0.75, width: 512, height: 682 },
    { name: 'Landscape', ratio: 1.33, width: 682, height: 512 },
    { name: 'Wide', ratio: 1.77, width: 768, height: 432 }
  ];

  function updateDimensions(newWidth: number, newHeight: number) {
    generationActions.updateDimension('width', newWidth);
    generationActions.updateDimension('height', newHeight);
  }

  function applyAspectRatio(preset: typeof aspectRatios[0]) {
    updateDimensions(preset.width, preset.height);
  }

  function validateDimension(value: string): boolean {
    const num = parseInt(value);
    return !isNaN(num) && num >= 256 && num <= 1024;
  }

  function handleWidthChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const newWidth = parseInt(target.value);
    if (validateDimension(target.value)) {
      updateDimensions(newWidth, height);
    }
  }

  function handleHeightChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const newHeight = parseInt(target.value);
    if (validateDimension(target.value)) {
      updateDimensions(width, newHeight);
    }
  }

  function formatPixelCount(): string {
    const count = width * height;
    if (count >= 1000000) {
      return `${(count / 1000000).toFixed(1)}M`;
    } else if (count >= 1000) {
      return `${(count / 1000).toFixed(0)}K`;
    }
    return count.toString();
  }
</script>

<div class="dimension-input card glass">
  <div class="dimension-header">
    <h3 class="dimension-title">Image Dimensions</h3>
    <p class="dimension-subtitle">Set the size of your generated image</p>
  </div>

  <div class="dimension-content">
    <!-- Width and Height Inputs -->
    <div class="dimension-controls">
      <div class="dimension-group">
        <label for="width-input" class="dimension-label">
          <span class="label-text">Width</span>
          <span class="label-unit">pixels</span>
        </label>
        <input
          id="width-input"
          type="number"
          bind:value={width}
          on:change={handleWidthChange}
          min="256"
          max="1024"
          step="64"
          class="dimension-field input"
          placeholder="512"
        />
      </div>

      <div class="dimension-separator">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"/>
        </svg>
      </div>

      <div class="dimension-group">
        <label for="height-input" class="dimension-label">
          <span class="label-text">Height</span>
          <span class="label-unit">pixels</span>
        </label>
        <input
          id="height-input"
          type="number"
          bind:value={height}
          on:change={handleHeightChange}
          min="256"
          max="1024"
          step="64"
          class="dimension-field input"
          placeholder="512"
        />
      </div>
    </div>

    <!-- Pixel Count Display -->
    <div class="pixel-info">
      <div class="pixel-count">
        <span class="count-label">Total Pixels:</span>
        <span class="count-value text-accent">{formatPixelCount()}</span>
      </div>
      <div class="aspect-ratio">
        <span class="ratio-label">Aspect Ratio:</span>
        <span class="ratio-value text-muted">{(width / height).toFixed(2)}:1</span>
      </div>
    </div>

    <!-- Aspect Ratio Presets -->
    <div class="preset-section">
      <h4 class="preset-title">Quick Presets</h4>
      <div class="preset-grid">
        {#each aspectRatios as preset}
          <button
            class="preset-btn btn btn-secondary"
            class:active={width === preset.width && height === preset.height}
            on:click={() => applyAspectRatio(preset)}
          >
            <div class="preset-icon">
              <div class="preset-shape" style="aspect-ratio: {preset.ratio}"></div>
            </div>
            <span class="preset-name">{preset.name}</span>
            <span class="preset-size">{preset.width}×{preset.height}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Validation Messages -->
    <div class="validation-info">
      <div class="validation-item">
        <svg class="validation-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="m9 12 2 2 4-4"/>
        </svg>
        <span class="validation-text">Minimum: 256×256 pixels</span>
      </div>
      <div class="validation-item">
        <svg class="validation-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="m9 12 2 2 4-4"/>
        </svg>
        <span class="validation-text">Maximum: 1024×1024 pixels</span>
      </div>
    </div>
  </div>
</div>

<style>
  .dimension-input {
    padding: var(--spacing-xl);
    max-width: 600px;
    margin: 0 auto;
  }

  .dimension-header {
    text-align: center;
    margin-bottom: var(--spacing-xl);
  }

  .dimension-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .dimension-subtitle {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0;
  }

  .dimension-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .dimension-controls {
    display: flex;
    align-items: flex-end;
    gap: var(--spacing-md);
    justify-content: center;
  }

  .dimension-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    min-width: 120px;
  }

  .dimension-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label-text {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .label-unit {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  .dimension-field {
    text-align: center;
    font-weight: 600;
    font-size: var(--font-size-lg);
  }

  .dimension-separator {
    display: flex;
    align-items: center;
    padding: 0 var(--spacing-sm);
    color: var(--text-muted);
  }

  .dimension-separator svg {
    width: 20px;
    height: 20px;
  }

  .pixel-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-primary);
  }

  .pixel-count, .aspect-ratio {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .count-label, .ratio-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .count-value {
    font-size: var(--font-size-lg);
    font-weight: 700;
  }

  .ratio-value {
    font-size: var(--font-size-md);
    font-weight: 500;
  }

  .preset-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .preset-title {
    font-size: var(--font-size-md);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    text-align: center;
  }

  .preset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: var(--spacing-sm);
  }

  .preset-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-md);
    min-height: 80px;
    position: relative;
  }

  .preset-btn.active {
    background: var(--accent-gradient);
    color: var(--bg-primary);
    border-color: var(--accent-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-glow);
  }

  .preset-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .preset-shape {
    width: 100%;
    height: 100%;
    background: currentColor;
    border-radius: var(--radius-sm);
    opacity: 0.3;
  }

  .preset-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
  }

  .preset-size {
    font-size: var(--font-size-xs);
    opacity: 0.8;
  }

  .validation-info {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .validation-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm);
    background: rgba(100, 255, 218, 0.05);
    border: 1px solid rgba(100, 255, 218, 0.1);
    border-radius: var(--radius-sm);
  }

  .validation-icon {
    width: 16px;
    height: 16px;
    color: var(--accent-primary);
    flex-shrink: 0;
  }

  .validation-text {
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .dimension-input {
      padding: var(--spacing-lg);
      margin: 0 var(--spacing-md);
    }

    .dimension-controls {
      flex-direction: column;
      align-items: center;
      gap: var(--spacing-lg);
    }

    .dimension-separator {
      transform: rotate(90deg);
    }

    .pixel-info {
      flex-direction: column;
      gap: var(--spacing-md);
      text-align: center;
    }

    .preset-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style> 