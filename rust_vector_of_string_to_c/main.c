#include <stdio.h>
#include "rust.h"

int main () {
    printf("start\n");
    void* vector = vector_create();
    printf("get_size\n");
    uint32_t size = vector_size(vector);
    printf("size %d\n", size);
    for (uint32_t i = 0; i < size; i++) {
        printf(
            "dedede%s dedede\n",
            vector_get(vector, i)
        );
    }
    return 0;
}
