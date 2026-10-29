/**
 * @file stdlib.h 
 * @brief This is a general purpose library. 
 * (memory allocation, string manipulation, sorting etc.).
 *
 * @author Leonardo Bouez
 * @date October 24, 2026
 *
 */
 
#ifndef __STDLIB_H__
#define __STDLIB_H__

/**
 * @brief Convert a string to an integer. 
 *
 * This does not detect any errors.
 *
 * @param s String to be converted.
 *
 * @return The converted value.
 */
int atoi(const char *s);

/**
 * @brief Allocate a specified amount of memory. 
 *
 * @param size String to be converted.
 *
 * @return The pointer to the memory if successful, `NULL` otherwise.
 */
void* malloc(unsigned int size);

/**
 * @brief Deallocate memory specified by a pointer. 
 *
 * The pointer must come from `malloc` or other related function.
 *
 * @param p Pointer to the memory to be deallocated.
 */
void free(void* p);

#endif
