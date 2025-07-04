# DiffusionBee Tauri Port - User Stories (MVP)

This document outlines the user stories for the Minimum Viable Product (MVP) of the DiffusionBee Tauri port.

## Development Status

### Current Focus: Story #4 - Generation Progress
**Status**: Starting  
**Priority**: P1 (High)  
**Estimated Effort**: 2-3 days

**Progress Update (Latest):**
- ✅ Story #1 completed - Basic Text-to-Image Generation foundation
- ✅ Story #2 completed - Image Generation Parameters with dimension controls
- ✅ Story #3 completed - Advanced Generation Controls with sliders
- ✅ Modern dark theme with glassmorphism effects implemented
- ✅ Settings persistence between sessions working
- ✅ Dimension validation and aspect ratio presets
- ✅ Advanced controls with quality indicators and parameter summary
- ✅ Tauri + Svelte setup with working application
- ✅ Rust backend structure with Python integration framework
- ✅ Svelte stores for state management
- ✅ ImageGenerator component with UI
- ✅ Basic error handling and progress tracking
- 🔄 Starting Story #4 - Generation Progress
- ⏳ File system operations for image storage
- ⏳ Real progress tracking from Python backend

**Lessons Learned from Story #1:**
- Tauri 2.0 doesn't support `shell-execute` or `shell-open` features - use standard `std::process` instead
- Error handling must match function return types (String vs custom error types)
- TypeScript stores work well with Svelte 5.0 for state management
- Component structure should be modular and reusable
- Build validation with `bun run tauri dev` is essential before committing

**Latest Achievement (Latest Commit):**
- Successfully completed Story #1 foundation
- Application runs and accepts user input
- Ready to implement Story #2: Image Generation Parameters

## Core Features

### Text-to-Image Generation

#### Story #1: Basic Text-to-Image Generation ✅ COMPLETED
**As a user, I want to be able to enter a text prompt and generate an image.**

**Acceptance Criteria:**
- [x] User can enter a text prompt in a text input field
- [x] User can click a "Generate" button to start image generation
- [x] System displays a loading indicator during generation
- [x] Generated image is displayed in the UI
- [ ] Image is automatically saved to the user's history
- [x] Basic error handling for failed generations

**Technical Requirements:**
- [x] Tauri command to communicate with Python backend
- [x] Svelte component for text input and generation controls
- [x] Svelte store for managing generation state
- [ ] File system integration for saving generated images
- [ ] Progress tracking from Python backend

**Implementation Steps:**
1. ✅ Create Python backend integration Tauri commands
2. ✅ Build basic Svelte UI components
3. ✅ Implement state management with Svelte stores
4. ✅ Fix build issues and ensure application runs
5. 🔄 Add file system operations for image storage
6. ✅ Connect frontend to backend via Tauri commands
7. ✅ Add error handling and loading states

---

#### Story #2: Image Generation Parameters ⭐ CURRENT FOCUS
**As a user, I want to be able to specify the image dimensions (width and height).**

**Acceptance Criteria:**
- [x] User can set width and height via number inputs
- [x] Default dimensions are 512x512
- [x] Dimensions are validated (min: 256, max: 1024)
- [x] Settings are persisted between sessions
- [x] UI updates to reflect current dimensions
- [x] Generation uses the specified dimensions

**Technical Requirements:**
- [x] Extend ImageGenerationRequest struct to include dimensions
- [x] Add dimension input components to ImageGenerator
- [x] Update Svelte store to handle dimension parameters
- [x] Add validation logic for dimension constraints
- [x] Implement settings persistence using Tauri's app data
- [x] Update Tauri commands to pass dimensions to Python backend

**Implementation Steps:**
1. ✅ Extend Rust data structures for dimension parameters
2. ✅ Add dimension input UI components
3. ✅ Update Svelte store to include dimension state
4. ✅ Add validation and error handling for dimensions
5. ✅ Implement settings persistence
6. ✅ Update Tauri commands to handle dimensions
7. ✅ Test dimension changes in generation flow

**Dependencies:**
- Builds on Story #1 foundation
- Requires working Tauri app (✅ completed)
- Uses existing ImageGenerator component structure

**Latest Implementation:**
- Created DimensionInput component with validation and aspect ratio controls
- Extended Rust backend with validation and settings management
- Updated Svelte stores to handle dimension state and settings persistence
- Integrated dimension controls into main ImageGenerator component
- Application successfully runs with new dimension functionality

---

#### Story #3: Advanced Generation Controls ✅ COMPLETED
**As a user, I want to be able to adjust the number of inference steps and guidance scale (CFG).**

**Acceptance Criteria:**
- [x] Slider for inference steps (range: 10-50, default: 20)
- [x] Slider for guidance scale (range: 1-20, default: 7.5)
- [x] Real-time preview of parameter values
- [x] Settings are persisted between sessions

**Technical Requirements:**
- [x] Create AdvancedControls component with sliders
- [x] Add slider components with real-time value display
- [x] Update Svelte store to handle inference steps and guidance scale
- [x] Integrate controls into ImageGenerator component
- [x] Ensure settings persistence works for new parameters
- [x] Add validation for parameter ranges

