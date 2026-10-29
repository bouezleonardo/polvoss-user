#include <stdbool.h>
#include <stddef.h>
#include "unistd.h"
#include "stdlib.h"

#include "stdio.h"

/*************************|MEMORY MANAGEMENT|***************************/

/** @brief End address of process data segments. 
 * This symbol is provided at the end of user.ld and its address is
 * the indicates where the data loaded by the kernel ends. This is used
 * as a base for the heap.
*/
extern char euser;
static void* heap_base = (void*)&euser;

/** @brief Whether or not the program break is positioned. */
static bool brk_positioned = false;

typedef struct BlockHeader BlockHeader;

/** @brief Memory block header. */
struct BlockHeader {
  bool in_use;        // Whether the block is in use
  unsigned int size;  // Block size in bytes (not counting header)
  BlockHeader* next;  // Next block
};

/** @brief Resize a block.
 *
 * @param b Block header to resize.
 * @param len Size of the block after resize .
*/
static void block_resize(BlockHeader* b, unsigned int len) {
  // Update the blocks
  b->size += len;
  b->next->size -= len;
  
  // Bump the b->next header
  *(BlockHeader*)((char*)b->next+len) = *b->next;
}

/** @brief Resize a block.
 *
 * @param b Block header to divide.
 * @param len Size of the block after divide.
 *
 * @return New block header that was created.
*/
static BlockHeader* block_divide(BlockHeader* b, unsigned int len) {
  // New block
  BlockHeader* newb = (BlockHeader*)((char*)b+len+sizeof(BlockHeader)); 
  
  // Update the block links
  newb->next = b->next;
  b->next = newb;
  
  // Update the block sizes
  newb->size = b->size-len-sizeof(BlockHeader);
  b->size = len;
  
  newb->in_use = false;
  
  return newb;
}

/**
 * @brief Allocate a new block of memory. 
 * 
 * Search a linked list of blocks in memory and allocate the first
 * fit.
 *
 * @param size Size of the block.
 *
 * @return Block header of the allocated memory.
 */
static BlockHeader* alloc_block(unsigned int size) {
  BlockHeader* block = NULL;
  
  // Check if the program break is positioned
  if(!brk_positioned) {
    // Try to reposition the program break
    if(brk(heap_base) == -1)
      return NULL;   
    
    // Init the first Block header
    *(BlockHeader*)heap_base = (BlockHeader) {
      .in_use = false, 
      .size = 0, 
      .next = NULL
    };
    
    brk_positioned = true;
  } else {
    // Start reading from the heapbase
    block = (BlockHeader*)heap_base;
  }
  
  // Look for free blocks
  while(block != NULL) {
    // Check if the block is in use
    if(block->in_use == false) {
      // Check if there is enough space in this block
      if(block->size == size) {
        block->in_use == true;
        return block;
      } else if(block->size < size) {
        BlockHeader* next = block->next;
        
        // Check if the next block is free and has memory to give
        if(next != NULL && !next->in_use && (block->size + next->size) >= size) {
          block_resize(block, size - block->size);
          block->in_use == true;
          return block;
        }
      } else if(block->size > size) {
        // Divide the block in two
        (void) block_divide(block, size);
        block->in_use = true;
        return block;
      }
    }
    block = block->next;
  }
  
  // If no free block was found
  // A new block will be created on the top of the heap
  block = (BlockHeader*)sbrk(0);
  
  // Ask the OS for more memory
  if(sbrk(size+sizeof(BlockHeader)) == (void*)-1)
    return NULL;
  block->in_use = true;
  block->size = size;
  block->next = NULL;

  return block;
}

/**
 * @brief Deallocate a block of memory. 
 * 
 * Deallocate a block of memory and give the memory back to the OS
 * if necessary.
 *
 * @param block Block header to be deallocated.
 */
static void dealloc_block(BlockHeader *block) {

}

/*************************|STDLIB|***************************/

int atoi(const char *s) {
  int i = 0;  
  
  // Ignore white space
  while (s[i]=='\f'||s[i]=='\n'||s[i] =='\r'||s[i]=='\t'||s[i]=='\v')
    ++i;
  
  // Is negative
  int is_neg = 0;
  
  // Ignore the sign
  if (s[i] == '-'|| s[i] == '+') {
    i += 1;
    is_neg = 1;
  }
  
  int num = 0;
  while (s[i] >= '0' && s[i] <= '9') {
    // Advance the decimal place
    num *= 10;
    // Get the number from the character
    num += s[i]-'0';
    ++i;
  }
  
  if (is_neg)
    num *= -1;
  
  return num;
}

void* malloc(unsigned int size) {
  // Allocate memory
  BlockHeader* p = alloc_block(size);
  
  if(p == NULL) {
    return NULL;
  }
  // Return the first usable position
  return (void*)(p+1);
}

void free(void* p) {
  if(p != NULL) {
    // Pass the block header
    dealloc_block((BlockHeader*)p-1);
  } 
}
