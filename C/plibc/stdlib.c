#include "stdlib.h"

int atoi(const char *s) {
  int i = 0;  
  
  // Ignore white space
  while (s[i]=='\f'||s[i]=='\n'||s[i] =='\r'||s[i]=='\t'||s[i]=='\v')
    ++i;
  
  // Is negative
  int is_neg = 0;
  
  // Ignore the sign
  if (s[i] == '-'|| s[i] == '+') {
    i += 1;
    is_neg = 1;
  }
  
  int num = 0;
  while (s[i] != '\0') {
    // Advance the decimal place
    num *= 10;
    // Get the number from the character
    num += s[i]-'0';
    ++i;
  }
  
  if (is_neg)
    num *= -1;
  
  return num;
}
