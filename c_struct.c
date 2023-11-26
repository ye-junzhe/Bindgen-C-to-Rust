#include <stdio.h>

typedef struct {
    int x;
    int y;
} CoolStruct;

void cool_function(CoolStruct cs) {
    printf("Inside the CoolStruct => x: %d, y: %d\n", cs.x, cs.y);
}

#define ARRAY_LENGTH(arr) (sizeof(arr) / sizeof(arr[0]))
int A[10];
int A_len = ARRAY_LENGTH(A);
