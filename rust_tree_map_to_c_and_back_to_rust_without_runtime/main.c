#include "rust.h"

int main (int argc, char** argv) {

    void* data = create_tree();
    // print two times just to be sure
    // print_tree does not free the memory
    print_tree(data);
    print_tree(data);

}
