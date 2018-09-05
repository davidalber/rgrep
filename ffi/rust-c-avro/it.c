#include <stdint.h>
#include <stdio.h>

char *hello_rust();

void call_me_back(char *msg) {
    printf("%s\n", msg);
}

int main() {
    printf("%s\n", hello_rust());
    return 0;
}
