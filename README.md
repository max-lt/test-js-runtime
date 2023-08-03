# Worker Runtime Environment for the OpenWorkers project

This repository contains the source code for the OpenWorkers project. OpenWorkers is a project that aims to provide a simple, easy to use, and secure platform for running code in the cloud. The project is currently in the early stages of development, and is not yet ready for production use.

## Getting Started

Install rustup and cargo:

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Usage

### Options
```bash
USAGE:
  rs-engine [filepath or eval script] [OPTIONS]

OPTIONS:
  --fetch   Trigger the fetch event
```

### Examples
```bash
# Development
cargo run path/to/file.js
cargo run eval "console.log('Hello World!')"
cargo run serve path/to/file.js # Will serve the worker script on localhost:3000
# Build
./target/release/rs-engine path/to/file.js --fetch
./target/release/rs-engine eval "console.log('Hello World!')"
```

## Building

### Build a single file
```bash
cargo build --release
```
