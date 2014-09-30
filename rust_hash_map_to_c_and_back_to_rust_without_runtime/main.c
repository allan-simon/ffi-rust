#include "rust.h"

int main (int argc, char** argv) {
    void* data = create_hash();
    print_hash(data);
    print_hash(data);
    hash_free(data);
}

