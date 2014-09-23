#include "rust.h"

void do_stuff();

int main (int argc, char** argv) {
    return run(argc, argv, do_stuff);
}

void do_stuff () {
    void* data = create_hash();
    print_hash(data);
    print_hash(data);
}
