# 🦀 My Rust Learning Journey

This repository is my personal learning diary as I explore Rust, a programming language focused on safety, concurrency, and performance. Here, I will store all the code I develop throughout my introductory course on Rust.

## 🚀 Getting Started

This course is a personal journey to master Rust, using Visual Studio Code as my primary editor.
```bash
git clone https://github.com/Sastre113/rust-fundamentals.git
cd rust-fundamentals
```

This will download all the course materials to your local machine, allowing you to start learning and experimenting with Rust right away.

### Compiling and Running Rust Programs on Windows 10

Rust uses `cargo`, its package manager and build system, to manage projects, compile, and run programs. Here's a step-by-step guide to get you started:

### Installing Rust on Windows 10

1. Download and install `rustup` by visiting [https://rustup.rs/](https://rustup.rs/). `rustup` is Rust's official installer.
2. Follow the installer instructions. It will install the Rust compiler (`rustc`), the Rust package manager (`cargo`), and the standard library.

### Creating a New Rust Project

To create a new project, open your Command Prompt (cmd) or PowerShell and run:

```cmd
cargo new my_project
cd my_project
```
This creates a new directory named my_project with a basic Rust project setup.

### Compiling Your Rust Program  

To compile your program, ensure you are in the project directory (where the Cargo.toml file is located) and run:
```cmd
cargo build
```

### Running Your Rust Program  
To compile (if necessary) and run your Rust program, use:
```cmd
cargo run
```
This will execute the main program of your project.

### Checking Your Code for Errors Without Compiling  
If you wish to check your code for errors without compiling, you can use:  
cargo check
```cmd
cargo check
```
This is a faster way to catch errors early in the development process.

### Building for Release
When your project is ready for deployment, compile it with optimizations by running:

```cmd
cargo build --release
```
The optimized executable will be located in the target/release directory.

## 🎯 Goals

- Learn and apply Rust's fundamental concepts.
- Practice writing safe and efficient code.
- Explore Rust's unique features, such as ownership and concurrency.

## 📖 Resources Used

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get used to reading and writing Rust code.

## 💡 Projects

This repository will include individual projects and exercises I undertake, each in its respective directory.

## 🛠 Tools

![Visual Studio Code](https://img.shields.io/badge/editor-VS%20Code-blue.svg) - My editor of choice for this Rust journey.

## 🤝 Contributions

Although this is a personal learning project, any feedback or suggestions are welcome.

## 📄 License

This project is under the MIT License - see the [LICENSE](LICENSE) file for details.
