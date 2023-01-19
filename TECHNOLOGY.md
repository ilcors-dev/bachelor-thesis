# Technology - A list of technologies used

## Rust 🦀
Rust is a low-level compiled programming language like C/C++. It is designed to be a systems programming language with a focus on safety, performance and concurrency.

## Wasm ⚙️
WebAssembly (WASM) is a binary instruction format for a stack-based virtual machine. It is designed as a portable target for the compilation of high-level languages like C, C++, and Rust, enabling deployment on the web for client and server applications.

It is a low-level assembly-like language with a compact binary format, that is designed to be fast to decode and execute. It is designed to be a safe and secure execution environment, providing a sandboxed environment for running code. This allows for faster and more efficient execution of code on the web, compared to traditional JavaScript.

The code is delivered in binary format, which is smaller and faster to download and parse than JavaScript. Additionally, the code can be precompiled, which allows for faster startup time and reduced memory usage.

## Wasi 🌐
Wasi is a system interface for WebAssembly.
It provides a set of APIs for interacting with the host operating system and allows WASM modules to access system resources such as the file system and network. WASI is designed to be platform-agnostic, meaning that it can be implemented on any operating system, and it is intended to be used in both web and non-web environments.

## Wasmtime 🕰
Wasmtime is a standalone, embeddable WebAssembly runtime. It is written in Rust and is developed by the [Bytecode Alliance](https://bytecodealliance.org/). Wasmtime provides a low-level and customizable API for executing WebAssembly (WASM) modules, and it can be integrated into various programming languages and environments.

It provides various features for managing memory and interacting with the host operating system, and it supports different execution modes (single threaded and multi threaded) and engines ([Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift) and Lightbeam). Wasmtime is suitable for use in a variety of contexts, such as cloud services, edge devices, and desktop applications.

It supports the WebAssembly System Interface (WASI), which enables interaction with the host operating system in a safe and secure way, allowing developers to access the filesystem, environment variables, and other system resources from within the WASM module.

## Wasmer 🧬
Wasmer is a standalone, embeddable WebAssembly runtime written in Rust. It is designed to provide a simple and easy-to-use API for running WebAssembly (WASM) modules and can be integrated into various programming languages and environments.

Wasmer is designed to be lightweight, fast, and efficient, and it supports multiple backends, such as Cranelift and [LLVM](https://llvm.org/) (as Wasmtime).

As Wasmtime, it supports WASI.

Wasmer also provides a number of features for managing memory, such as the ability to configure the amount of memory available to a WASM module and the ability to resize the memory at runtime.

## Wasmetime vs Wasmer

| Feature           | Wasmtime | Wasmer |
| ----------------- | -------- | ------ |
| WASI              | ✅        | ✅      |
| Memory management | ✅        | ✅      |
| Multi-threading   | ✅        | ✅      |
| Cranelift         | ✅        | ✅      |
| Lightbeam         | ✅        | ❌      |
| LLVM              | ❌        | ✅      |
| WASI              | ✅        | ✅      |
| Headless          | ✅        | ❌      |

The [wasmer docs](https://wasmer.io/wasmer-vs-wasmtime) say the following:
- wasmer is faster than wasmtime during startup (~1000x)
- wasmer is faster than wasmtime during execution (~2x)

  _Wasmer is designed from the ground up to match the speed of native executables thanks to our LLVM integration. Experience startup performance unlike any other._
- it supports other compilers other than cranelift
- integrates with more languages

    | Language     | Wasmtime | Wasmer |
    | ------------ | -------- | ------ |
    | Go           | ✅        | ✅      |
    | Python       | ✅        | ✅      |
    | Ruby         | ❌        | ✅      |
    | PHP          | ❌        | ✅      |
    | Java         | ❌        | ✅      |
    | C#           | ❌        | ✅      |
    | C            | ✅        | ✅      |
    | C++          | ❌        | ✅      |
    | Rust         | ✅        | ✅      |
    | Javascript   | ❌        | ✅      |
    | R            | ❌        | ✅      |
    | Elixir       | ❌        | ✅      |
    | Postgres (?) | ❌        | ✅      |
    | Ruby         | ❌        | ✅      |

### Considerations
- It actually seems that wasmer is a little more mature than wasmtime.

  Github stats at time of writing (2023-01-13):

  | Github  | Wasmtime | Wasmer |
  | ------- | -------- | ------ |
  | Stars   | 11.2k    | 14.2k  |
  | Commits | 10724    | 13428  |

- Wasmtime is developed by the bytecode allience which is a non-profit behind the actual WASI standard which is backed by [Fastly](https://www.fastly.com/)

### Links about differences between wasmtime and wasmer
- https://news.ycombinator.com/item?id=27540994