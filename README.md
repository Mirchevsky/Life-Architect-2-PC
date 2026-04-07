# Life Architect 2 PC

**Level up your life.**

![License](https://img.shields.io/badge/License-Apache%202.0-dea584?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-1.87-dea584?style=flat-square&logo=rust&logoColor=white)
![Iced](https://img.shields.io/badge/Iced-0.14-dea584?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-dea584?style=flat-square)

**Life Architect 2 PC** is a native Windows desktop application designed to transform personal development into an engaging and rewarding journey. By applying principles of gamification, the app helps users define their goals, break them down into actionable tasks, and track their progress through a system of experience points (XP), levels, and data. It serves as an efficient tool for anyone looking to build habits, learn new skills, and systematically construct a better life.

This repository contains the **100% Rust** desktop port of the original [Life Architect 2 Android app](https://github.com/Mirchevsky/Life-Architect-2).

## Features

The application is in active development. The current feature set provides the core engine for the gamified task management system on Windows.

| Feature | Status | Description |
| --- | --- | --- |
| **Task Management** | ✅ Complete | Create, track, and complete tasks. |
| **Gamification Engine** | ✅ Complete | Earn XP for completing tasks. |
| **Leveling System** | ✅ Complete | Progress through ranks. |
| **Elm Architecture** | ✅ Complete | A scalable and testable state management architecture using Iced. |
| **Offline-First Storage** | ✅ Complete | All data is stored locally using SQLite for instant access. |
| **Analytics & Insights** | 🚧 In Progress | Visualize progress with charts and historical data. |
| **Trending Feed** | 🚧 Planned | A daily feed of trending topics. |
| **Cloud Sync** | 🚧 Planned | Sync progress across devices with cloud backup. |

## Tech Stack & Architecture

This project is a case study in modern, native Windows desktop development, emphasizing best practices, pure native performance, and a clean, scalable architecture without relying on web technologies.

*   **100% Rust** — The entire codebase is written in Rust, ensuring memory safety, zero-cost abstractions, and blazing-fast performance.

*   **Iced GUI Framework** — The UI is built entirely with [Iced](https://github.com/iced-rs/iced), a cross-platform GUI library for Rust focused on simplicity and type safety. No JavaScript, no HTML, no WebView, no Electron.

*   **SQLite Database** — A local SQLite database for offline-first data persistence, using the `rusqlite` crate with the `bundled` feature to compile the database engine directly into the executable.

*   **Elm Architecture** — A unidirectional data flow pattern (State → Message → Update → View) that ensures a predictable and debuggable application state.

*   **Single Executable** — The build process produces a single, standalone `.exe` file with no external DLL dependencies, making distribution and installation effortless.

## Building from Source

Requirements: Rust stable toolchain + MSVC Build Tools (Windows)

```powershell
git clone https://github.com/Mirchevsky/Life-Architect-2-PC.git
cd Life-Architect-2-PC
cargo build --release
```

The compiled standalone executable will be located at `target\release\life-architect-2-pc.exe`.

## Contributing

This is an open-source project and contributions are welcome. Whether you are fixing a bug, implementing a planned feature, or improving the documentation, feel free to open an issue or submit a pull request.
## License

This project is licensed under the **Apache License 2.0**.
