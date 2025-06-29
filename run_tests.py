import subprocess
import sys
from pathlib import Path

stages = [1, 2]
target_archs = ['arm64']
BASE = Path("testsuite")
any_failed = False

# colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
RESET = "\033[0m"


def run_test(file: Path, expect_success: bool, arch: str):
    global any_failed
    print(f"Testing {file}...", end=" ")
    result = subprocess.run(
        ["cargo", "run", "--quiet", "--", str(file), "--arch", arch],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    success = result.returncode == 0
    if expect_success and not success:
        print(f"{RED}ERROR: should succeed{RESET}")
        print(f"{result.stderr.decode()}")
        any_failed = True
    elif not expect_success and success:
        print(f"{RED}ERROR: should fail{RESET}")
        any_failed = True
    else:
        print(f"{GREEN}PASS{RESET}")


def main():
    for arch in target_archs:
        print(f"\n=== Architecture: {arch} ===")

        total = 0
        passed = 0
        failed = 0

        for stage in stages:
            stage_dir = BASE / f"stage_{stage}"
            valid = stage_dir / "valid"
            invalid = stage_dir / "invalid"

            for f in valid.glob("*.c"):
                total += 1
                if run_test(f, expect_success=True, arch=arch):
                    passed += 1
                else:
                    failed += 1

            for f in invalid.glob("*.c"):
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
            sys.exit(1)
        else:
            print(f"{GREEN}All tests passed.{RESET}")
            sys.exit(0)


if __name__ == "__main__":
    main()
