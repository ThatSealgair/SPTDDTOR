#include "aoc2021.h"

#define BUFFER 10

int distance = 0;
int depth = 0;
int aim = 0;

int navigation(char *cmd, char *val) {
	if(strcmp(cmd, "forward") == 0) {
		distance += (val[0] - 48);
	}
	else if (strcmp(cmd, "up") == 0) {
		depth -= (val[0] - 48); 
	}
	else if (strcmp(cmd, "down") == 0) {
		depth += (val[0] - 48);
	}
	return distance * depth;
}

int manual(char *cmd, char *val) {
	if(strcmp(cmd, "forward") == 0) {
		distance += (val[0] - 48);
		depth += (aim * (val[0] - 48));
	}
	else if (strcmp(cmd, "up") == 0) {
		aim -= (val[0] - 48);
	}
	else if (strcmp(cmd, "down") == 0) {
		aim += (val[0] - 48);
	}
	return distance * depth;
}

int main(void) {
	char *string;
	int size;
	struct stat st;

	stat("day2.txt", &st);
	string = (char *) malloc(size * sizeof(char));


	FILE *fp;

	fp = fopen("input.txt", "r");
	
	if (fp) {
		fscanf(fp, "%c", &string[0]);

		while(feof(fp)) {
			for (int i = 1; i < size; i++) {
				fscanf(fp, "%c", &string[i]);
			}
		}
	}

	fclose(fp);


	char *command = strtok(string, " ");
	char *value = strtok(NULL, "\n");

	int result_1 = 0;
	int result_2 = 0;

	while (command != NULL && value != NULL) {
		command = strtok(NULL, " ");
		value = strtok(NULL, "\n");

		if (command != NULL && value != NULL) {
			result_1 = compare(command, value);
			result_2 = manual(command, value);
		}
	}

	printf("[Part 1] %d\n", result_1);
	printf("[Part 2] %d\n", result_2);

	return EXIT_SUCCESS;
}
