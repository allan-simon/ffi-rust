#ifndef PLOP
#define PLOP

// we define the rust functions here
// so our C compiler know this functions exists
void* hash_new();

void hash_print(void* data);

void hash_free(void* data);

#endif
