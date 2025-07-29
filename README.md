# petri-rs

[![build](https://github.com/rznn7/petri-rs/actions/workflows/build.yml/badge.svg)](https://github.com/rznn7/petri-rs/actions/workflows/build.yml)

A Conway's Game of Life implementation in Rust with GUI. **This project was primarily created as a learning exercise to familiarize with Rust.**

## Features

- Interactive GUI with configurable grid size
- Play/pause simulation with manual stepping
- Click cells to toggle state
- Zoom and pan controls

## Controls

- **Click Play**: Start/stop simulation
- **Left Click**: Toggle cell (auto-pauses)
- **Ctrl + Wheel**: Zoom
- **Middle Click + Drag**: Pan

## Quick Start

```bash
git clone https://github.com/rznn7/petri-rs.git
cd petri-rs
cargo run
```

## Learning Focus

This project explores key Rust concepts:

- Ownership & borrowing patterns
- Error handling with `Result<T, E>`
- Testing with mocks and abstractions
- Module organization and trait implementation
- GUI development with egui

Built with [eframe](https://github.com/emilk/egui) for cross-platform compatibility.
