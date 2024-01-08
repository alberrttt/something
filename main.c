#include <stdio.h>

int main() {
    // Get the value of the base pointer (rbp)
    void* rbp_value;
    asm("mov %%rbp, %0" : "=r" (rbp_value));

    // Print the value of rbp
    printf("Value of rbp: %p\n", rbp_value);

    return 0;
}
