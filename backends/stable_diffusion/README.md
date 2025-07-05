# DiffusionBee Backend

This is the Python backend for the DiffusionBee Tauri port, providing Stable Diffusion image generation capabilities.

## Setup

### Prerequisites

- Python 3.8 or higher
- Poetry (for dependency management)

### Installation

1. Install dependencies using Poetry:
```bash
poetry install
```

2. Activate the virtual environment:
```bash
poetry shell
```

### Development

For development, you can run the backend directly:
```bash
poetry run python diffusionbee_backend.py
```

Or use the Poetry script:
```bash
poetry run diffusionbee-backend
```

### Building for Production

To create a standalone executable:
```bash
poetry run pyinstaller --onefile diffusionbee_backend.py
```

## Usage

The backend expects input via stdin in the format:
```
b2py t2im {"prompt": "your prompt", "img_width": 512, "img_height": 512, ...}
```

And outputs results via stdout in the format:
```
sdbk nwim {"generated_img_path": "/path/to/image.png"}
```

## Model Files

The backend requires `.tdict` model files to be placed in `~/.diffusionbee/imported_models/`.

## Architecture

- `diffusionbee_backend.py` - Main entry point
- `stable_diffusion/` - Core Stable Diffusion implementation
- `applets/` - Additional functionality (frame interpolation, etc.)
- `fake_interface/` - Mock interface for testing 