#include <stdio.h>
#include <stdlib.h>

int main() {
  char name[] = "World";
  int year = 2026;
  
  printf("\nHello, %s, this is %d!", name, year);
  
  printf("\nType your age: ");
  char buf[20];
  gets(buf, 19);
  
  printf("\n%%Your age is %x!", atoi(buf));
  
  return 0;
}
