#include "rust.h"

int main (int argc, char** argv) {
    void* data = hash_new();
    hash_print(data);
    hash_print(data);
    hash_free(data);
}

