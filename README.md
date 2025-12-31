# 🧤 Alfred

**A blazingly fast CLI tool to automatically organize your folders.**

Built with Rust for maximum performance. Organize hundreds of files in milliseconds.

![Demo](./alfred-clean.gif)

## ✨ Features

- 🚀 **Blazingly Fast** - Organizes 200+ files in under 100ms
- 📁 **Smart Sorting** - Automatically categorizes files by extension
- 👀 **Watch Mode** - Monitors folders for new files in real-time
- 🧪 **Dry Run** - Preview changes without moving files
- 🎨 **Beautiful CLI** - Colored output with progress indicators

## 📦 Installation

### From crates.io
```bash
cargo install alfred-cli
```

### From source
```bash
git clone https://github.com/gregcrn/alfred
cd alfred
cargo install --path .
```

## 🚀 Usage

```bash
# Clean your Downloads folder (default)
alfred clean

# Clean a specific folder
alfred --path ~/Documents clean

# Watch for new files
alfred watch

# Preview without moving files
alfred --dry-run clean

# Show help
alfred --help
```

## 📂 File Categories

| Category | Extensions |
|----------|------------|
| 📄 PDFs | `.pdf` |
| 🖼️ Images | `.jpg`, `.jpeg`, `.png`, `.gif`, `.webp` |
| 📦 Archives | `.zip`, `.tar`, `.gz`, `.rar`, `.7z` |
| 🎬 Videos | `.mp4`, `.mov`, `.avi`, `.mkv` |
| 📝 Documents | `.docx`, `.doc`, `.xlsx`, `.pptx`, `.txt` |
| 🎵 Music | `.mp3`, `.wav`, `.flac` |
| 📁 Others | Everything else |

## ⚡ Performance

Alfred is built with Rust, which means:
- **No runtime overhead** - Direct system calls
- **Zero garbage collection** - Predictable performance
- **Minimal memory usage** - Runs efficiently on any machine

## 🛠️ Development

```bash
# Clone the repo
git clone https://github.com/gregcrn/alfred
cd alfred

# Run in development
cargo run -- clean

# Build for release
cargo build --release
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 👤 Author

**Gregory Corin**
- GitHub: [@gregcrn](https://github.com/gregcrn)
- LinkedIn: [Gregory Corin](https://linkedin.com/in/gregorycorin)

---

Made with ❤️ and Rust 🦀
