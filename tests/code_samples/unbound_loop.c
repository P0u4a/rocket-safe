int unbound_while() {
  int n = 0;

  while (1) {
    if (n >= 10) {
      break;
    }

    n += 1;
  }

  return n;
}

int unbound_for() {
  int n = 0;

  for (int i = 0; i != 4; i++) {
    --n;
  }

  return n;
}

int main() {
  int i = 0;

  while (i < 10) {
    ++i;
  }

  int n = 10;

  for (int i = 0;; i++) {
    if (i == 3) {
      break;
    }

    --n;
  }

  return 0;
}