import subprocess
import sys
from pathlib import Path
import os

STAGES = [1, 2, 3]
TARGET_ARCHS = ['arm64']
BASE = Path("testsuite")

# colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
RESET = "\033[0m"

USE_GITHUB_FORMAT = os.getenv("GITHUB_ANNOTATIONS") == "1"


def run_test(file: Path, expect_success: bool, arch: str) -> bool:
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
        return False
    elif not expect_success and success:
        print(f"{RED}ERROR: should fail{RESET}")
        return False
    else:
        print(f"{GREEN}PASS{RESET}")
        return True


def main():
    for arch in TARGET_ARCHS:
        print(f"\n=== Architecture: {arch} ===")

        total = 0
        passed = 0
        failed = 0

        for stage in STAGES:
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
