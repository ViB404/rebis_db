# 🚀 RebisDB

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-Async_Runtime-green.svg)](https://tokio.rs/)

**A lightweight, high-performance key-value store implemented in Rust**

</div>

## 📋 Overview

RebisDB is a minimalist yet powerful key-value store inspired by Redis, built from the ground up in Rust. It provides a simple TCP-based interface for storing, retrieving, and managing string data with persistence capabilities. The project demonstrates the power of Rust's performance combined with the simplicity of a Redis-like command structure.

## ✨ Features

- **🔄 Asynchronous Architecture**: Built on Tokio for non-blocking I/O operations
- **🔌 TCP REPL Interface**: Simple command-line interface over TCP
- **💾 Persistent Storage**: Automatic saving to disk ensures data durability between restarts
- **⚡ Lightweight & Fast**: Minimal dependencies with focus on performance
- **🧩 Simple Command Set**: Easy-to-remember commands for basic operations

## 🛠️ Installation

### Prerequisites

- Rust and Cargo (1.70 or newer)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/ViB404/rebis_db.git
cd rebis_db

# Build and run
cargo run
```

The server will start on `127.0.0.1:6379` by default.

## 📝 Commands

| Command | Format | Description |
|---------|--------|-------------|
| GET | `GET <key>` | Retrieve the value for the specified key |
| SET | `SET <key> <value>` | Store a key-value pair |
| DEL | `DEL <key>` | Remove a key-value pair |

## 💻 Usage Example

Connect to the server using any TCP client (like netcat):

```bash
nc 127.0.0.1 6379
```

Example session:

```
>> SET user:1 "Jane Smith"
OK
>> GET user:1
Jane Smith
>> SET counter 42
OK
>> GET counter
42
>> DEL user:1
OK
>> GET user:1
(nil)
```

## 🏗️ Architecture

RustyDB uses a simple but effective architecture:

```
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  TCP Server   │──────▶  Command      │──────▶  Storage      │
│  (Tokio)      │      │  Processor    │      │  Engine       │
└───────────────┘      └───────────────┘      └───────┬───────┘
                                                      │
                                                      ▼
                                              ┌───────────────┐
                                              │  JSON         │
                                              │  Persistence  │
                                              └───────────────┘
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Tokio Documentation](https://tokio.rs/docs/)
- [Redis Command Reference](https://redis.io/commands) (for inspiration) (not properly used in this)

---

<div align="center">

Made with ❤️ and 🦀 Rust by [ViB](https://devvib.vercel.app)

</div>
