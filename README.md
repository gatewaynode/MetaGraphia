# DiffusionBee Tauri Port

This document outlines the plan to port the DiffusionBee application from Electron to Tauri, leveraging Svelte for the frontend.

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

*   **Frontend:** Svelte / SvelteKit
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

*   **Version Control:** Use Git with a branching strategy (e.g., GitFlow).
*   **Commits:** Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.
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