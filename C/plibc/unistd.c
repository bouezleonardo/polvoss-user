/**
 * @file syscall.c 
 * @brief This file defines the system call stubs.
 *
 * @author Leonardo Bouez
 * @date October 5, 2026
 *
 */

#include "unistd.h"

void exit(int status) {
  asm volatile (
    "li a7, 1\n"
    "ecall"
  );
}

pid_t fork() {
  pid_t pid;
  asm volatile (
    "li a7, 2\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(pid)
  );
  return pid;
}

int read(int fd, const void *buf, unsigned int count) {
  // Bytes read
  int bytes;
  asm volatile (
    "li a7, 3\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(bytes) // output
  );
  return bytes;
}

int write(int fd, const void *buf, unsigned int count) {
  // Bytes written
  int bytes;
  asm volatile (
    "li a7, 4\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(bytes) // output
  );
  return bytes;
}

int open(const char *path, int flags) {
  // File descriptor
  int fd;
  asm volatile (
    "li a7, 5\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(fd) // output
  );
  return fd;
}

int close(int fd) {
  int ret;
  asm volatile (
    "li a7, 6\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(ret) // output
  );
  return ret;
}

pid_t waitpid(pid_t pid, int *status) {
  asm volatile (
    "li a7, 7\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(pid)
  );
  return pid;
}

pid_t wait(int *status) {
  return waitpid(-1, status);
}

int execv(const char *path, const char **argv) {
  int ret;
  asm volatile (
    "li a7, 8\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(ret)
  );
  return ret;
}

int dup(int fd) {
  int newfd;
  asm volatile (
    "li a7, 10\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(newfd)
  );
  return newfd;
}

int kill(pid_t pid, int sig){
  int ret;
  asm volatile (
    "li a7, 11\n"
    "ecall\n"
    "mv %0, a0"
    :"=r" (ret)
  );
  return ret;
}

int pause(){
  int ret;
  asm volatile (
    "li a7, 12\n"
    "ecall\n"
    "mv %0, a0"
    :"=r" (ret)
  );
  return ret;
}

pid_t getpid() {
  pid_t pid;
  asm volatile (
    "li a7, 13\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(pid)
  );
  return pid;
}

unsigned long int sleep(unsigned long int ticks) {
  asm volatile (
    "li a7, 14\n"
    "ecall\n"
    "mv %0, a0"
    : "=r"(ticks) // output
  );
  return ticks;
}

unsigned long int uptime() {
  unsigned long int uptime;
  asm volatile (
    "li a7, 15\n"
    "ecall\n"
    "mv %0, a0"
    :"=r" (uptime)
  );
  return uptime;
}

void* sbrk(int increment){
  void* prev_break; // Previous program break
  asm volatile (
    "li a7, 16\n"
    "ecall\n"
    "mv %0, a0"
    :"=r" (prev_break)
  );
  return prev_break;
}
