<script lang="ts">
  import { generationStore, generationActions } from '../stores/imageGeneration';

  // Subscribe to store values
  $: ({ params } = $generationStore);
  $: inferenceSteps = params.num_inference_steps;
  $: guidanceScale = params.guidance_scale;

  // Slider configurations
  const inferenceConfig = {
    min: 10,
    max: 50,
    step: 1,
    default: 20
  };

  const guidanceConfig = {
    min: 1,
    max: 20,
    step: 0.5,
    default: 7.5
  };

  function updateInferenceSteps(value: number) {
    generationActions.updateParams({ num_inference_steps: value });
  }

  function updateGuidanceScale(value: number) {
    generationActions.updateParams({ guidance_scale: value });
  }

  function resetToDefaults() {
    updateInferenceSteps(inferenceConfig.default);
    updateGuidanceScale(guidanceConfig.default);
  }

  // Helper function to get quality indicator
  function getQualityIndicator(steps: number): { label: string; color: string; description: string } {
    if (steps >= 40) return { label: 'Ultra', color: 'var(--accent-primary)', description: 'Highest quality, slower generation' };
    if (steps >= 30) return { label: 'High', color: '#64ffda', description: 'High quality, balanced speed' };
    if (steps >= 20) return { label: 'Good', color: '#00d4aa', description: 'Good quality, faster generation' };
    return { label: 'Fast', color: '#ff6b6b', description: 'Fast generation, lower quality' };
  }

  function getGuidanceIndicator(scale: number): { label: string; color: string; description: string } {
    if (scale >= 15) return { label: 'Strict', color: '#ff6b6b', description: 'Very strict prompt adherence' };
    if (scale >= 10) return { label: 'Strong', color: '#ffa726', description: 'Strong prompt adherence' };
    if (scale >= 5) return { label: 'Balanced', color: '#64ffda', description: 'Balanced creativity and adherence' };
    return { label: 'Creative', color: '#00d4aa', description: 'More creative, less strict' };
  }
</script>

<div class="advanced-controls card glass">
  <div class="controls-header">
    <h3 class="controls-title">Advanced Controls</h3>
    <p class="controls-subtitle">Fine-tune your image generation parameters</p>
    <button class="reset-btn btn btn-secondary" on:click={resetToDefaults}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
        <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/>
        <path d="M3 21v-5h5"/>
      </svg>
      Reset to Defaults
    </button>
  </div>

  <div class="controls-content">
    <!-- Inference Steps Control -->
    <div class="control-group">
      <div class="control-header">
        <label for="inference-slider" class="control-label">
          <span class="label-text">Inference Steps</span>
          <span class="label-value">{inferenceSteps}</span>
        </label>
        <div class="quality-indicator" style="--indicator-color: {getQualityIndicator(inferenceSteps).color}">
          <span class="quality-label">{getQualityIndicator(inferenceSteps).label}</span>
          <span class="quality-desc">{getQualityIndicator(inferenceSteps).description}</span>
        </div>
      </div>
      
      <div class="slider-container">
        <input
          id="inference-slider"
          type="range"
          min={inferenceConfig.min}
          max={inferenceConfig.max}
          step={inferenceConfig.step}
          bind:value={inferenceSteps}
          on:input={(e) => {
            const target = e.target as HTMLInputElement;
            updateInferenceSteps(parseInt(target.value));
          }}
          class="slider"
        />
        <div class="slider-track">
          <div class="slider-fill" style="width: {((inferenceSteps - inferenceConfig.min) / (inferenceConfig.max - inferenceConfig.min)) * 100}%"></div>
        </div>
        <div class="slider-thumb" style="left: {((inferenceSteps - inferenceConfig.min) / (inferenceConfig.max - inferenceConfig.min)) * 100}%"></div>
      </div>
      
      <div class="slider-labels">
        <span class="slider-min">{inferenceConfig.min}</span>
        <span class="slider-max">{inferenceConfig.max}</span>
      </div>
    </div>

    <!-- Guidance Scale Control -->
    <div class="control-group">
      <div class="control-header">
        <label for="guidance-slider" class="control-label">
          <span class="label-text">Guidance Scale (CFG)</span>
          <span class="label-value">{guidanceScale}</span>
        </label>
        <div class="quality-indicator" style="--indicator-color: {getGuidanceIndicator(guidanceScale).color}">
          <span class="quality-label">{getGuidanceIndicator(guidanceScale).label}</span>
          <span class="quality-desc">{getGuidanceIndicator(guidanceScale).description}</span>
        </div>
      </div>
      
      <div class="slider-container">
        <input
          id="guidance-slider"
          type="range"
          min={guidanceConfig.min}
          max={guidanceConfig.max}
          step={guidanceConfig.step}
          bind:value={guidanceScale}
          on:input={(e) => {
            const target = e.target as HTMLInputElement;
            updateGuidanceScale(parseFloat(target.value));
          }}
          class="slider"
        />
        <div class="slider-track">
          <div class="slider-fill" style="width: {((guidanceScale - guidanceConfig.min) / (guidanceConfig.max - guidanceConfig.min)) * 100}%"></div>
        </div>
        <div class="slider-thumb" style="left: {((guidanceScale - guidanceConfig.min) / (guidanceConfig.max - guidanceConfig.min)) * 100}%"></div>
      </div>
      
      <div class="slider-labels">
        <span class="slider-min">{guidanceConfig.min}</span>
        <span class="slider-max">{guidanceConfig.max}</span>
      </div>
    </div>

    <!-- Parameter Summary -->
    <div class="parameter-summary">
      <div class="summary-item">
        <span class="summary-label">Generation Quality:</span>
        <span class="summary-value" style="color: {getQualityIndicator(inferenceSteps).color}">
          {getQualityIndicator(inferenceSteps).label}
        </span>
      </div>
      <div class="summary-item">
        <span class="summary-label">Prompt Adherence:</span>
        <span class="summary-value" style="color: {getGuidanceIndicator(guidanceScale).color}">
          {getGuidanceIndicator(guidanceScale).label}
        </span>
      </div>
      <div class="summary-item">
        <span class="summary-label">Estimated Time:</span>
        <span class="summary-value">
          {Math.round(inferenceSteps * 0.5)}s
        </span>
      </div>
    </div>
  </div>
