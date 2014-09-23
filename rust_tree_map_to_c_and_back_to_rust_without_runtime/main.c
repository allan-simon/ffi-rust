#include "rust.h"

int main (int argc, char** argv) {

    void* data = create_hash();
    // print two times just to be sure
    // print_hash does not free the memory
    print_hash(data);
    print_hash(data);

}
