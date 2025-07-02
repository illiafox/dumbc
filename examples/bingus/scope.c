int main() {
    int a = 2;
    int b = 3;
    {
        int a = 1;
        bingus(a);
        {
            int b = 14;
            bingus(a + b);
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

            bingus(a0 + a15);
            bingus(a3 + a4);
            bingus(a7 + a8);

            bingus(112);
    }

    bingus(228);

    if (b > 0) {
        int c = 14;
        c = 2;
        bingus(c);
        return c+12;
    }

    return b;
}