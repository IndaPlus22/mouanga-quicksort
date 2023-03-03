#include <stdio.h>

void print_list(int *list, int len) {
    for(int i = 0; i < len; i++) {
        printf("%d ", list[i]);
    }
    printf("\n");
}

int main() {

    // i/o
    int len;
    scanf("%d", &len);

    int arr[len];
    for (int i = 0; i < len; i++) {
        scanf("%d", &arr[i]);
    }

print_list(arr, len);
}


