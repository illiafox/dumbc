import argparse
import difflib
import os
import subprocess
import sys
from pathlib import Path

STAGES = [1, 2, 3, 4, 5, 6, 7]
TARGET_ARCHS = ["aarch64"]
BASE = Path("testsuite")

# colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
RESET = "\033[0m"

USE_GITHUB_FORMAT = os.getenv("GITHUB_ANNOTATIONS") == "1"
CROSS_COMPILE = os.getenv("CROSS_COMPILE") == "1"


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--arch-style",
        choices=["mac", "gnu"],
        default="mac",
        help="Use 'mac' for `-arch arm64` (macOS), 'gnu' for `--target=aarch64-linux-gnu` (Linux)",
    )
    parser.add_argument(
        "--disable-compile",
        action="store_true",
        help="Disable compilation and output comparison",
    )
    return parser.parse_args()


args = parse_args()

COMPILE_DISABLED = args.disable_compile
ARCH_STYLE = args.arch_style


def build_clang_command(source: Path, output: Path, arch: str, style: str) -> list[str]:
    cmd = ["clang"]
    if style == "mac":
        cmd += ["-arch", "arm64"]
    elif style == "gnu":  # assume GNU
        cmd += [f"--target={arch}-linux-gnu"]
    else:
        raise ValueError(f"Unsupported arch style: {style!r}. Expected 'mac' or 'gnu'.")
    cmd += ["-o", str(output), str(source)]
    return cmd


def compile_and_run(source: Path, output: Path, arch: str) -> tuple[str, int]:
    # Compile source to output binary
    clang_cmd = build_clang_command(source, output, arch, ARCH_STYLE)

    try:
        subprocess.run(
            clang_cmd,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except subprocess.CalledProcessError as e:
        print(f"{RED}Compilation failed:{RESET}")
        print(f"Command: {' '.join(clang_cmd)}")
        print(f"stderr:\n{e.stderr.decode().strip()}")
        raise

    # Run the compiled binary
    run_cmd = ["qemu-aarch64", str(output)] if CROSS_COMPILE else [str(output)]
    result = subprocess.run(
        run_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
    )
    return result.stdout.strip(), result.returncode


def compare_c_and_s_outputs(c_file: Path, s_file: Path, arch: str = "arm64") -> bool:
    if not c_file.exists() or not s_file.exists():
        print(f"{RED}Missing file(s): {c_file}, {s_file}{RESET}")
        return False

    base = c_file.stem
    bin_c = c_file.with_name(f"{base}.c.bin")
    bin_s = s_file.with_name(f"{base}.s.bin")

    try:
        output_c, code_c = compile_and_run(c_file, bin_c, arch)
        output_s, code_s = compile_and_run(s_file, bin_s, arch)
    except subprocess.CalledProcessError as e:
        print(f"{RED}Compilation failed: {e}{RESET}")
        return False
    finally:
        for bin_file in (bin_c, bin_s):
            if bin_file.exists():
                try:
                    bin_file.unlink()
                except Exception as e:
                    print(f"{RED}Failed to delete {bin_file}: {e}{RESET}")

    match_output = output_c == output_s
    match_code = code_c == code_s

    if match_output and match_code:
        print(f"{GREEN}PASS (+ MATCH){RESET}")
        return True

    print(f"{RED}MISMATCH DETECTED{RESET}")
    if not match_code:
        print(f"Return codes differ: C={code_c}, ASM={code_s}")
    if not match_output:
        print("--- C output ---")
        print(output_c)
        print("--- S output ---")
        print(output_s)
        print("--- Diff ---")
        diff = difflib.ndiff(output_c.splitlines(), output_s.splitlines())
        print("\n".join(diff))
    return False


def run_test(c_file: Path, expect_success: bool, arch: str) -> bool:
    print(f"Testing {c_file}...", end=" ")
    result = subprocess.run(
        ["cargo", "run", "--quiet", "--", str(c_file), "--arch", arch],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    success = result.returncode == 0
    if expect_success and not success:
        if c_file.name.startswith("skip_on_failure"):
            print(f"{YELLOW}SKIPPED (skip_on_failure){RESET}")
            return True

        print(f"{RED}ERROR: should succeed{RESET}")
        print(f"{result.stdout.decode()}")
        print(f"{result.stderr.decode()}")
        return False
    elif not expect_success and success:
        print(f"{RED}ERROR: should fail{RESET}")
        return False
    else:
        if expect_success and not COMPILE_DISABLED:
            s_file = c_file.with_suffix(".s")
            return compare_c_and_s_outputs(c_file, s_file, arch)
        print(f"{GREEN}PASS{RESET}")
        return True


def main():
    for arch in TARGET_ARCHS:
        print(f"\n=== Architecture: {arch} ===")

        total = 0
        passed = 0
        failed = 0

        examples_dir = Path("examples")
        for f in examples_dir.glob("*.c"):
            total += 1
            if run_test(f, expect_success=True, arch=arch):
                passed += 1
            else:
                failed += 1

        for stage in STAGES:
            stage_dir = BASE / f"stage_{stage}"
            valid = stage_dir / "valid"
            invalid = stage_dir / "invalid"

            for f in valid.glob("**/*.c"):
                total += 1
                if run_test(f, expect_success=True, arch=arch):
                    passed += 1
                else:
                    failed += 1

            for f in invalid.glob("**/*.c"):
                total += 1
                if run_test(f, expect_success=False, arch=arch):
                    passed += 1
                else:
                    failed += 1

        print("\nSummary:")
        if total == 0:
            print(f"{RED}ERROR: No test files found.{RESET}")
            sys.exit(1)

        print(f"Total: {total}, Passed: {passed}, Failed: {failed}")
        if failed:
            print(f"{RED}Some tests failed.{RESET}")
            if USE_GITHUB_FORMAT:
                print(f"::error ::{failed} of {total} tests failed.")
            sys.exit(1)
        else:
            print(f"{GREEN}All tests passed.{RESET}")
            if USE_GITHUB_FORMAT:
                print("::notice ::All tests passed successfully.")
            sys.exit(0)


if __name__ == "__main__":
    main()
