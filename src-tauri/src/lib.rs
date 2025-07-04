use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use anyhow::{Result, Context};

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum DiffusionError {
    #[error("Python backend error: {0}")]
    PythonBackend(String),
    #[error("File system error: {0}")]
    FileSystem(String),
    #[error("JSON serialization error: {0}")]
    JsonError(String),
    #[error("Process error: {0}")]
    ProcessError(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

// Data structures for image generation
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub img_width: u32,
    pub img_height: u32,
    pub num_imgs: u32,
    pub num_inference_steps: u32,
    pub guidance_scale: f32,
}

impl ImageGenerationRequest {
    pub fn validate(&self) -> Result<(), String> {
        // Validate prompt
        if self.prompt.trim().is_empty() {
            return Err("Prompt cannot be empty".to_string());
        }

        // Validate dimensions
        if self.img_width < 256 || self.img_width > 1024 {
            return Err("Width must be between 256 and 1024".to_string());
        }
        if self.img_height < 256 || self.img_height > 1024 {
            return Err("Height must be between 256 and 1024".to_string());
        }

        // Validate other parameters
        if self.num_imgs == 0 || self.num_imgs > 10 {
            return Err("Number of images must be between 1 and 10".to_string());
        }
        if self.num_inference_steps < 10 || self.num_inference_steps > 50 {
            return Err("Inference steps must be between 10 and 50".to_string());
        }
        if self.guidance_scale < 1.0 || self.guidance_scale > 20.0 {
            return Err("Guidance scale must be between 1.0 and 20.0".to_string());
        }

        Ok(())
    }

    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            img_width: 512,
            img_height: 512,
            num_imgs: 1,
            num_inference_steps: 20,
            guidance_scale: 7.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    pub generated_img_path: String,
    pub aux_output_image_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub current_step: u32,
    pub total_steps: u32,
    pub status: String,
    pub is_complete: bool,
    pub is_cancelled: bool,
}

// Settings structure for persistence
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_width: u32,
    pub default_height: u32,
    pub default_inference_steps: u32,
    pub default_guidance_scale: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_width: 512,
            default_height: 512,
            default_inference_steps: 20,
            default_guidance_scale: 7.5,
        }
    }
}

// Python backend manager
#[derive(Clone)]
struct PythonBackend {
    backend_path: PathBuf,
}

impl PythonBackend {
    fn new() -> Result<Self> {
        // Get the backend path relative to the project root
        // In development, we're in the src-tauri directory, so go up to project root
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        let project_root = current_dir
            .parent()
            .context("Failed to get project root")?
            .parent()
            .context("Failed to get workspace root")?;
        
        let backend_path = project_root
            .join("backends")
            .join("stable_diffusion")
            .join("diffusionbee_backend.py");

        Ok(Self {
            backend_path,
        })
    }

    fn start_backend(&self) -> Result<()> {
        // TODO: Implement proper backend startup
        // For now, just validate the backend path exists
        if !self.backend_path.exists() {
            return Err(anyhow::anyhow!("Python backend not found at {:?}", self.backend_path));
        }
        Ok(())
    }
}

// Global backend instance with proper synchronization
static BACKEND: Mutex<Option<PythonBackend>> = Mutex::new(None);

// Global generation state
static GENERATION_PROGRESS: Mutex<Option<GenerationProgress>> = Mutex::new(None);
static GENERATION_CANCELLED: Mutex<bool> = Mutex::new(false);

fn get_backend() -> Result<PythonBackend> {
    let mut backend_guard = BACKEND.lock()
        .map_err(|_| anyhow::anyhow!("Failed to acquire backend lock"))?;
    
    if backend_guard.is_none() {
        *backend_guard = Some(PythonBackend::new()?);
    }
    
    // Clone the backend to avoid holding the lock
    Ok(backend_guard.as_ref().unwrap().clone())
}

