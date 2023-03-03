#include <stdio.h>

void print_list(const int* arr, const int len) {
    for(int i = 0; i < len; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}


// a and b are not the indices, they are the addresses of the values
void swap(int* a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

void isort(int* arr, const int len) {
    for(int i = 1; i < len; i++) {
        int j = i;
        while(j > 0 && arr[j-1] > arr[j]) {
            swap(&arr[j], &arr[j-1]);
            j -= 1;
        }
    }
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
isort(arr, len);
print_list(arr, len);
}


