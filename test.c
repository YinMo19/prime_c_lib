#include "prime.h"
#include <stdio.h>

int main() {
    FILE    *f_ptr = fopen("primes.txt", "w");
    uint64_t n     = 0;
    for (uint64_t i = 0; i < 10000000; i++) {
        if (rust_is_prime(i)) {
            fprintf(f_ptr, "%llu\n", i);
        }
    }
    fclose(f_ptr);
    return 0;
}
