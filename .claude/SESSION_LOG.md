# Vilik Rust OS - Session Log

## 2024-12-02 - Project Initialization & UART

### What was done
- Created `.claude/` directory with project instructions
- Installed Rust nightly toolchain (1.93.0-nightly)
- Added `aarch64-unknown-none` target
- Added `rust-src` component for build-std
- Created project structure:
  - `Cargo.toml` - project manifest
  - `.cargo/config.toml` - build configuration (target, linker flags, build-std)
  - `linker.ld` - linker script (copied from C version, added `.text._start` section)
  - `boot/boot.S` - assembly startup code (copied from C version)
  - `src/main.rs` - kernel entry point with panic handler
  - `src/uart.rs` - UART driver (putc, puts, getc, gets)
- Successfully built and tested in QEMU
- Fixed UART address bug: `0x9000_0000` -> `0x0900_0000`

### Current state
- [x] Rust nightly toolchain installed
- [x] aarch64-unknown-none target added
- [x] Cargo.toml and project structure
- [x] Linker script
- [x] Main source file (src/main.rs)
- [x] Build and QEMU test
- [x] UART driver - putc, puts, getc, gets all working

### Project structure
```
vilik-rust/
├── .cargo/
│   └── config.toml
├── boot/
│   └── boot.S
├── src/
│   ├── main.rs
│   └── uart.rs
├── Cargo.toml
├── linker.ld
└── README.md
```

### Key learnings
- Linker script needs `.text._start` section to ensure `_start` is at beginning
- UART address for QEMU virt: `0x0900_0000` (not `0x9000_0000`!)
- `#[no_mangle]` needed for functions called from assembly
- `unsafe` can be inside function or function itself can be `unsafe`
- `b'H'` is byte literal (u8), `'H'` is char (4 bytes Unicode)
- Last expression without semicolon = return value
- UART FR register: RXFE (bit 4) for RX empty, TXFF (bit 5) for TX full

### Next steps
- Implement shell with command parsing
- Panic handler with proper output
- Memory management (mm module)

---

## 2025-12-06 - printk & Test Framework

### What was done
- Implemented `printk!` macro using `heapless::String` and `core::fmt`
- Added `rust-toolchain.toml` to enforce nightly toolchain
- Created test framework with Cargo feature flag:
  - `src/test/mod.rs` - test runner with `run_all()`
  - `src/test/printk.rs` - printk tests (all integer types, floats, bool, strings, arrays, tuples)
  - Tests are conditionally compiled with `#[cfg(feature = "test")]`
  - Build without tests: `cargo build --release`
  - Build with tests: `cargo build --release --features test`

### Current state
- [x] UART driver (putc, puts, getc, gets)
- [x] printk! macro
- [x] Test framework with feature flag
- [x] rust-toolchain.toml for nightly

### Project structure
```
vilik-rust/
├── .cargo/
│   └── config.toml
├── .claude/
│   ├── instructions.md
│   └── SESSION_LOG.md
├── boot/
│   └── boot.S
├── src/
│   ├── main.rs
│   ├── printk.rs
│   ├── uart.rs
│   └── test/
│       ├── mod.rs
│       └── printk.rs
├── Cargo.toml
├── linker.ld
├── rust-toolchain.toml
└── README.md
```

### Key learnings
- `heapless::String<N>` for stack-allocated strings in no_std
- `core::fmt::Write` trait for formatted output
- Cargo features for conditional compilation: `#[cfg(feature = "...")]`
- `rust-toolchain.toml` ensures correct toolchain without `+nightly` flag

### Next steps
- Implement shell with command parsing
- Panic handler with proper output
- Memory management (frame allocator)
