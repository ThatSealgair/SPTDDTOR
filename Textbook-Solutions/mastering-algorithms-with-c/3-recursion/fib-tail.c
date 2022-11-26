#include <stdio.h>
#include <stdlib.h>

int fact_tail(int n, int a) {
	if (n < 0) {
		return 0;
	}
	else if (n == 0) {
		return 1;
	}
	else if (n == 1) {
		return a;
	}
	else {
		return fact_tail(n - 1, n * a);
	}
}
