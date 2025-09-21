# WebAssembly Pixel Editor Tutorial

An interactive pixel art editor built with Rust and compiled to WebAssembly, demonstrating the power of WASM for high-performance web applications.

## About This Project

This repository contains a complete tutorial and working implementation of a pixel editor using WebAssembly. The project showcases how to build performance-critical web applications by combining Rust's memory safety and speed with JavaScript's web capabilities.

## Complete Tutorial

This project has a comprehensive tutorial: [`Introduction to WebAssembly.pdf`](Introduction%20to%20WebAssembly.pdf)

The tutorial covers:
- **Introduction**: What is WebAssembly and why it matters
- **Setup Guide**: Installing Rust and WASM toolchain
- **Step-by-Step Implementation**: Building the pixel editor from scratch
- **Performance Analysis**: Understanding WASM advantages
- **Debugging Techniques**: Development workflow and optimization

## What You'll Learn

- **WebAssembly Fundamentals**: Understanding WASM's role in modern web development
- **Rust-WASM Integration**: Using `wasm-pack` and `wasm-bindgen` for seamless bindings
- **Performance Optimization**: Achieving near-native speed for computational tasks
- **Memory Management**: Leveraging Rust's ownership system in web environments
- **Cross-Language Development**: Bridging Rust logic with JavaScript UI

## 📁 Project Structure

```
pixel-editor-wasm/
├── src/
│   └── lib.rs              # Core Rust implementation
├── pkg/                    # Generated WASM bindings (after build)
│   ├── pixel_editor_wasm.js
│   ├── pixel_editor_wasm_bg.wasm
│   └── ...
├── index.html              # Web interface and demo
├── Cargo.toml              # Rust dependencies and configuration
└── README.md               # This file
```

## Features

- **Interactive Drawing**: Click and drag to paint pixels on the canvas
- **Color Selection**: Use the color picker to change brush colors
- **Canvas Management**: Clear the canvas or apply algorithmic gradients
- **Real-time Performance**: Experience the speed difference of WASM vs JavaScript
- **Developer Console**: View debug messages from the WASM module

## Key Implementation Details

### Rust Core ([`src/lib.rs`](pixel-editor-wasm/src/lib.rs))

The [`PixelEditor`](pixel-editor-wasm/src/lib.rs) struct manages:
- Pixel buffer as a flat `Vec<u8>` for optimal performance
- RGBA color management with proper alpha channel handling
- Coordinate conversion from 2D canvas to 1D memory layout
- Memory-safe bounds checking to prevent buffer overflows

### Web Integration ([`index.html`](pixel-editor-wasm/index.html))

The HTML interface provides:
- Canvas rendering using `ImageData` for direct pixel manipulation
- Mouse event handling for interactive drawing
- Color picker integration with hex-to-RGB conversion
- Asynchronous WASM module loading and initialization

### Build Configuration ([`Cargo.toml`](pixel-editor-wasm/Cargo.toml))

Configured for WebAssembly with:
- `cdylib` crate type for dynamic library generation
- `wasm-bindgen` for JavaScript bindings
- `web-sys` for browser API access

## Educational Value

This project demonstrates key Computer Science concepts:

- **Data Structures**: Efficient memory layout for graphics programming
- **Algorithms**: Mathematical calculations for pixel manipulation and gradients
- **Systems Programming**: Memory management and performance optimization
- **Software Engineering**: Cross-language integration and modular design
- **Computer Graphics**: Pixel buffer management and rendering pipelines

## Performance Benefits

WebAssembly provides significant advantages for this type of application:

- **Speed**: 5-20x faster than equivalent JavaScript for mathematical operations
- **Memory Safety**: Rust prevents buffer overflows and memory leaks
- **Predictable Performance**: Consistent execution times for real-time applications
- **Code Reuse**: Same logic can be compiled for multiple platforms

## Browser Compatibility

- **Chrome/Edge**: Full support with DevTools integration
- **Firefox**: Complete WASM support with debugging capabilities
- **Safari**: Modern versions support all features used

## 🔗 References

- [WebAssembly Official Documentation](https://webassembly.org/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
