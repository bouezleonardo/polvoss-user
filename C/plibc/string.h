/**
 * @brief Get the size of a string (excluding \0). 
 * 
 * This should receive only null byte terminated strings.
 *
 * @param s String pointer
 *
 * @return number of bytes in the string pointed to by s.
 */
int strlen(const char* s);

/**
 * @brief Compare two strings. 
 * 
 * Compares two strings character by character based on their ASCII values. 
 * Stop when until it finds a mismatch or reaches the null terminator ('\0').
 *
 * @param s1 First string pointer
 * @param s2 Second string pointer
 *
 * @return number of bytes in the string pointed to by s.
 */
int strcmp(const char* s1, const char* s2);
