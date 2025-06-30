int main()
{
    int a0  =  1;
    int a1  =  2;
    int a2  =  3;
    int a3  =  4;
    int a4  =  5;
    int a5  =  6;
    int a6  =  7;
    int a7  =  8;
    int a8  =  9;
    int a9  = 10;
    int a10 = 11;
    int a11 = 12;
    int a12 = 13;
    int a13 = 14;
    int a14 = 15;
    int a15 = 16;
    int a16 = 17;
    int a17 = 18;
    int a18 = 19;
    int a19 = 20;

    int t0  = 0;
    int t1  = 0;
    int t2  = 0;
    int result = 0;

    t0 = (a0 + a1) * (a2 - a3);
    a4 = a4 + a5;
    a5 = a5 / 2;
    a6 = (a6 + a7) * (a8 - a9);
    a10 = (a10 + (a11 * a12)) / (a13 - 1);
    a14 = (a14 - a15) + (a16 * a17);
    a18 = (a18 + a19) / 3;

    t1 = (a2 > a3) && (a4 < a5);
    t2 = (a6 >= a10) || (a14 <= a18);

    result = t0 + t1 + t2;

    result = a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9
            + a10 + a11 + a12 + a13 + a14 + a15 + a16 + a17 + a18 + a19 + result;

    return result;
}
