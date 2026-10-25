#include "unistd.h"
#include "stdio.h"
#include "string.h"
#include <stddef.h>

void spawn(const char *path, const char **argv) {
  pid_t pid = fork();
      
  if(pid <= -1) {
    printf("\nFailed to fork the process.");
  } else if(pid == 0) {
    execv(path, argv);
    printf("\nUnable to run program: %s", path);
    exit(EXIT_FAILURE);
  } else {
    waitpid(pid, NULL);
  }
}

int main(int argc, char **argv) {
  char input[128];
  
  printf("\n");
  
  // Main loop
  while (1) {
    printf("$ ");
    gets(input, 128); // Read input
    
    // Interpret the input
    if (strcmp(input, "clear") == 0) {
      printf("\x1B[2J");
    } else if (strcmp(input, "exit") == 0) {
      break;
    } else if (input[0] == '.') {
      // Execute a program
      spawn(input+1, NULL);
      printf("\n");
    } else {
      printf("\nUnknown command.\n");
    }
  }
  
  return EXIT_SUCCESS;
}
