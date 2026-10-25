#ifndef __STDIO_H__
#define __STDIO_H__

// Standard Files for I/O
#define STDIN 0
#define STDOUT 1
#define STDERR 3

// End of file
#define EOF -1

/**
 * @brief Formatted print to the standard output.
 * 
 * Print a formatted string to the standard output.
 *
 * @param format Pointer to the format to be printed
 * @param ... Additional arguments specified by the format
 *
 * @return Number of bytes printed (null character excluded)
 */
int printf(const char *format, ...);

/**
 * @brief Formatted print to a file descriptor. 
 * 
 * Print a formatted string to a file.
 *
 * @param fd File descriptor
 * @param format Pointer to the format to be printed
 * @param ... Additional arguments specified by the format
 *
 * @return Number of bytes printed (null character excluded)
 */
int fprintf(int fd, const char *format, ...);

/**
 * @brief Get a character from a file. 
 *
 * @param fd File descriptor
 *
 * @return Unsigned char casted as int, or EOF on error or
 * end of input.
 */
int getc(int fd);

/**
 * @brief Write a character from a file. 
 *
 * @param fd File descriptor
 * @param c Character to write
 *
 * @return The written unsigned char casted as int, or EOF 
 * on error
 */
int putc(int fd, int c);

/**
 * @brief Get a character from the standard input. 
 * 
 * Read a character from stdin.
 *
 * @return Unsigned char casted as int, or EOF on error or
 * end of input.
 */
int getchar();

/**
 * @brief Write a character from the standard output. 
 *
 * @param c Character to write
 *
 * @return The written unsigned char casted as int, or EOF 
 * on error
 */
int putchar(int c);

/**
 * @brief Read a line from a file. 
 *
 * This reads up to `len`-1 bytes, so it's safe to use if `len` 
 * is set to the size of the buffer or less.
 *
 * @param fd File descriptor
 * @param buf Buffer to store the characters
 * @param len Buffer length
 *
 * @return Amount of bytes read, -1 on error
 */
int getline(int fd, char *buf, unsigned int len);

/**
 * @brief Read a line from the standard input. 
 *
 * This reads up to `len`-1 bytes, so it's safe to use if `len` 
 * is set to the size of the buffer or less.
 *
 * @param buf Buffer to store the characters
 * @param len Buffer length
 *
 * @return Amount of bytes read, -1 on error
 */
int gets(char *buf, unsigned int len);

#endif
