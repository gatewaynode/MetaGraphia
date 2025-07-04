# DiffusionBee Tauri Port

This document outlines the plan to port the DiffusionBee application from Electron to Tauri, leveraging Svelte for the frontend.

## Development Progress

### Current Status: Story #1 Implementation
We have successfully implemented the foundation for Story #1 (Basic Text-to-Image Generation) with the following components:

#### ✅ Completed Features
- **Tauri + Svelte Setup**: Modern tech stack with Tauri 2.0 and Svelte 5.0
- **Rust Backend Structure**: Python integration framework with proper error handling
- **Svelte State Management**: Comprehensive store system for generation state
- **UI Components**: Clean, modern interface for text-to-image generation
- **Error Handling**: Basic error handling and user feedback
- **Progress Tracking**: UI for generation progress (placeholder implementation)

#### 🔄 In Progress
- **Python Backend Integration**: Framework in place, needs real implementation
- **File System Operations**: Image storage and management

#### ⏳ Next Steps
- Implement real Python backend communication
- Add file system operations for image storage
- Implement progress tracking from Python backend
- Add image history functionality

### Running the Application
```bash
# Install dependencies
bun install

# Start development server
bun run dev

# Run Tauri in development mode
bun run tauri dev

# Build for production
bun run build
bun run tauri build
```

## Product Requirements Document (PRD)

### 1. Introduction & Vision

The goal of this project is to create a more performant, secure, and lightweight version of DiffusionBee by porting it from Electron to Tauri. The new application will retain the core features of the original while providing a better user experience through a native webview and a smaller bundle size.

### 2. Core Features (MVP)

The Minimum Viable Product (MVP) will include the following features:

*   **Text-to-Image Generation:** Generate images from a text prompt.
*   **Image-to-Image Generation:** Generate images from an initial image and a text prompt.
*   **Inpainting:** Edit parts of an image by masking and providing a text prompt.
*   **History:** View a gallery of previously generated images.
*   **Model Management:** Download and switch between different Stable Diffusion models.
*   **Settings:** Configure application settings.

### 3. Technology Stack

*   **Frontend:** Svelte 5 + SvelteKit
*   **Backend:** Rust (Tauri)
*   **AI Backend:** Python (utilizing the existing `backends` directory)

### 4. Target Audience

The target audience includes existing DiffusionBee users and new users who are looking for a native and efficient Stable Diffusion UI for their desktop.

### 5. Non-Functional Requirements

*   **Performance:** The application must be fast, responsive, and have a low memory footprint.
*   **Cross-Platform:** The application must be buildable for macOS, Windows, and Linux.
*   **Security:** The application should be secure, benefiting from Tauri's security features.

## Coding Style Guide

### General

*   **Development Management:** Only work on one story at a time taken from the USER-STORIES.md file, update the story with comments when finished.
*   **Version Control:** Use Git with a new branch for every story named "story-#".
*   **Dependencies:** Always use the latest stable versions of dependencies.
*   **Logging:** Always log all messages to ./diffusion.logs file in an appending manner.
*   **Security Review:** Always review code for security flaws before finishing a task.
*   **Simplicity First:** Always review code to make sure it is simple and straightforward before finishing a task.
*   **Commits:** Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.
*   **Validation:** Validate at least minimally from this folder with `bun run tauri dev`
*   **Documentation:** All new features should be documented in this README or relevant files.

### Frontend (Svelte / JavaScript)

*   **Formatting:** Use Prettier for automatic code formatting. A `.prettierrc` file should be included in the project.
*   **Linting:** Use ESLint to identify and fix code quality issues.
*   **Component Design:** Components should be small, reusable, and focused on a single responsibility.
*   **State Management:** For complex state, use Svelte stores.
*   **API Interaction:** All communication with the Tauri backend should be done through the `invoke` function, with clear command names.

### Backend (Rust)

*   **Formatting:** Use `rustfmt` to maintain a consistent code style.
*   **Linting:** Use `clippy` to catch common mistakes and improve code quality.
*   **Error Handling:** Use `Result` and `?` for robust error handling. Avoid `unwrap()` in production code.
*   **Modularity:** Organize code into modules to keep it maintainable.
*   **Tauri Commands:** All Tauri commands should be well-documented and handle errors gracefully.


I'll review the codebase, README.md, and USER-STORIES.md to understand the project structure and goals. Let me start by examining the current state of the project.

Read file: ported-diffusion-app/package.json
Read file: ported-diffusion-app/src-tauri/Cargo.toml
Read file: ported-diffusion-app/src-tauri/src/main.rs
Read file: ported-diffusion-app/src-tauri/src/lib.rs
Read file: ported-diffusion-app/src/routes/+page.svelte
Ran tool
Read file: backends/stable_diffusion/diffusionbee_backend.py
Read file: electron_app/package.json
Now let me examine the current state of the Tauri app more closely:

