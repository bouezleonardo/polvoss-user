#include "unistd.h"
#include "stdio.h"
#include "string.h"
#include <stdarg.h>
#include <stdbool.h>

/****************|AUXILIARY|*****************/
static const char digits[] = "0123456789abcdef";

/** @brief Print an integer in decimal base. */
static int print_int(int fd, int n) {  
  int buf[20];      // Buffer for the string
  bool neg = false; // Number is negative
  
  if (n < 0) {
    neg = true;
    n *= -1;
  }
  
  // Get each digit char
  int i = 0;
  do {
    buf[i] = n%10 + '0';
    n /= 10;
    ++i;
  } while (n > 0);
  
  int bytes = 0;
  
  // Print sign
  if (neg) {
    // Try printing one character
    if (putc(fd, '-') == EOF)
      return EOF;
    ++bytes;  
  }
  
  // Printing buf in reverse
  while (i > 0) {
    // Try printing one character
    if (putc(fd, buf[--i]) == EOF)
      return EOF;
    ++bytes;  
  }
  
  return bytes;
}

/** @brief Print an unsigned int in decimal or hexadecimal base. */
static int print_unsigned_int(int fd, unsigned int n, int base) {  
  int buf[20];      // Buffer for the string
  
  if(base != 16 && base != 10) {
    base = 10;
  }
  
  // Get each digit char
  int i = 0;
  do {
    buf[i] = digits[n%base];
    n /= base;
    ++i;
  } while (n > 0);
  
  int bytes = 0;
  
  // Printing buf in reverse
  while (i > 0) {
    // Try printing one character
    if (putc(fd, buf[--i]) == EOF) 
      return EOF;
    ++bytes;  
  }
  
  return bytes;
}

/** @brief Print a double in decimal base. */
static int print_double(int fd, double n) {
  // Print the integer part
  print_int(fd, (int)n);
  putc(fd, '.');
  // Print the 6 first decimal points
  print_int(fd, (int)((n - (int)n)*1000000));
  return 0;
} 

/** @brief Format and write a string to a file. */
static int _print(int fd, const char *format, va_list ap) {
  // String length
  int len = strlen(format);
  // Last index where the write begun
  int last = 0;
  // Number of characters written
  int bytes = 0;
  
  for(int i = 0;i < len;++i) {
    char c = format[i];
    
    if(c == '%') {
      // Write everything up to this point except %
      bytes += write(fd, format+last, i-last);
      
      // Check if it is another %
      if(format[i+1] != '%') {
        switch(format[i+1]) {
          case 'd':
          case 'i':
            bytes += print_int(fd, va_arg(ap, int));
            break;
          case 'u':
            bytes += print_unsigned_int(fd, va_arg(ap, unsigned int), 10);
            break;
          case 'x':
            bytes += print_unsigned_int(fd, va_arg(ap, unsigned int), 16);
            break;
          case 'f':
            bytes += print_double(fd, va_arg(ap, double));
            break;
          case 'c':
            bytes += putc(fd, va_arg(ap, int)); 
            break;
          case 's':
            char *s = (char *)va_arg(ap, int);
            bytes += write(fd, s, strlen(s));
            break;
        }
      } else {
        bytes += putc(fd, '%');
      }
      // Bypass the specifier
      i += 2;
      last = i; 
    }
  }
  
  // Check if there is something to write
  if(len-last != 0) {
    bytes += write(fd, format+last, len-last);
  }
  
  return bytes;
}

/******************|STD|*********************/ 
int printf(const char *format, ...) {
  // Initialize the variadic list
  va_list ap;
  va_start(ap, format);
  
  int bytes = _print(STDOUT, format, ap);
  
  va_end(ap);
  
  return bytes;
}

int fprintf(int fd, const char *format, ...) {
  // Initialize the variadic list
  va_list ap;
  va_start(ap, format);
  
  int bytes = _print(fd, format, ap);
  
  va_end(ap);
  
  return bytes;
}

int getc(int fd) {
  char buf[2];
  if (read(fd, buf, 1) == EOF)
    return EOF;
  return buf[0];
}

int putc(int fd, int c) {
  if (write(fd, (char *)&c, 1) == -1)
    return EOF;
  return c;
}

int getline(int fd, char *buf, unsigned int len) {
  return read(fd, buf, len);
}

int getchar() {
  return getc(STDIN);
}

int putchar(int c) {
  return putc(STDOUT, c);
}

int gets(char *buf, unsigned int len) {
  return read(STDIN, buf, len);
}
