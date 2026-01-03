# Alfred

[![Crates.io](https://img.shields.io/crates/v/alfred-clean.svg)](https://crates.io/crates/alfred-clean)
[![Build Status](https://github.com/gregcrn/alfred/workflows/CI/badge.svg)](https://github.com/gregcrn/alfred/actions)

A high-performance command line tool to automatically organize files in a directory based on their extensions.

## Installation

```bash
cargo install alfred-clean
```

## Usage

Clean the current directory:

```bash
alfred clean
```

Clean a specific directory:

```bash
alfred clean --path ~/Downloads
```

Watch a directory for new files and organize them automatically:

```bash
alfred watch --path ~/Downloads
```

Perform a dry run to see what would happen without moving files:

```bash
alfred clean --dry-run
```

## Features

- **Performance**: Built with Rust for minimal overhead and instant execution.
- **Safety**: Optional dry-run mode to preview changes before they occur.
- **Automation**: Watch mode to continuously organize incoming files.
- **Smart Sorting**: Files are automatically categorized into logical groups (Images, Videos, Documents, Archives, Audio).

## License

This project is licensed under the [MIT license](LICENSE).
