int main() {
    int a, b, x, res, dummy;

    a = 10;
    b = 3;
    res = 0;

    // Unary
    res += -a;
    res += ~a + 256;  // keep positive
    res += !a;

    // Binary arithmetic
    res += a + b;
    res += a - b;
    res += a * b;
    res += a / b;
    res += a % b;

    // Bitwise
    res += a & b;
    res += a | b;
    res += a ^ b;
    res += a << 1;
    res += a >> 1;

    // Compound assignment
    x = 10;
    x += 2;  res += x;
    x -= 1;  res += x;
    x *= 2;  res += x;
    x /= 3;  res += x;
    x %= 5;  res += x;

    x = 5;
    x <<= 1; res += x;
    x >>= 1; res += x;
    x &= 3;  res += x;
    x |= 1;  res += x;
    x ^= 2;  res += x;

    // Logical
    res += a && b;
    res += a || b;

    // Comparison Operators
    res += (a == b);
    res += (a != b);
    res += (a < b);
    res += (a <= b);
    res += (a > b);
    res += (a >= b);

    // Comma
    a = 5;
    dummy = (a = 5);
    res += dummy;

    // Increment / Decrement
    a = 10;
    res += a;
    res += a; a++;
    res += a;
    res += a; a--;

    res = res - ((res / 256) * 256);
    return res;
}
