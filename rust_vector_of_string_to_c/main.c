#include <stdio.h>
#include "rust.h"

int main () {
    void* vector = vector_create();
    uint32_t size = vector_size(vector);
    for (uint32_t i = 0; i < size; i++) {

        const char* value = vector_get(vector, i);
        printf(
            "value %d : %s\n",
            i,
            value
        );
        vector_value_free(value);
    }
    vector_print(vector);
    vector_free(vector);
    return 0;
}
