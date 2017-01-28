# ionic_engine

[![Build Status](https://travis-ci.org/AperEntertainment/ionic_engine.svg?branch=master)](https://travis-ci.org/AperEntertainment/ionic_engine)

IonicEngine is an multimedia graphic library in Rust. It uses [glfw-rs](https://github.com/PistonDevelopers/glfw-rs).

## Using IonicEngine

### Prerequisites

#### Windows

IonicEngine use [glfw-rs](https://github.com/PistonDevelopers/glfw-rs), it needs GLFW 3.x and [CMake](https://cmake.org) installed.
Add them in your PATH environment variable.

#### Others

Just install GLFW 3.x and [CMake](https://cmake.org) via your package manager.

### Including IonicEngine in your project

Add this to your `Cargo.toml`:

```toml
[dependencies.ionic_engine]
git = "https://github.com/AperEntertainment/ionic_engine.git"
```
