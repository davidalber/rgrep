#include <stdint.h>
#include <stdio.h>

char *hello_rust();

void call_me_back(char *msg) {
    printf("[C] Passed by Rust to `call_me_back`: %s\n", msg);
}

int main() {
    printf("[C] Calling `hello_rust`\n");
    printf("[C] Returned by Rust: %s\n", hello_rust());
    return 0;
}
