# Ray Tracer (RT)

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-000000?style=for-the-badge&logo=rust&logoColor=white)
![Ray Tracing](https://img.shields.io/badge/Ray%20Tracing-FF6B6B?style=for-the-badge&logo=raylib&logoColor=white)
![JSON](https://img.shields.io/badge/JSON-000000?style=for-the-badge&logo=json&logoColor=white)

</div>

A physically-based ray tracing renderer written in Rust that simulates realistic light behavior and generates photorealistic images.

<details>
<summary>📋 Table of Contents</summary>

- [Ray Tracer (RT)](#ray-tracer-rt)
  - [🎨 About](#-about)
  - [✨ Features](#-features)
  - [🚀 Installation](#-installation)
    - [Prerequisites](#prerequisites)
    - [Setup](#setup)
  - [💻 Usage](#-usage)
    - [Basic Usage](#basic-usage)
    - [Custom Scene](#custom-scene)
  - [🎬 Scene Configuration](#-scene-configuration)
    - [Camera](#camera)
    - [Light](#light)
    - [Objects](#objects)
  - [📸 Examples](#-examples)
  - [📚 Documentation](#-documentation)
  - [📁 Project Structure](#-project-structure)
  - [🛠️ Dependencies](#️-dependencies)
  - [🤝 Contributing](#-contributing)

</details>

## 🎨 About

This ray tracer implements a rendering technique that simulates the physical behavior of light as it interacts with objects in a 3D scene. By tracing rays of light from a virtual camera through each pixel, the renderer calculates realistic lighting, shadows, and reflections to produce high-quality images.

## ✨ Features

- **Multiple Primitive Types**
  - Spheres
  - Planes
  - Cubes
  - Cylinders

- **Realistic Lighting**
  - Point light sources with configurable intensity
  - Color-tinted lighting
  - Shadow casting

- **Flexible Camera System**
  - Configurable camera position and orientation
  - Look-at target positioning

- **JSON-based Scene Definition**
  - Easy scene configuration
  - Hot-swappable scene files
  - Human-readable format

- **PPM Image Output**
  - Standard image format output
  - Configurable resolution

## 🚀 Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rt
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## 💻 Usage

### Basic Usage

Run the ray tracer with the default scene configuration:

```bash
cargo run --release
```

This will:
1. Read the scene configuration from `scene.json`
2. Render the scene
3. Generate a PPM image file as output

### Custom Scene

Modify the `scene.json` file to create your own custom scenes. See the [Scene Configuration](#scene-configuration) section for details.

## 🎬 Scene Configuration

The `scene.json` file defines all elements of your 3D scene. It uses a JSON structure with the following components:

### Camera

Defines the viewpoint and orientation:

```json
"camera": {
    "origin": { "e": [3.0, 2.2, -4.0] },
    "look_at": { "e": [2.0, 0.0, 0.0] }
}
```

### Light

Configures the light source:

```json
"light": {
    "position": { "e": [2.0, 6.0, -4.0] },
    "color": { "e": [1.0, 1.0, 1.0] },
    "intensity": 1.0
}
```

### Objects

Add various geometric primitives to your scene:

```json
"spheres": [...],
"planes": [...],
"cubes": [...],
"cylinders": [...]
```

For detailed information about all configuration options, see the [documentation](./documentation.md).

## 📸 Examples

Here's an example of a rendered scene:

![Ray Traced Scene](Screenshot%20from%202025-10-15%2022-49-49.png)

## 📚 Documentation

For comprehensive documentation on:
- Ray tracing concepts
- Detailed JSON schema
- Object properties and parameters
- Color specifications

Please refer to the [documentation.md](./documentation.md) file.

## 📁 Project Structure

```
rt/
├── src/
│   ├── main.rs           # Entry point
│   ├── camera.rs         # Camera implementation
│   ├── color.rs          # Color utilities
│   ├── ray.rs            # Ray mathematics
│   ├── hittable.rs       # Hit detection trait
│   ├── hittable_list.rs  # Collection of hittable objects
│   ├── sphere.rs         # Sphere primitive
│   ├── plane.rs          # Plane primitive
│   ├── cube.rs           # Cube primitive
│   ├── cylinder.rs       # Cylinder primitive
│   ├── light.rs          # Lighting calculations
│   ├── vec3.rs           # 3D vector mathematics
│   ├── output.rs         # Image generation
│   ├── param.rs          # Parameters
│   └── common.rs         # Common utilities
├── scene.json            # Scene configuration
├── documentation.md      # Detailed documentation
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## 🛠️ Dependencies

- `rand` (0.8.5) - Random number generation
- `serde` (1.0.215) - Serialization framework
- `serde_json` (1.0.133) - JSON parsing

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

<div align="center">

**おはよう世界！** 🌅

**⭐ Star this repository if you found it helpful! ⭐**

Made with ❤️ from 🇸🇳

</div>