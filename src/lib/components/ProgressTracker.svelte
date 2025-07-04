<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { generationStore, generationActions } from '../stores/imageGeneration';

  // Subscribe to store values
  $: ({ isGenerating, progress } = $generationStore);
  $: progressPercentage = progress.total_steps > 0 ? (progress.current_step / progress.total_steps) * 100 : 0;

  let progressInterval: number | null = null;
  let startTime: number | null = null;
  let estimatedTimeRemaining: string = '';

  // Start progress polling when generation begins
  $: if (isGenerating && !progressInterval) {
    startProgressPolling();
  } else if (!isGenerating && progressInterval) {
    stopProgressPolling();
  }

  function startProgressPolling() {
    startTime = Date.now();
    progressInterval = window.setInterval(async () => {
      try {
        const progressData = await invoke('get_generation_progress') as any;
        generationActions.updateProgress(progressData);
        updateTimeEstimate();
      } catch (error) {
        console.error('Failed to get progress:', error);
      }
    }, 500); // Poll every 500ms
  }

  function stopProgressPolling() {
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = null;
    }
    startTime = null;
    estimatedTimeRemaining = '';
  }

  function updateTimeEstimate() {
    if (!startTime || progress.current_step === 0) {
      estimatedTimeRemaining = '';
      return;
    }

    const elapsed = Date.now() - startTime;
    const stepsCompleted = progress.current_step;
    const stepsRemaining = progress.total_steps - progress.current_step;
    
    if (stepsCompleted > 0) {
      const timePerStep = elapsed / stepsCompleted;
      const estimatedRemaining = timePerStep * stepsRemaining;
      estimatedTimeRemaining = formatTime(estimatedRemaining);
    }
  }

  function formatTime(milliseconds: number): string {
    const seconds = Math.ceil(milliseconds / 1000);
    if (seconds < 60) {
      return `${seconds}s`;
    } else if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = seconds % 60;
      return `${minutes}m ${remainingSeconds}s`;
    } else {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return `${hours}h ${minutes}m`;
    }
  }

  async function cancelGeneration() {
    try {
      await invoke('cancel_generation');
      generationActions.setError('Generation cancelled by user');
    } catch (error) {
      console.error('Failed to cancel generation:', error);
      generationActions.setError('Failed to cancel generation');
    }
  }

  onDestroy(() => {
    stopProgressPolling();
  });

  // Get progress status color based on completion
  function getProgressColor(): string {
    if (progressPercentage >= 90) return 'var(--accent-primary)';
    if (progressPercentage >= 70) return '#00d4aa';
    if (progressPercentage >= 40) return '#ffa726';
    return '#ff6b6b';
  }

  // Get progress status text
  function getProgressStatus(): string {
    if (progressPercentage >= 90) return 'Finalizing...';
    if (progressPercentage >= 70) return 'Refining details...';
    if (progressPercentage >= 40) return 'Building image...';
    if (progressPercentage >= 10) return 'Initializing...';
    return 'Starting...';
  }
</script>

{#if isGenerating}
  <div class="progress-tracker card glass fade-in">
    <div class="progress-header">
      <h3 class="progress-title">Generating Image</h3>
      <button class="cancel-btn btn btn-danger" on:click={cancelGeneration}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
        Cancel
      </button>
    </div>

    <div class="progress-content">
      <!-- Progress Bar -->
      <div class="progress-bar-container">
        <div class="progress-bar">
          <div 
            class="progress-fill" 
            style="width: {progressPercentage}%; background: {getProgressColor()}"
          ></div>
        </div>
        <div class="progress-labels">
          <span class="progress-text">{getProgressStatus()}</span>
          <span class="progress-percentage">{Math.round(progressPercentage)}%</span>
        </div>
      </div>

      <!-- Progress Details -->
      <div class="progress-details">
        <div class="detail-item">
          <span class="detail-label">Step:</span>
          <span class="detail-value">{progress.current_step} / {progress.total_steps}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Status:</span>
          <span class="detail-value">{progress.status}</span>
        </div>
        {#if estimatedTimeRemaining}
          <div class="detail-item">
            <span class="detail-label">Time Remaining:</span>
            <span class="detail-value text-accent">{estimatedTimeRemaining}</span>
          </div>
        {/if}
      </div>

      <!-- Progress Animation -->
      <div class="progress-animation">
        <div class="animation-dots">
          <div class="dot"></div>
          <div class="dot"></div>
          <div class="dot"></div>
        </div>
        <span class="animation-text">Processing...</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .progress-tracker {
    padding: var(--spacing-xl);
    max-width: 600px;
    margin: 0 auto;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-lg);
  }

  .progress-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .cancel-btn {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
  }

  .cancel-btn svg {
    width: 14px;
    height: 14px;
  }

  .progress-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .progress-bar-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .progress-bar {
    width: 100%;
    height: 12px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-lg);
    overflow: hidden;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    border-radius: var(--radius-lg);
    transition: width 0.3s ease, background-color 0.3s ease;
    position: relative;
    overflow: hidden;
  }

  .progress-fill::after {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% { left: -100%; }
    100% { left: 100%; }
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .progress-text {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .progress-percentage {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .progress-details {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
    background: var(--bg-tertiary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .detail-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detail-value {
    font-size: var(--font-size-md);
    font-weight: 600;
    color: var(--text-primary);
  }

  .progress-animation {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
  }

  .animation-dots {
    display: flex;
    gap: var(--spacing-xs);
  }

  .dot {
    width: 8px;
    height: 8px;
    background: var(--accent-primary);
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  .dot:nth-child(2) {
    animation-delay: 0.2s;
  }

  .dot:nth-child(3) {
    animation-delay: 0.4s;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 0.3;
      transform: scale(0.8);
    }
    50% {
      opacity: 1;
      transform: scale(1.2);
    }
  }

  .animation-text {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    font-weight: 500;
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .progress-tracker {
      padding: var(--spacing-lg);
      margin: 0 var(--spacing-md);
    }

    .progress-header {
      flex-direction: column;
      gap: var(--spacing-md);
      text-align: center;
    }

    .progress-details {
      grid-template-columns: 1fr;
      gap: var(--spacing-sm);
    }

    .cancel-btn {
      align-self: stretch;
    }
  }
</style> 