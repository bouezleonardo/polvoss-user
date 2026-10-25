#include "stdio.h"
#include "unistd.h"
#include <stddef.h>

void main() {  
  while (1) {
    printf("\nFork!");
    pid_t pid = fork();
    if (pid != 0) {
      wait(NULL);
      break;
    } else if (pid == -1) {
      break;
    }
  }
  printf("\nLEAVING!");
}
