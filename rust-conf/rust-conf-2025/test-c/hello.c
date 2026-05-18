#include <stdio.h>
#include <stdlib.h>

// Procedure language and does not support OOP features
// no function keyword
int main() {
    int lucky = 23;
    printf("Value: %i \n", lucky);
    printf("Pointer: %p \n", &lucky);

    // array of characters to represent string
    // each character has its own memory address and terminated by null character
    char hello[] = "Hello";

    char *str = malloc(4);
    str[0] = 'h';
    str[1] = 'e';
    str[2] = 'y';
    str[3] = '\0';
    free(str);

    // 0 for exit success
    return 0;
}