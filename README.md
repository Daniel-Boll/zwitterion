# <h1 align="center">Zwitterion</h1>

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

Welcome to **Zwitterion**, a specialized language interpreter implemented in Rust. This project is part of the [Rinha de Compiler](https://github.com/aripiprazole/rinha-de-compiler) event. Named after _"zwitterions"_, molecules that maintain both positive and negative charges while being overall neutral, **Zwitterion** aims to strike a similar balance of robust functionality and high performance in the world of language interpretation. As of now, it serves as a tree-walking parser, with future plans to implement advanced compiler optimizations.

## Installation

### Prerequisites

- Ensure Rust and Cargo are installed. Download them from [rustup.rs](https://rustup.rs/).

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/Daniel-Boll/Zwitterion.git
   ```
2. Navigate to the project directory:
   ```bash
   cd Zwitterion
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Usage

To run **Zwitterion**, execute:

```bash
cargo run # Or if you have `just` you can `just run`
```

## Roadmap

- [x] Tree-Walking Parser
- [ ] Symbol Table and Scope Management
- [ ] AST Optimization [[lalrpop](https://github.com/lalrpop/lalrpop) + [logos](https://github.com/maciejhirsz/logos) combo]
- [ ] Intermediate Representation
- [ ] Compilation via Cranelift (Future Goal)

## License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.