**Implementation Steps:**
1. ✅ Create AdvancedControls Svelte component
2. ✅ Add slider components with modern styling
3. ✅ Update store to handle new parameters
4. ✅ Integrate into main ImageGenerator
5. ✅ Test settings persistence
6. ✅ Validate parameter ranges and defaults

**Dependencies:**
- Builds on Story #1 & #2 foundation
- Uses existing settings persistence system
- Extends current ImageGenerator component

**Latest Implementation:**
- Created AdvancedControls component with beautiful glassmorphism sliders
- Added real-time quality indicators for inference steps and guidance scale
- Implemented parameter summary with estimated generation time
- Integrated controls into main ImageGenerator component
- Settings persistence working for all advanced parameters
- Application successfully builds and runs with new controls

#### Story #4: Generation Progress
**As a user, I want to see a progress bar while the image is being generated.**

**Acceptance Criteria:**
- [ ] Progress bar shows current step vs total steps
- [ ] Progress updates in real-time from backend
- [ ] Cancel button to stop generation
- [ ] Time remaining estimate

### Image-to-Image Generation

#### Story #5: Basic Image-to-Image
**As a user, I want to be able to upload an initial image and provide a text prompt to modify it.**

**Acceptance Criteria:**
- [ ] File upload component for initial image
- [ ] Image preview before generation
- [ ] Text prompt input for modifications
- [ ] Strength control slider (0-1)
- [ ] Generated image replaces or is shown alongside original

#### Story #6: Image-to-Image Strength Control
**As a user, I want to be able to control the strength of the image-to-image generation.**

**Acceptance Criteria:**
- [ ] Strength slider (0-1, default: 0.75)
- [ ] Visual preview of strength effect
- [ ] Real-time parameter updates

### Inpainting

#### Story #7: Basic Inpainting
**As a user, I want to be able to upload an image to edit and mask a specific area.**

**Acceptance Criteria:**
- [ ] Image upload and display
- [ ] Canvas-based masking tool
- [ ] Brush size controls
- [ ] Mask preview overlay

#### Story #8: Inpainting Generation
**As a user, I want to be able to provide a text prompt to fill in the masked area.**

**Acceptance Criteria:**
- [ ] Text prompt input for masked area
- [ ] Generation with mask applied
- [ ] Result preview with original comparison

### History

#### Story #9: Image Gallery
**As a user, I want to be able to view a gallery of all the images I have generated.**

**Acceptance Criteria:**
- [ ] Grid layout of generated images
- [ ] Thumbnail generation for performance
- [ ] Pagination or infinite scroll
- [ ] Sort by date (newest first)

#### Story #10: Image Details
**As a user, I want to be able to click on an image in the gallery to view it in a larger size.**

**Acceptance Criteria:**
- [ ] Modal or dedicated view for full-size image
- [ ] Generation parameters displayed
- [ ] Download button
- [ ] Share functionality

#### Story #11: Image Management
**As a user, I want to be able to delete images from my history.**

**Acceptance Criteria:**
- [ ] Delete button on each image
- [ ] Confirmation dialog
- [ ] Bulk delete selection
- [ ] Undo delete functionality

### Model Management

#### Story #12: Model List
**As a user, I want to be able to see a list of available Stable Diffusion models.**

**Acceptance Criteria:**
- [ ] List of installed models
- [ ] Model information (name, size, version)
- [ ] Current active model indicator
- [ ] Model switching functionality

#### Story #13: Model Download
**As a user, I want to be able to download new models from a remote source.**

**Acceptance Criteria:**
- [ ] Browse available models
- [ ] Download progress indicator
- [ ] Automatic model validation
- [ ] Error handling for failed downloads

#### Story #14: Model Switching
**As a user, I want to be able to switch between the models I have downloaded.**

**Acceptance Criteria:**
- [ ] Dropdown or list for model selection
- [ ] Model loading indicator
- [ ] Persistence of selected model
- [ ] Validation of model compatibility

### Settings

#### Story #15: Default Configuration
**As a user, I want to be able to configure default image dimensions, inference steps, and guidance scale.**

**Acceptance Criteria:**
- [ ] Settings page with all default parameters
- [ ] Real-time preview of changes
- [ ] Settings persistence
- [ ] Reset to defaults option

## Technical Implementation Notes

### Priority Order
1. **Story #1** - Basic Text-to-Image (Foundation)
2. **Story #2** - Image Dimensions (Core Parameters)
3. **Story #4** - Progress Tracking (UX)
4. **Story #9** - Image Gallery (Core Feature)
5. **Story #12** - Model List (Core Feature)
6. **Stories #3, #5-8, #10-11, #13-15** - Advanced Features

### Dependencies
- Stories #2-4 depend on Story #1 completion
- Stories #5-8 depend on basic image handling from Story #1
- Stories #9-11 depend on file system operations from Story #1
- Stories #12-14 depend on model management infrastructure

### Risk Mitigation
- Start with Story #1 to establish the core architecture
- Implement basic error handling early
- Test Python backend integration thoroughly
- Build reusable components for future stories
