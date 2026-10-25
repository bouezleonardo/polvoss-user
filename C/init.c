/**
 * @file init.c 
 * @brief The init process is responsible for initiliazing
 * the shell and "adopting" orfan child processes.
 *
 * @author Leonardo Bouez
 * @date October 6, 2026
 *
 */

#include "unistd.h"
#include "stdio.h"
#include <stddef.h>

// Init process specific exit codes
// for debbuging
#define EXIT_IO_FAILURE -2
#define EXIT_FORK_FAILURE -3
#define EXIT_EXEC_FAILURE -4

// Fork a process for the shell
pid_t init_shell() {
  // Create a process for the shell
  pid_t sh_pid = fork();
  
  // Check if the process was opened sucessfully
  if (sh_pid <= -1) {
    printf("\n[init]: failed to fork the shell.");
    exit(EXIT_FORK_FAILURE);
  } else if (sh_pid == 0) {
    execv("/shell", NULL);
    printf("\n[init]: failed to execute the shell.");
    exit(EXIT_EXEC_FAILURE);
  }
  
  return sh_pid;
}

int main(int argc, char **argv) {
  // Open console for IO. This will be inherited
  // by all child processes from fork()
  int stdin = open("/console", O_RDONLY);
  int stdout = open("/console", O_WRONLY);
  int stderr = open("/console", O_WRONLY);
  
  // Check if all files opened successfully
  if (stdin <= -1 || stdout <= -1 || stderr <= -1) {
    exit(EXIT_IO_FAILURE);
  }
  
  printf("\n[init]: starting the shell...");
  
  // Initialize the shell
  pid_t sh_pid = init_shell();
  
  // Init loop to wait on it's zombie children  
  while (1) {
    int status;
    pid_t wait_pid = wait(&status);
    
    // Check if the shell exited
    if(wait_pid == -1){
      printf("\x1B[N");
      printf("\n[init]: the init process was not able to wait.");
      printf("\n[init]: type anything to restart the shell: ");
      getchar();
      
      sh_pid = init_shell();
    }else if (wait_pid == sh_pid) {
      // Guarantee the terminal is in canonical mode
      printf("\x1B[N");
      
      printf("\n[init]: shell exited with status %d.", status);
      
      // Pause the execution to allow reading the message above
      printf("\n[init]: type anything to restart the shell...");
      getchar();
      
      sh_pid = init_shell();
    }
  }
}
