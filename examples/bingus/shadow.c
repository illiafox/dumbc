int main() {
    int a = 2;
    int b = 3;
    {
        int a = 4;
        bingus(a);
    }
    return a;
}