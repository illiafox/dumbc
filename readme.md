# dumbc — Dumb C Compiler

> A small C compiler written for learning.

Built by following [Nora Sandler’s "Write a Compiler"](https://norasandler.com/2017/11/29/Write-a-Compiler.html) series.

---

## Supported Architectures

- **arm64** (AArch64 only)

---

## Implemented Parts

- [x] Part 1: Compile `int main() { return <int>; }`
- [x] Part 2: Add unary operators (`-`, `~`, `!`)
- [ ] Part 3: Add binary operators (`+`, `-`, etc.)
- [ ] Part 4+: Control flow, variables, types...

---

## Usage

### Compile and run a `.c` file:

```bash
cargo run -- path/to/file.c [--arch arm64]


- If --arch is not specified, the system architecture is used.
- Only arm64 is supported now

### Example:

```bash
cargo run -- tests/return_42.c
```

Produces `return_42.s`

You can compile it with aarch64 GCC or run with QEMU.

Example for macOS:
```bash
clang -arch arm64 -o return_42_mac return_42.s
./return_42
echo $?  # prints: 42
```

## License

This compiler is licensed under the **GNU General Public License**, as described in the `LICENSE` file.