</div>

<style>
  .advanced-controls {
    padding: var(--spacing-xl);
    max-width: 600px;
    margin: 0 auto;
  }

  .controls-header {
    text-align: center;
    margin-bottom: var(--spacing-xl);
    position: relative;
  }

  .controls-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .controls-subtitle {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0 0 var(--spacing-lg) 0;
  }

  .reset-btn {
    position: absolute;
    top: 0;
    right: 0;
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-xs);
  }

  .reset-btn svg {
    width: 14px;
    height: 14px;
  }

  .controls-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .control-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--spacing-md);
  }

  .control-label {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    flex: 1;
  }

  .label-text {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .label-value {
    font-size: var(--font-size-lg);
    font-weight: 700;
    color: var(--accent-primary);
  }

  .quality-indicator {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--spacing-xs);
    padding: var(--spacing-sm) var(--spacing-md);
    background: rgba(var(--indicator-color), 0.1);
    border: 1px solid rgba(var(--indicator-color), 0.2);
    border-radius: var(--radius-md);
    min-width: 120px;
  }

  .quality-label {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--indicator-color);
  }

  .quality-desc {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    text-align: right;
    line-height: 1.3;
  }

  .slider-container {
    position: relative;
    height: 40px;
    display: flex;
    align-items: center;
  }

  .slider {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 2;
  }

  .slider-track {
    position: absolute;
    width: 100%;
    height: 6px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .slider-fill {
    height: 100%;
    background: var(--accent-gradient);
    border-radius: var(--radius-sm);
    transition: width 0.1s ease;
  }

  .slider-thumb {
    position: absolute;
    width: 20px;
    height: 20px;
    background: var(--accent-primary);
    border: 3px solid var(--bg-primary);
    border-radius: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    box-shadow: var(--shadow-glow);
    transition: left 0.1s ease;
    z-index: 1;
  }

  .slider-thumb:hover {
    transform: translate(-50%, -50%) scale(1.1);
    box-shadow: 0 0 20px rgba(100, 255, 218, 0.4);
  }

  .slider-labels {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin-top: var(--spacing-xs);
  }

  .parameter-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
    background: var(--bg-tertiary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
    margin-top: var(--spacing-lg);
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    text-align: center;
  }

  .summary-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .summary-value {
    font-size: var(--font-size-md);
    font-weight: 600;
    color: var(--text-primary);
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .advanced-controls {
      padding: var(--spacing-lg);
      margin: 0 var(--spacing-md);
    }

    .control-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--spacing-sm);
    }

    .quality-indicator {
      align-self: stretch;
      align-items: center;
      text-align: center;
    }

    .parameter-summary {
      grid-template-columns: 1fr;
      gap: var(--spacing-sm);
    }

    .reset-btn {
      position: static;
      align-self: center;
      margin-top: var(--spacing-md);
    }
  }
</style> 