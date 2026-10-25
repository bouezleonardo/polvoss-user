#include "stdio.h"

void dumb_recursive(int a) {
  printf("\nDumb recursive");
  if (a >= 0) {
    printf("\nTrying to overflow!");
    int page[1024];
    dumb_recursive(a);
  } else {
    printf("\nWhat?");
  }
}

void main() {
  dumb_recursive(1);
}
