# Worker Runtime Environment for the OpenWorkers project

This repository contains the source code for the OpenWorkers project. OpenWorkers is a project that aims to provide a simple, easy to use, and secure platform for running code in the cloud. The project is currently in the early stages of development, and is not yet ready for production use.

## Getting Started

Install rustup and cargo:

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Usage

### Run a single file
```bash
# Development
cargo run path/to/file.js
# Build
./target/release/rs-engine path/to/file.js
```

### Run a single command
```bash
# Development
cargo run -- --eval "console.log('Hello World!')"
# Build
./target/release/rs-engine --eval "console.log('Hello World!')"
```

## Building

### Build a single file
```bash
cargo build --release
```
