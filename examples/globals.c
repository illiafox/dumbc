int g = 42;

int putchar(int ch); // std func

int square(int x) {
    return x * x;
}

int print_digit(int d) {
    return putchar('0' + d);
}

int main() {
    int i = 0, sum = 0;

    for (i = 1; i <= 5; i++) {
        int s = square(i);
        sum += s;
        print_digit(s / 10);
        print_digit(s % 10);
        putchar('\n');
    }

    // Bitwise and logical operations
    if ((sum & 1) == 0 && sum > 0) {
        putchar('E');
        putchar('v');
        putchar('e');
        putchar('n');
        putchar('\n');
    } else {
        putchar('O');
        putchar('d');
        putchar('d');
        putchar('\n');
    }

    int r;
    r = sum; g += 1;
    print_digit((r / 10) % 10);
    print_digit(r % 10);
    putchar('\n');

    return 0;
}
