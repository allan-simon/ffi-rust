#include <stdio.h>
#include "rust.h"

int main () {
    void* vector = vector_create();
    uint32_t size = vector_size(vector);
    for (uint32_t i = 0; i < size; i++) {
        printf(
            "value %d : %s\n",
            i,
            vector_get(vector, i)
        );
    }
    vector_print(vector);
    return 0;
}
