# ROS2 Launch Language Server (LSP)

![ROS2](https://img.shields.io/badge/ROS2-Humble%7CIron-blue)
![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange)
![Python](https://img.shields.io/badge/Python-3.8%2B-blue)

A Language Server Protocol implementation for ROS2 Python launch files (`*.launch.py`) providing:

- 🚀 Intelligent code completion
- 🔍 Go-to-definition navigation
- ❌ Syntax error detection
- 📝 Documentation hints

## 📂 Project Structure

```text
ros2-launch-lsp/
├── lsp_server/              # Core LSP implementation
│   ├── scripts/             # Python parsing scripts
│   │   └── parser_launch.py # Launch file AST parser
│   └── src/
│       ├── main.rs          # Server entry point
│       ├── python_parser.rs # Rust-Python bridge
│       └── server.rs        # LSP protocol handlers
├── my_rust_lib/             # Supporting library
├── test_files/              # Example launch files
└── venv/                    # Python virtualenv
