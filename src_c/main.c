#include <stdio.h>
#include <stdbool.h>

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



int partition(int* arr, int low, int high) {
    int pivot = arr[(low + high) / 2];
    int i = low - 1;
    int j = high + 1;

    for(;;) {
        do {i++;} while(arr[i] < pivot);
        do {j--;} while(arr[j] > pivot);

        if (i >= j) {
            return j;
        }

        swap(&arr[i], &arr[j]);
    }

}

void _quicksort(int* arr, const int low, const int high) {
    if (high - low < 30 && high > 0) {
        int _len = high - low + 1;
        isort(&arr[low], _len);
        return;
    }
    if(low >= 0 && high >= 0 && low < high) {
        int partition_index = partition(arr, low, high);
        _quicksort(arr, low, partition_index);
        _quicksort(arr, partition_index + 1, high);  
    }
}

void quicksort(int* arr, const int low, const int high) {
    if (high - low < 30 && high > 0) {
        int _len = high - low + 1;
        isort(&arr[low], _len);
        return;
    }
    if(low >= 0 && high >= 0 && low < high) {
        int partition_index = partition(arr, low, high);
        _quicksort(arr, low, partition_index);
        _quicksort(arr, partition_index + 1, high);  
    }
}


int main() {

    // in
    int len;
    scanf("%d", &len);

    int arr[len];
    for (int i = 0; i < len; i++) {
        scanf("%d", &arr[i]);
    }

    // DoStuff();
    quicksort(arr, 0, len - 1);

    // out
    print_list(arr, len);
}


