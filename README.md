# VilikOS - Rust Edition

Educational ARM64 operating system written in Rust. Parallel project to the C version - learning OS development and Rust simultaneously.

## Target Platform

- **Architecture**: ARM64 (AArch64)
- **Emulation**: QEMU (`qemu-system-aarch64`, virt machine, cortex-a53)
- **Toolchain**: Rust nightly, target `aarch64-unknown-none`

## Prerequisites

```bash
# Install Rust nightly
rustup toolchain install nightly

# Add ARM64 bare-metal target
rustup target add aarch64-unknown-none --toolchain nightly

# Add rust-src for build-std
rustup component add rust-src --toolchain nightly

# QEMU for testing
brew install qemu  # macOS
```

## Build

```bash
cargo +nightly build          # debug build
cargo +nightly build --release # release build
```

## Run in QEMU

```bash
qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/release/vilikOS
```

Exit QEMU: `Ctrl+A` then `X`

## Project Structure

```
vilik-rust/
├── .cargo/
│   └── config.toml    # Build config (target, linker flags, build-std)
├── boot/
│   └── boot.S         # Assembly startup (stack, FPU, BSS, jump to kernel)
├── src/
│   ├── main.rs        # Kernel entry point
│   └── uart.rs        # UART driver (PL011)
├── Cargo.toml         # Project manifest
├── linker.ld          # Memory layout for QEMU virt machine
└── README.md
```

## Memory Map (QEMU virt)

| Address        | Description          |
|----------------|----------------------|
| `0x0900_0000`  | PL011 UART           |
| `0x4008_0000`  | Kernel load address  |

## Current Status

- [x] Basic project setup
- [x] Boot code (stack setup, FPU init, BSS clearing)
- [x] Kernel boots in QEMU
- [x] UART driver (putc, puts)
- [x] Console output working
- [ ] Memory management
- [ ] Interrupts
