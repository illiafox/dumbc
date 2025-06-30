int main() {
    int a, b, x, res, dummy;

    a = 10;
    b = 3;
    res = 0;

    // Unary
    res = res + (-a);
    res = res + (~a + 256);  // to keep positive
    res = res + (!a);        // a is non-zero → 0

    // Binary arithmetic
    res = res + (a + b);
    res = res + (a - b);
    res = res + (a * b);
    res = res + (a / b);
    res = res + (a % b);

    // Bitwise
    res = res + (a & b);
    res = res + (a | b);
    res = res + (a ^ b);
    res = res + (a * 2);     // shift left
    res = res + (a / 2);     // shift right

    // Assignment compound
    x = 10;
    x = x + 2; res = res + x;
    x = x - 1; res = res + x;
    x = x * 2; res = res + x;
    x = x / 3; res = res + x;
    x = x % 5; res = res + x;
    x = 6 & 7; res = res + x;
    x = 2 | 1; res = res + x;
    x = 3 ^ 1; res = res + x;
    x = 4 * 2; res = res + x;
    x = 8 / 2; res = res + x;

    // Logical (simulate with arithmetic)
    res = res + (1 * 0);     // true && false = 0
    res = res + (1 + 0);     // true || false = 1

    // Comparison (simulate as 0/1 constants — no ifs)
    res = res + 0; // a == b
    res = res + 1; // a != b
    res = res + 0; // a < b
    res = res + 0; // a <= b
    res = res + 1; // a > b
    res = res + 1; // a >= b

    // Comma
    a = 5;
    dummy = a + 1; // simulate (a = 5, a + 1)
    res = res + dummy;

    // ++ and --
    a = 10;
    a = a + 1; res = res + a;
    res = res + a; a = a + 1;
    a = a - 1; res = res + a;
    res = res + a; a = a - 1;

    // Simulate: res = res % 256 without loops
    res = res - ((res / 256) * 256);

    return res;
}
