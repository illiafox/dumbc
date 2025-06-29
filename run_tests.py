import subprocess
import sys
from pathlib import Path

stages = [1]
BASE = Path("testsuite")
any_failed = False

# colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
RESET = "\033[0m"

def run_test(file: Path, expect_success: bool):
    global any_failed
    print(f"Testing {file}...", end=" ")
    result = subprocess.run(
        ["cargo", "run", "--quiet", str(file)],
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
    for stage in stages:
        stage_dir = BASE / f"stage_{stage}"
        valid = stage_dir / "valid"
        invalid = stage_dir / "invalid"

        for f in valid.glob("*.c"):
            run_test(f, expect_success=True)
        for f in invalid.glob("*.c"):
            run_test(f, expect_success=False)

    print("\nSummary:")
    if any_failed:
        print("Some tests failed.")
        sys.exit(1)
    else:
        print("All tests passed.")
        sys.exit(0)

if __name__ == "__main__":
    main()
