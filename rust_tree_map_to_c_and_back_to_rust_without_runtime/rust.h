#ifndef PLOP
#define PLOP

// we define the rust functions here
// so our C compiler know this functions exists
void* create_hash();

void print_hash(void* data);
int run(int argc, char** argv, void (*kont)(void));

#endif
