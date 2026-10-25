/**
 * @file crt0.c 
 * @brief The C runtime 0 provides the startup 
 * routines that are automatically linked to user
 * programs.
 *
 * @author Leonardo Bouez
 * @date October 6, 2026
 *
 */

/**
 * @brief Program entry point
 * 
 * The `_start` function gets linked to a program
 * and serves as the entry point of execution to
 * call `main` to and receive its
 * return status to terminate execution.
 */
void _start() {
  asm volatile (
    "call main\n"
    "call exit"
  );
}
