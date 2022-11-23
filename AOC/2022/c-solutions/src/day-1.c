#include "aoc2021.h"

#define SIZE 2000


int positive_change(int *depth) {
	int counter = 0;
	for(int i = 0; i < SIZE - 1; i++) {
		if(depth[i] < depth[i+1]) {
			counter++;
		}
	}
	return counter;
}


int sliding_window(int *depth) {
	int counter = 0;
	for(int i = 0; i < SIZE - 2; i++) {
		if(depth[i] < depth[i+3]) {
			counter++;
		}
	}
	return counter;
}

int main(int argc, char **argv) {
	FILE *fp;
	int i = 0;	
	int *depth;

	fp = fopen("day1.txt", "r");
	depth = (int*) malloc(SIZE * sizeof(int));

	if(fp) {
		fscanf(fp, "%d", &depth[i]);

		while(!feof(fp)) {
			for (i = 1; i < SIZE; i++) {
				fscanf(fp, "%d", &depth[i]);
			}
		}
		fclose(fp);
	}

	int solution_1 = positive_change(depth);
	int solution_2 = sliding_window(depth);

	printf("[Total Increasing Measurements] %d\n", solution_1);
	printf("[Sliding Window Measurements] %d\n", solution_2);

	return 0;
}
