# dumbc — small C Compiler

> A small C compiler written for learning.

Built by following [Nora Sandler’s "Write a Compiler"](https://norasandler.com/2017/11/29/Write-a-Compiler.html) series.

[![Tests](https://github.com/illiafox/dumbc/actions/workflows/test.yaml/badge.svg)](https://github.com/illiafox/dumbc/actions/workflows/test.yaml)

---

## Supported Architectures

- **arm64** (darwin and linux only)

---

## Implemented Parts

- [x] Part 1: Compile `int main() { return <int>; }`
- [x] Part 2: Add unary operators (`-`, `~`, `!`)
- [x] Part 3: Add binary operators (`+`, `-`, etc.)
- [x] Part 4: Even More Binary Operators (`&&`, `||`, `==`, `!=`, `<`, `<=`, `>`, `>=`)
- [x] Part 4.1: Other Binary Operators (`%`, `&`, `|`, `^`, `<<`, `>>`)
- [x] Part 5: Local Variables
- [x] Part 5.1: 
  - Compound Assignment Operators (`+=`,`-=`,`/=`,`*=`,`%=`,`<<=`,`>>=`,`&=`,`|=`,`^=`)
  - Increment/Decrement(`++`/`--`)
  - Comma operators (`int a = 2, b, c = a + 4;`)
- [x] Part 6: Conditionals
- [x] Part 7: Compound Statements
- [x] Part 8: Loops
- [x] Part 9: Functions (with calls to the C standard library!)
- [ ] Part 10: Global Variables

---

## Usage

### Compile and run a `.c` file:

```bash
cargo run -- path/to/file.c [--arch arm64] [--platform macos|linux]
```

- If `--arch` is not specified, the system architecture is used. Same for `--platform`
- Only arm64 is supported now

### Example:

```bash
cargo run -- examples/return_42.c
```

Produces `return_42.s`

You can compile it with aarch64 GCC or run with QEMU.

Example for macOS:
```bash
clang -arch arm64 -o return_42_mac return_42.s
./return_42
echo $?  # prints: 42
```

## Non-standard features

### `bingus` keyword
`bingus(arg)` is a built-in pseudo-function, which evaluates expression and prints it into stdout. Available on macOS platform

**DEPRECATED**: calls to the standard library functions are supported

Example:
```c
int main() {
    int a = 10, b = 8;
    bingus(a);
    bingus(a * b);
    bingus(b - 10);
}
```
Output:
```
10
80
-2
```

## Tests
The `testsuite` submodule contains tests cases from [`nlsandler/write_a_c_compiler`](https://github.com/nlsandler/write_a_c_compiler). 

To run them, use
```bash
python run_tests.py
```

## License

This compiler is licensed under the **GNU General Public License**, as described in the `LICENSE` file.
