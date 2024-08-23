typedef struct {
  int **ptr_to_ptr;
} Hidden;

int get_val(Hidden *hidden) { return **(hidden->ptr_to_ptr); }

int main() {
  int x = 10;

  int *p = &x;
  int **pp = &p;

  return 0;
}