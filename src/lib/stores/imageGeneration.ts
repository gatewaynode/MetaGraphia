import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Types
export interface GenerationParams {
    prompt: string;
    img_width: number;
    img_height: number;
    num_imgs: number;
    num_inference_steps: number;
    guidance_scale: number;
}

export interface GenerationProgress {
    current_step: number;
    total_steps: number;
    status: string;
}

export interface GenerationResult {
    generated_img_path: string;
    aux_output_image_path?: string;
}

export interface GenerationState {
    params: GenerationParams;
    isGenerating: boolean;
    progress: GenerationProgress;
    currentResult: GenerationResult | null;
    error: string | null;
    history: GenerationResult[];
}

export interface AppSettings {
  default_width: number;
  default_height: number;
  default_inference_steps: number;
  default_guidance_scale: number;
  output_directory: string;
  model_path: string;
}

// Default generation parameters
const defaultParams: GenerationParams = {
    prompt: '',
    img_width: 512,
    img_height: 512,
    num_imgs: 1,
    num_inference_steps: 20,
    guidance_scale: 7.5
};

// Create the main generation store
export const generationStore: Writable<GenerationState> = writable({
    // Current generation parameters
    params: { ...defaultParams },
    
    // Generation state
    isGenerating: false,
      progress: {
    current_step: 0,
    total_steps: 20,
    status: 'Ready',
    is_complete: false,
    is_cancelled: false
  },
    
    // Results
    currentResult: null,
    error: null,
    
    // History
    history: []
});

// Settings store
export const settingsStore: Writable<AppSettings> = writable({
    default_width: 512,
    default_height: 512,
    default_inference_steps: 20,
    default_guidance_scale: 7.5,
    output_directory: '',
    model_path: ''
});

// Actions
export const generationActions = {
    // Update generation parameters
    updateParams: (newParams: Partial<GenerationParams>) => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            params: { ...store.params, ...newParams }
        }));
    },
    
    // Update specific dimension
    updateDimension: (dimension: 'width' | 'height', value: number) => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            params: {
                ...store.params,
                [`img_${dimension}`]: value
            }
        }));
    },
    
    // Start generation
    startGeneration: () => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            isGenerating: true,
            error: null,
            progress: {
                current_step: 0,
                total_steps: store.params.num_inference_steps,
                status: 'Initializing...'
            }
        }));
    },
    
    // Update progress
    updateProgress: (progress: GenerationProgress) => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            progress: { ...progress }
        }));
    },
    
    // Set result
    setResult: (result: GenerationResult) => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            isGenerating: false,
            currentResult: result,
            history: [result, ...store.history].slice(0, 50) // Keep last 50 images
        }));
    },
    
    // Set error
    setError: (error: string) => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            isGenerating: false,
            error: error
        }));
    },
    
    // Reset to default
    reset: () => {
        generationStore.update((store: GenerationState) => ({
            ...store,
            params: { ...defaultParams },
            isGenerating: false,
            error: null,
            progress: {
                current_step: 0,
                total_steps: 20,
                status: 'Ready'
            }
        }));
    }
};

// Settings actions
export const settingsActions = {
    // Load settings from backend
    loadSettings: async () => {
        try {
            const settings = await invoke<AppSettings>('get_settings');
            settingsStore.set(settings);
            
            // Also update the generation params with the loaded settings
            generationStore.update((store: GenerationState) => ({
                ...store,
                params: {
                    ...store.params,
                    img_width: settings.default_width,
                    img_height: settings.default_height,
                    num_inference_steps: settings.default_inference_steps,
                    guidance_scale: settings.default_guidance_scale
                }
            }));
        } catch (error) {
            console.error('Failed to load settings:', error);
        }
    },
    
    // Save settings to backend
    saveSettings: async (settings: AppSettings) => {
        try {
            await invoke('save_settings', { settings });
            settingsStore.set(settings);
        } catch (error) {
            console.error('Failed to save settings:', error);
            throw error;
        }
    },
    
    // Update a specific setting
    updateSetting: async (key: keyof AppSettings, value: number) => {
        settingsStore.update((settings: AppSettings) => {
            const newSettings = { ...settings, [key]: value };
            
            // Save to backend
            settingsActions.saveSettings(newSettings).catch(console.error);
            
            return newSettings;
        });
    },
    
    // Reset to defaults
    resetToDefaults: async () => {
        try {
            const defaultSettings: AppSettings = {
                default_width: 512,
                default_height: 512,
                default_inference_steps: 20,
                default_guidance_scale: 7.5,
                output_directory: '',
                model_path: ''
            };
            
            await invoke('save_settings', { settings: defaultSettings });
            settingsStore.set(defaultSettings);
        } catch (error) {
            console.error('Failed to reset settings:', error);
            throw error;
        }
    }
}; 