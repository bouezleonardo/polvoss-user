#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
  printf("\nsbrk: %u", sbrk(0));
  
  extern char euser;
  printf("\nEnd addr: %u", &euser);
  
  int* ints = (int*)malloc(10*sizeof(int));
  printf("\nAllocated %d. The ints is in %u", 10*sizeof(int), ints);
  printf("\nNew sbrk: %u", sbrk(0));
  
  char* chars = (char*)malloc(21*sizeof(char));
  printf("\nAllocated %d. The chars is in %u", 21*sizeof(char), chars);
  printf("\nNew sbrk: %u", sbrk(0));
  
  printf("\nints = (");
  for(int i = 0;i < 10;++i) {
    printf("%d, ", ints+i);
  }
  printf(")");
  
  printf("\nchars = (");
  for(int i = 0;i < 21;++i) {
    printf("%d, ", chars+i);
  }
  printf(")");
  
  return 0;
}