Read file: ported-diffusion-app/src-tauri/tauri.conf.json
## Codebase Review Summary

Based on my analysis of the codebase, README.md, and USER-STORIES.md, here's my comprehensive review:

## **Project Overview**

This is a **DiffusionBee Tauri Port** project that aims to convert the existing Electron-based DiffusionBee application to a Tauri-based desktop app using Svelte for the frontend. The project is currently in its very early stages.

## **Current State Analysis**

### **What's Already Done:**
1. **Basic Tauri + Svelte Setup**: The project has a minimal Tauri 2.0 + Svelte 5.0 + SvelteKit setup
2. **Project Structure**: Proper separation between frontend (`src/`) and backend (`src-tauri/`)
3. **Build Configuration**: Basic Tauri configuration with proper build commands
4. **Existing Backend**: The original Python backend (`backends/`) is available and functional

### **What's Missing (Major Gaps):**
1. **No Core Features Implemented**: The current app only has a basic "Hello World" example
2. **No Python Backend Integration**: The existing Python backend isn't connected to Tauri
3. **No UI Components**: No Svelte components for the actual DiffusionBee features
4. **No State Management**: No Svelte stores or state management system
5. **No Error Handling**: Basic error handling structure missing

## **Technical Architecture Assessment**

### **Strengths:**
- **Modern Tech Stack**: Tauri 2.0 + Svelte 5.0 is cutting-edge
- **Performance**: Tauri will provide better performance than Electron
- **Security**: Tauri's security model is superior to Electron
- **Existing Backend**: The Python backend is mature and feature-complete

### **Challenges:**
- **Python Integration**: Need to bridge Tauri (Rust) with the existing Python backend
- **Complex UI**: The original Vue.js UI is quite sophisticated and needs to be rebuilt in Svelte
- **File Management**: Need to handle model downloads, image storage, etc.

## **Recommendations for Development**

### **Phase 1: Foundation (Current Priority)**
1. **Python Backend Integration**: Create Tauri commands to communicate with the Python backend
2. **Basic UI Framework**: Set up Svelte stores and basic layout components
3. **File System Operations**: Implement model management and image storage

### **Phase 2: Core Features**
1. **Text-to-Image**: Implement the basic text-to-image generation
2. **Image Gallery**: Create the history/gallery view
3. **Settings**: Basic configuration management

### **Phase 3: Advanced Features**
1. **Image-to-Image**: Build on the foundation
2. **Inpainting**: More complex UI interactions
3. **Model Management**: Download and switching functionality

## **Specific Technical Recommendations**

### **1. Python Backend Integration Strategy**
```rust
// Example Tauri command structure needed:
#[tauri::command]
async fn generate_image(prompt: String, options: ImageOptions) -> Result<ImageResult, String> {
    // Call Python backend via subprocess or IPC
}
```

### **2. State Management Architecture**
```javascript
// Svelte stores needed:
- imageGenerationStore
- modelManagementStore  
- settingsStore
- historyStore
```

### **3. UI Component Structure**
```
src/
├── lib/
│   ├── components/
│   │   ├── ImageGenerator.svelte
│   │   ├── ImageGallery.svelte
│   │   ├── ModelManager.svelte
│   │   └── Settings.svelte
│   ├── stores/
│   └── utils/
```

## **Risk Assessment**

### **High Risk Areas:**
1. **Python Integration Complexity**: Bridging Rust and Python can be challenging
2. **Performance**: Need to ensure the Python backend doesn't block the UI
3. **File Management**: Handling large model files and images efficiently

### **Medium Risk Areas:**
1. **UI Recreation**: Rebuilding the complex Vue.js UI in Svelte
2. **Cross-Platform Compatibility**: Ensuring the Python backend works on all platforms

## **Next Steps Recommendation**

Based on the USER-STORIES.md, I recommend starting with:

1. **Story #1**: "As a user, I want to be able to enter a text prompt and generate an image"
   - This will force you to solve the Python integration problem
   - Create the basic UI framework
   - Establish the communication pattern between Tauri and Python

2. **Infrastructure First**: Before diving into features, establish:
   - Python backend communication
   - Basic Svelte store structure
   - Error handling patterns
   - File system operations

Would you like me to help you start implementing any of these specific areas? I can help with:
- Setting up the Python backend integration
- Creating the basic Svelte component structure
- Implementing the first user story
- Setting up proper error handling and state management

What would you like to focus on first?