# DiffusionBee Tauri Port - User Stories (MVP)

This document outlines the user stories for the Minimum Viable Product (MVP) of the DiffusionBee Tauri port.

## Core Features

### Text-to-Image Generation

*   **As a user, I want to be able to enter a text prompt and generate an image.**
*   **As a user, I want to be able to specify the image dimensions (width and height).**
*   **As a user, I want to be able to adjust the number of inference steps.**
*   **As a user, I want to be able to set the guidance scale (CFG).**
*   **As a user, I want to see a progress bar while the image is being generated.**

### Image-to-Image Generation

*   **As a user, I want to be able to upload an initial image.**
*   **As a user, I want to be able to provide a text prompt to modify the initial image.**
*   **As a user, I want to be able to control the strength of the image-to-image generation.**

### Inpainting

*   **As a user, I want to be able to upload an image to edit.**
*   **As a user, I want to be able to mask a specific area of the image.**
*   **As a user, I want to be able to provide a text prompt to fill in the masked area.**

### History

*   **As a user, I want to be able to view a gallery of all the images I have generated.**
*   **As a user, I want to be able to click on an image in the gallery to view it in a larger size.**
*   **As a user, I want to be able to delete images from my history.**

### Model Management

*   **As a user, I want to be able to see a list of available Stable Diffusion models.**
*   **As a user, I want to be able to download new models from a remote source.**
*   **As a user, I want to be able to switch between the models I have downloaded.**

### Settings

*   **As a user, I want to be able to configure the default image dimensions.**
*   **As a user, I want to be able to set the default number of inference steps.**
*   **As a user, I want to be able to choose the default guidance scale.**
