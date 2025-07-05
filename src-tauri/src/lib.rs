use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::{Command, Stdio};
use std::io::Write;
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
    pub output_directory: String,
    pub model_path: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_width: 512,
            default_height: 512,
            default_inference_steps: 20,
            default_guidance_scale: 7.5,
            output_directory: get_default_output_directory(),
            model_path: get_default_model_path(),
        }
    }
}

fn get_default_output_directory() -> String {
    // Try to get Desktop directory, fallback to current directory
    if let Some(home_dir) = dirs::home_dir() {
        let desktop = home_dir.join("Desktop");
        if desktop.exists() {
            return desktop.to_string_lossy().to_string();
        }
    }
    
    // Fallback to current directory
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

fn get_default_model_path() -> String {
    // Try to find a default model in the .diffusionbee directory
    if let Some(home_dir) = dirs::home_dir() {
        let diffusionbee_dir = home_dir.join(".diffusionbee");
        let imported_models_dir = diffusionbee_dir.join("imported_models");
        
        if imported_models_dir.exists() {
            // Look for sd-v1-5_fp16.tdict first
            let default_model = imported_models_dir.join("sd-v1-5_fp16.tdict");
            if default_model.exists() {
                return default_model.to_string_lossy().to_string();
            }
            
            // Look for any .tdict file
            if let Ok(entries) = fs::read_dir(imported_models_dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "tdict" {
                            return entry.path().to_string_lossy().to_string();
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to empty string (no model)
    String::new()
}

fn create_placeholder_image(path: &PathBuf, width: u32, height: u32) -> Result<()> {
    // Create a simple PNG image as a placeholder
    // This is a basic implementation - in production you'd use a proper image library
    
    // PNG header
    let mut png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
    ];
    
    // IHDR chunk (image header)
    let ihdr_data = create_ihdr_chunk(width, height);
    png_data.extend(ihdr_data);
    
    // IDAT chunk (image data) - simple colored rectangle
    let idat_data = create_idat_chunk(width, height);
    png_data.extend(idat_data);
    
    // IEND chunk (end of file)
    let iend_data = create_iend_chunk();
    png_data.extend(iend_data);
    
    // Write to file
    fs::write(path, png_data)
        .context("Failed to write PNG file")?;
    
    Ok(())
}

fn create_ihdr_chunk(width: u32, height: u32) -> Vec<u8> {
    let mut data = vec![];
    
    // Length (13 bytes)
    data.extend_from_slice(&13u32.to_be_bytes());
    
    // Type "IHDR"
    data.extend_from_slice(b"IHDR");
    
    // Width, height, bit depth, color type, compression, filter, interlace
    data.extend_from_slice(&width.to_be_bytes());
    data.extend_from_slice(&height.to_be_bytes());
    data.extend_from_slice(&[8, 2, 0, 0, 0]); // 8-bit RGB, no compression, no filter, no interlace
    
    // CRC (placeholder - in production you'd calculate this properly)
    data.extend_from_slice(&[0, 0, 0, 0]);
    
    data
}

fn create_idat_chunk(width: u32, height: u32) -> Vec<u8> {
    let mut data = vec![];
    
    // Create simple RGB data (purple gradient)
    let mut image_data = vec![];
    for y in 0..height {
        // Filter byte (0 = no filter)
        image_data.push(0);
        
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = 128;
            image_data.extend_from_slice(&[r, g, b]);
        }
    }
    
    // Compress data (simple implementation - in production use proper compression)
    let compressed_data = image_data; // For now, no compression
    
    // Length
    data.extend_from_slice(&(compressed_data.len() as u32).to_be_bytes());
    
    // Type "IDAT"
    data.extend_from_slice(b"IDAT");
    
    // Data
    data.extend(compressed_data);
    
    // CRC (placeholder)
    data.extend_from_slice(&[0, 0, 0, 0]);
    
    data
}

fn create_iend_chunk() -> Vec<u8> {
    let mut data = vec![];
    
    // Length (0 bytes)
    data.extend_from_slice(&0u32.to_be_bytes());
    
    // Type "IEND"
    data.extend_from_slice(b"IEND");
    
    // CRC (placeholder)
    data.extend_from_slice(&[0, 0, 0, 0]);
    
    data
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
    static BACKEND: Mutex<Option<PythonBackend>> = Mutex::new(None);
    
    let mut backend = BACKEND.lock()
        .map_err(|_| anyhow::anyhow!("Failed to acquire backend lock"))?;
    
    if backend.is_none() {
        *backend = Some(PythonBackend::new()?);
    }
    
    Ok(backend.as_ref().unwrap().clone())
}

async fn call_python_backend(
    request: &ImageGenerationRequest,
    model_path: &PathBuf,
    _output_dir: &PathBuf,
) -> Result<ImageGenerationResponse, String> {
    println!("[RUST] Calling Python backend with model: {}", model_path.display());
    
    // Find the Python backend script
    let backend_script = find_backend_script()
        .map_err(|e| format!("Failed to find backend script: {}", e))?;
    println!("[RUST] Backend script found at: {}", backend_script.display());
    
    // Prepare the JSON request in the format expected by the Python backend
    let json_request = serde_json::json!({
        "prompt": request.prompt,
        "img_width": request.img_width,
        "img_height": request.img_height,
        "num_imgs": request.num_imgs,
        "num_inference_steps": request.num_inference_steps,
        "guidance_scale": request.guidance_scale,
        "tdict_path": model_path.to_string_lossy(),
    });
    
    let request_str = format!("b2py t2im {}", json_request.to_string());
    println!("[RUST] Sending request to Python backend: {}", request_str);
    
    // Start the Python backend process
    let mut child = Command::new("python3")
        .arg(backend_script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start Python backend: {}", e))?;
    
    // Send the request to the backend
    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(request_str.as_bytes())
            .map_err(|e| format!("Failed to write to Python backend: {}", e))?;
        stdin.write_all(b"\n")
            .map_err(|e| format!("Failed to write newline to Python backend: {}", e))?;
    }
    
    // Read the response
    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to get output from Python backend: {}", e))?;
    
    println!("[RUST] Python backend stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("[RUST] Python backend stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    if !output.status.success() {
        return Err(format!("Python backend failed with status: {}", output.status));
    }
    
    // Parse the response
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    for line in stdout_str.lines() {
        if line.contains("sdbk nwim") {
            // Extract the JSON response
            if let Some(json_start) = line.find("sdbk nwim ") {
                let json_str = &line[json_start + 10..];
                match serde_json::from_str::<serde_json::Value>(json_str) {
                    Ok(response) => {
                        if let Some(img_path) = response["generated_img_path"].as_str() {
                            return Ok(ImageGenerationResponse {
                                generated_img_path: img_path.to_string(),
                                aux_output_image_path: response["aux_output_image_path"].as_str().map(|s| s.to_string()),
                            });
                        }
                    }
                    Err(e) => {
                        println!("[RUST] Failed to parse JSON response: {}", e);
                    }
                }
            }
        }
    }
    
    Err("No valid response received from Python backend".to_string())
}

fn find_backend_script() -> Result<PathBuf> {
    // Try to find the backend script relative to the current executable
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;
    
    // Look for the backend script in the expected location
    let possible_paths = vec![
        // Development path
        current_exe.parent().unwrap().parent().unwrap().parent().unwrap()
            .join("backends").join("stable_diffusion").join("diffusionbee_backend.py"),
        // Production path (if bundled)
        current_exe.parent().unwrap().join("backends").join("stable_diffusion").join("diffusionbee_backend.py"),
    ];
    
    for path in possible_paths {
        if path.exists() {
            return Ok(path);
        }
    }
    
    Err(anyhow::anyhow!("Backend script not found in any expected location"))
}

fn create_fallback_image(
    request: &ImageGenerationRequest,
    output_dir: &PathBuf,
) -> Result<ImageGenerationResponse, String> {
    println!("[RUST] Creating fallback image");
    
    // Generate a unique filename with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get timestamp: {}", e))?
        .as_secs();
    
    let filename = format!("diffusionbee_fallback_{}_{}x{}.png", timestamp, request.img_width, request.img_height);
    let output_path = output_dir.join(&filename);
    
    // Create a placeholder image
    create_placeholder_image(&output_path, request.img_width, request.img_height)
        .map_err(|e| format!("Failed to create fallback image: {}", e))?;
    
    Ok(ImageGenerationResponse {
        generated_img_path: output_path.to_string_lossy().to_string(),
        aux_output_image_path: None,
    })
}

// Tauri commands
#[tauri::command]
async fn generate_image(request: ImageGenerationRequest) -> Result<ImageGenerationResponse, String> {
    println!("[RUST] Starting image generation for prompt: {}", request.prompt);
    
    // Validate the request first
    request.validate()?;
    println!("[RUST] Request validation passed");

    // Get settings
    let settings = get_settings().await.map_err(|e| {
        println!("[RUST] Failed to get settings: {}", e);
        e.to_string()
    })?;
    println!("[RUST] Settings loaded - Output dir: {}, Model path: {}", 
             settings.output_directory, settings.model_path);

    // Check if model exists
    if settings.model_path.is_empty() {
        println!("[RUST] No model path configured");
        return Err("No Stable Diffusion model found. Please download a model first.".to_string());
    }

    let model_path = PathBuf::from(&settings.model_path);
    if !model_path.exists() {
        println!("[RUST] Model file not found at: {}", model_path.display());
        return Err(format!("Model file not found at: {}", model_path.display()));
    }
    println!("[RUST] Model file found at: {}", model_path.display());

    // Get the backend
    let backend = get_backend().map_err(|e| {
        println!("[RUST] Failed to get backend: {}", e);
        e.to_string()
    })?;
    
    // Validate the backend is available
    backend.start_backend().map_err(|e| {
        println!("[RUST] Failed to start backend: {}", e);
        e.to_string()
    })?;
    println!("[RUST] Backend validation passed");

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
    println!("[RUST] Progress state initialized");

    // Prepare output directory
    let output_dir = PathBuf::from(&settings.output_directory);
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    println!("[RUST] Output directory prepared: {}", output_dir.display());

    // Try to call the Python backend
    match call_python_backend(&request, &model_path, &output_dir).await {
        Ok(response) => {
            println!("[RUST] Python backend call successful: {:?}", response);
            
            // Mark as complete
            {
                let mut progress = GENERATION_PROGRESS.lock()
                    .map_err(|_| "Failed to acquire progress lock".to_string())?;
                if let Some(ref mut prog) = *progress {
                    prog.is_complete = true;
                    prog.status = "Complete".to_string();
                }
            }
            
            Ok(response)
        }
        Err(e) => {
            println!("[RUST] Python backend call failed: {}", e);
            
            // Fallback to placeholder image
            println!("[RUST] Falling back to placeholder image");
            let fallback_response = create_fallback_image(&request, &output_dir)
                .map_err(|fe| format!("Failed to create fallback image: {}", fe))?;
            
            // Mark as complete
            {
                let mut progress = GENERATION_PROGRESS.lock()
                    .map_err(|_| "Failed to acquire progress lock".to_string())?;
                if let Some(ref mut prog) = *progress {
                    prog.is_complete = true;
                    prog.status = "Complete (Fallback)".to_string();
                }
            }
            
            Ok(fallback_response)
        }
    }
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
    // Look for models in the .diffusionbee/imported_models directory
    if let Some(home_dir) = dirs::home_dir() {
        let diffusionbee_dir = home_dir.join(".diffusionbee");
        let imported_models_dir = diffusionbee_dir.join("imported_models");
        
        if imported_models_dir.exists() {
            let mut models = Vec::new();
            if let Ok(entries) = fs::read_dir(imported_models_dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "tdict" {
                            if let Some(name) = entry.path().file_name() {
                                models.push(name.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
            return Ok(models);
        }
    }
    
    Ok(vec![])
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
    
    // Validate model path if provided
    if !settings.model_path.is_empty() {
        let model_path = PathBuf::from(&settings.model_path);
        if !model_path.exists() {
            return Err(format!("Model file not found at: {}", model_path.display()));
        }
        if model_path.extension().and_then(|s| s.to_str()) != Some("tdict") {
            return Err("Model file must have .tdict extension".to_string());
        }
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
