import { writable, type Writable } from 'svelte/store';

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
        status: 'Ready'
    },
    
    // Results
    currentResult: null,
    error: null,
    
    // History
    history: []
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