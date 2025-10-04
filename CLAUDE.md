# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Flappy Bird-like game with neural network AI agents trained using genetic algorithms. The project is currently undergoing a major refactoring to separate the game engine from game-specific code.

**Current State**: The repository is in the middle of engine separation refactoring (see REFACTOR_PLAN.md). The codebase is organized as a Cargo workspace with:
- `engine/` - Generic game engine (stub, being developed)
- `examples/flappy_bird/` - Flappy Bird game implementation (currently contains all actual code)

## Build Commands

### Native Development
```bash
# Build and run the flappy bird example
cargo run --bin flappy

# Build the engine library
cargo build -p engine

# Run tests
cargo test

# Check compilation without building
cargo check
```

### WebAssembly Build
```bash
# Build for web deployment
wasm-pack build --target web

# The wasm-pack command builds the project for WASM target
# Output goes to pkg/ directory
```

**Important**: WASM builds use `wgpu` with the `webgl` feature and require special handling for `winit 0.30` compatibility.

## Architecture

### ECS Architecture (specs)
The project uses the `specs` ECS library for game entity management:

- **Components** (`examples/flappy_bird/src/components.rs`): Define data structures for entities
  - Generic: `Transform`, `Collider`, `Tile`, `Text`
  - Game-specific: `Player`, `Pipe`, `Background`, `DNA`, `NeuralLayer`

- **Systems** (`examples/flappy_bird/src/system/`): Implement game logic
  - `UnifiedDispatcher` pattern for system execution
  - Systems include: physics, collision detection, neural network processing, scrolling, camera updates

- **Resources** (`examples/flappy_bird/src/resources/`): Global state
  - `Camera`, `DeltaTime`, `InputHandler`, `GeneHandler`, `Score`, `GameStage`

### Rendering (wgpu + winit)
- **Renderer** (`examples/flappy_bird/src/renderer/`): wgpu-based rendering engine
  - `RenderState`: Main rendering interface
  - `Mesh`, `Texture`, `Vertex`: Core rendering primitives
  - `PipelineManager`, `GpuResourceManager`, `FontManager`: Resource management
  - `RenderInputData`: Conversion from ECS world to render data

- **Window Management** (`winit_state.rs`, `application.rs`):
  - Uses `winit 0.30` with `ApplicationHandler` trait
  - Handles window events, resize, input, and redraw requests
  - WASM-specific bindings in `wasm_bindings.rs`

### Neural Network & Genetic Algorithm
- **DNA Component**: Stores genetic information (weights/biases) as flat array
- **NeuralLayer Component**: Runtime neural network state (weights, values, bias)
- **ProcessNN System**: Executes forward pass through neural network
- **GeneHandler Resource**: Manages genetic algorithm operations (crossover, mutation, selection)

AI players make decisions based on:
- Distance to next pipe (horizontal)
- Height difference to pipe gap
- Player's vertical velocity

### Game State Flow
1. **GameState** (`game_state.rs`): Main game controller
   - Manages ECS `World` and system `Dispatcher`
   - Handles stage transitions: Ready → Run → End
   - Entity spawning via builder functions

2. **Application** (`application.rs`): Window and event loop
   - Owns `GameState` and `RenderState`
   - Bridges winit events to game logic
   - Manages frame timing and delta time

## WASM-Specific Considerations

- Use `instant` crate (not `std::time`) for cross-platform timing
- Random number generation requires `getrandom` with `js` feature
- Window creation uses web canvas via `web-sys`
- Font rendering and texture loading adapted for WASM environment
- Custom bindings in `wasm_bindings.rs` for JS interop

## Ongoing Refactoring

Per REFACTOR_PLAN.md, the goal is to:
1. Extract generic engine components to `engine/` crate
2. Implement `Scene` trait to abstract game-specific logic
3. Make `Application` generic over `Scene` implementations
4. Enable reusability for new projects (e.g., 2D physics demos)

**Current Branch**: `refactor/engine-separation`

When making changes, be aware that:
- Generic components (Transform, Collider, rendering) should eventually move to `engine/`
- Game-specific logic (Flappy Bird AI, pipes, scrolling) stays in `examples/flappy_bird/`
- The refactor must maintain 100% compatibility with existing Flappy Bird game and WASM builds