// Tauri commands
#[tauri::command]
async fn generate_image(request: ImageGenerationRequest) -> Result<ImageGenerationResponse, String> {
    // Validate the request first
    request.validate()?;

    let backend = get_backend().map_err(|e| e.to_string())?;
    
    // Validate the backend is available
    backend.start_backend().map_err(|e| e.to_string())?;

    // Reset progress and cancellation state
    {
        let mut progress = GENERATION_PROGRESS.lock()
            .map_err(|_| "Failed to acquire progress lock".to_string())?;
        *progress = Some(GenerationProgress {
            current_step: 0,
            total_steps: request.num_inference_steps,
            status: "Initializing...".to_string(),
            is_complete: false,
            is_cancelled: false,
        });
        
        let mut cancelled = GENERATION_CANCELLED.lock()
            .map_err(|_| "Failed to acquire cancellation lock".to_string())?;
        *cancelled = false;
    }

    // Simulate image generation with progress updates
    for step in 0..request.num_inference_steps {
        // Check if generation was cancelled
        {
            let cancelled = GENERATION_CANCELLED.lock()
                .map_err(|_| "Failed to acquire cancellation lock".to_string())?;
            if *cancelled {
                // Update progress to show cancellation
                if let Some(ref mut progress) = *GENERATION_PROGRESS.lock()
                    .map_err(|_| "Failed to acquire progress lock".to_string())? {
                    progress.is_cancelled = true;
                    progress.status = "Cancelled".to_string();
                }
                return Err("Generation cancelled by user".to_string());
            }
        }

        // Update progress
        {
            let mut progress = GENERATION_PROGRESS.lock()
                .map_err(|_| "Failed to acquire progress lock".to_string())?;
            if let Some(ref mut prog) = *progress {
                prog.current_step = step + 1;
                prog.status = match step {
                    s if s < request.num_inference_steps / 4 => "Initializing...".to_string(),
                    s if s < request.num_inference_steps / 2 => "Building image...".to_string(),
                    s if s < 3 * request.num_inference_steps / 4 => "Refining details...".to_string(),
                    _ => "Finalizing...".to_string(),
                };
            }
        }

        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Mark as complete
    {
        let mut progress = GENERATION_PROGRESS.lock()
            .map_err(|_| "Failed to acquire progress lock".to_string())?;
        if let Some(ref mut prog) = *progress {
            prog.is_complete = true;
            prog.status = "Complete".to_string();
        }
    }

    // Prepare the JSON request (for future IPC implementation)
    let _json_request = serde_json::to_string(&request)
        .map_err(|e| e.to_string())?;

    // Send the request to the Python backend
    // TODO: Implement proper IPC communication with Python backend
    // For now, we'll use a simple approach - in production, you'd want proper IPC
    // This is a placeholder implementation
    let response = ImageGenerationResponse {
        generated_img_path: "/tmp/placeholder.png".to_string(),
        aux_output_image_path: None,
    };

    Ok(response)
}

#[tauri::command]
async fn get_generation_progress() -> Result<GenerationProgress, String> {
    let progress = GENERATION_PROGRESS.lock()
        .map_err(|_| "Failed to acquire progress lock".to_string())?;
    
    match progress.as_ref() {
        Some(prog) => Ok(prog.clone()),
        None => Ok(GenerationProgress {
            current_step: 0,
            total_steps: 0,
            status: "No generation in progress".to_string(),
            is_complete: false,
            is_cancelled: false,
        }),
    }
}

#[tauri::command]
async fn cancel_generation() -> Result<(), String> {
    let mut cancelled = GENERATION_CANCELLED.lock()
        .map_err(|_| "Failed to acquire cancellation lock".to_string())?;
    *cancelled = true;
    
    // Update progress to show cancellation
    if let Some(ref mut progress) = *GENERATION_PROGRESS.lock()
        .map_err(|_| "Failed to acquire progress lock".to_string())? {
        progress.is_cancelled = true;
        progress.status = "Cancelling...".to_string();
    }
    
    Ok(())
}

#[tauri::command]
async fn get_models() -> Result<Vec<String>, String> {
    // Placeholder implementation
    Ok(vec!["stable-diffusion-v1-5".to_string()])
}

#[tauri::command]
async fn set_active_model(_model_name: String) -> Result<(), String> {
    // Placeholder implementation
    Ok(())
}

// Settings commands
#[tauri::command]
async fn get_settings() -> Result<AppSettings, String> {
    // For now, return default settings
    // In the future, this would load from persistent storage
    Ok(AppSettings::default())
}

#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), String> {
    // For now, just validate the settings
    // In the future, this would save to persistent storage
    if settings.default_width < 256 || settings.default_width > 1024 {
        return Err("Default width must be between 256 and 1024".to_string());
    }
    if settings.default_height < 256 || settings.default_height > 1024 {
        return Err("Default height must be between 256 and 1024".to_string());
    }
    if settings.default_inference_steps < 10 || settings.default_inference_steps > 50 {
        return Err("Default inference steps must be between 10 and 50".to_string());
    }
    if settings.default_guidance_scale < 1.0 || settings.default_guidance_scale > 20.0 {
        return Err("Default guidance scale must be between 1.0 and 20.0".to_string());
    }
    
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_image,
            get_generation_progress,
            cancel_generation,
            get_models,
            set_active_model,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
