int main() {
    int a = 2;
    int b = 3;
    {
        int a = 1;

        {
            int b = 14;
            b += b;
        }
        b = b + a;
    }

    {

            int a0 = 0;
            int a1 = 1;
            int a2 = 2;
            int a3 = 3;
            int a4 = 4;
            int a5 = 5;
            int a6 = 6;
            int a7 = 7;
            int a8 = 8;
            int a9 = 9;
            int a10 = 10;
            int a11 = 11;
            int a12 = 12;
            int a13 = 13;
            int a14 = 14;
            int a15 = 15;
            int w = 15;
    }

    if (b > 0) {
        int c = 14;
        c = 2;
        return c+12;
    }

    return b;
}