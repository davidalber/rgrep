#include <stdint.h>
#include <stdio.h>

char *hello_rust();

int main() {
    printf("%s\n", hello_rust());
    return 0;
}
