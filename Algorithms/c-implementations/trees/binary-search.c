#include "binary-search.h"

int binary_search_iter(int want, int arr[], int min, int max) {
	int mid = 0;

	while (min <= max) {
		mid = min + (max - min) / 2;

		if (want < arr[mid]) {
			max = mid + 1;
		}
		else if (want > arr[mid]) {
			min = mid + 1;
		}
		else {
			break;
		}
	}

	return (arr[mid] == want) ? (int) arr[mid] : NULL;
}
