#ifndef PLOP
#define PLOP

#include <stdint.h>
// we define the rust functions here
// so our C compiler know this functions exists
void * vector_create();

uint32_t vector_size(void* vector_ptr);

const char* vector_get(
    void* vector_ptr,
    uint32_t index
);

#endif
