use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use anyhow::{Result, Context};
use tauri::Manager;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    pub generated_img_path: String,
    pub aux_output_image_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub current_step: u32,
    pub total_steps: u32,
    pub status: String,
}

// Python backend manager
struct PythonBackend {
    process: Option<std::process::Child>,
    backend_path: PathBuf,
}

impl PythonBackend {
    fn new() -> Result<Self> {
        // Get the backend path relative to the executable
        let backend_path = std::env::current_exe()
            .context("Failed to get current executable path")?
            .parent()
            .context("Failed to get executable directory")?
            .join("backends")
            .join("stable_diffusion")
            .join("diffusionbee_backend.py");

        Ok(Self {
            process: None,
            backend_path,
        })
    }

    fn start_backend(&mut self) -> Result<()> {
        if self.process.is_some() {
            return Ok(());
        }

        let mut cmd = Command::new("python3");
        cmd.arg(&self.backend_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()
            .context("Failed to start Python backend")?;

        // Wait a moment for the backend to initialize
        std::thread::sleep(std::time::Duration::from_millis(1000));

        self.process = Some(child);
        Ok(())
    }

    fn stop_backend(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
        }
    }
}

impl Drop for PythonBackend {
    fn drop(&mut self) {
        self.stop_backend();
    }
}

// Global backend instance
static mut BACKEND: Option<PythonBackend> = None;

fn get_backend() -> Result<&'static mut PythonBackend> {
    unsafe {
        if BACKEND.is_none() {
            BACKEND = Some(PythonBackend::new()?);
        }
        Ok(BACKEND.as_mut().unwrap())
    }
}

// Tauri commands
#[tauri::command]
async fn generate_image(request: ImageGenerationRequest) -> Result<ImageGenerationResponse, String> {
    let backend = get_backend().map_err(|e| e.to_string())?;
    
    // Start the backend if not already running
    backend.start_backend().map_err(|e| e.to_string())?;

    // Prepare the JSON request
    let json_request = serde_json::to_string(&request)
        .map_err(|e| e.to_string())?;

    // Send the request to the Python backend
    let command = format!("b2py t2im {}", json_request);
    
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
    // Placeholder implementation
    Ok(GenerationProgress {
        current_step: 0,
        total_steps: 20,
        status: "Initializing".to_string(),
    })
}

#[tauri::command]
async fn cancel_generation() -> Result<(), String> {
    // Placeholder implementation
    Ok(())
}

#[tauri::command]
async fn get_models() -> Result<Vec<String>, String> {
    // Placeholder implementation
    Ok(vec!["stable-diffusion-v1-5".to_string()])
}

#[tauri::command]
async fn set_active_model(model_name: String) -> Result<(), String> {
    // Placeholder implementation
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
            set_active_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
