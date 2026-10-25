#include "string.h"

int strlen(const char* s) {
  int len = 0;
  // While \0 is not reached
  while(s[len++] != '\0');
  return len-1;
}

int strcmp(const char* s1, const char* s2) {
  int i = 0;
  // Compare until \0 is reached or when the strings diverge
  while(s1[i] == s2[i] && s1[i] != '\0' && s2[i] != '\0'){
    ++i;
  }
  if (s1[i] != s2[i]) {
    return s1[i] - s2[i];
  }
  return 0;
}
