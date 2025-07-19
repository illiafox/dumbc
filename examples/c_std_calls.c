int putchar(int c);
int sleep(int seconds);
int srand(int n);
int rand();
int abs(int n);
int isdigit(int c);

int main() {
  putchar(68);
  putchar(101);
  putchar(109);
  putchar(111);
  putchar(10);

  int zero_char = 48; // 0

  int i = 5;
  while (i > 0) {
    putchar(zero_char + i);
    putchar(10);
    sleep(1);
    i = i - 1;
  }

  srand(1234);
  int r = rand();
  int roll = (r % 6) + 1;

  putchar(zero_char + roll);
  putchar(10);

  int neg = 0 - roll;
  int pos = abs(neg);

  putchar(48 + pos);
  putchar(10);

  int ok = isdigit(zero_char + 7);
  if (ok) {
    putchar(89);
  } else {
    putchar(78);
  }
  putchar(10);

  return roll;
}