/**
 * @file unistd.h 
 * @brief This header declares the system call stubs and
 * constants.
 *
 * @author Leonardo Bouez
 * @date October 5, 2026
 *
 */

#ifndef __SYSCALL_H__
#define __SYSCALL_H__

// File flags
#define O_RDONLY 1<<0 // Read-only
#define O_WRONLY 1<<1 // Write-only
#define O_RDWR   1<<2 // Read-write
#define O_CREAT  1<<3 // Open syscall creates the file

// Exit codes
#define EXIT_SUCCESS 0
#define EXIT_FAILURE -1

// Kill signals
#define SIGKILL 9

// System call codes
#define SYS_EXIT    1
#define SYS_FORK    2
#define SYS_READ    3
#define SYS_WRITE   4
#define SYS_OPEN    5
#define SYS_CLOSE   6
#define SYS_WAITPID 7
#define SYS_EXECV   8
#define SYS_PIPE    9
#define SYS_DUP     10
#define SYS_KILL    11
#define SYS_PAUSE   12
#define SYS_GETPID  13
#define SYS_SLEEP   14
#define SYS_UPTIME  15
#define SYS_SBRK    16

// Process ID
typedef int pid_t;

void exit(int status);
pid_t fork();
int read(int fd, const void *buf, unsigned int count);
int write(int fd, const void *buf, unsigned int count);
int open(const char *path, int flags);
int close(int fd);
pid_t waitpid(pid_t pid, int *status);
pid_t wait(int *status);
int execv(const char *path, const char **argv);
int dup(int fd);
int kill(pid_t pid, int sig);
int pause();
pid_t getpid();
unsigned long int sleep(unsigned long int ticks);
unsigned long int uptime();
void* sbrk(int increment);
int brk(void *addr);

#endif
