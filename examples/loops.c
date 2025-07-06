int main() {
    int a = 1, b, d = a + 5;
    a++;
    a++;
    a--;
    d--;

    // 1. for loop with initializer only
    for (int a = 2;;) {
        d += a; // d = d + 2
        break;
    }

    // 2. infinite loop with break
    for (;;) {
        d += 3; // d = d + 3
        break;
    }

    // 3. for loop with init and increment
    for (d += 4;; d += 1) {
        d += 1; // d = d + 1 (one-time)
        break;
    }

    // 4. standard loop
    for (int i = 0; i < 2; i++) {
        d += i; // d = d + 0, then +1
    }

    // 5. empty-body loop (does nothing visible)
    for (int j = 0; j < 2; j++)
        ; // just loops, no effect

    // 6. controlled by external variable
    int x = 0;
    for (; x < 2;) {
        d += x; // d = d + 0, then +1
        x++;
    }

    return d;
}
