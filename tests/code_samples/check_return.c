int is_neg(int x) { return x < 0 ? 1 : 0; }

int main() {
  is_neg(-1);

  int flag = is_neg(2);

  if (is_neg(4)) {
    flag = 1;
  }

  return 0;
}