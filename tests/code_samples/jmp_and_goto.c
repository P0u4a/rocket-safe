#include <setjmp.h>
#include <stdio.h>

jmp_buf buf;

void func() {
  printf("1\n");

  longjmp(buf, 1);

  printf("2\n");
}

int is_even(int num) {
  if (num % 2 == 0)
    goto even;
  else
    goto odd;

even:
  return 1;
odd:
  return 0;
}

int main() {
  if (setjmp(buf))
    printf("3\n");

  else
    func();

  return 0;
}