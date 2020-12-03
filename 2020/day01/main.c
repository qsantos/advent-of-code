#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void puzzle1(void) {
    FILE* f = fopen("input", "r");
    if (!f) {
        fprintf(stderr, "Could not open file\n");
        return;
    }

    const int target = 2020;
    char seen[2020] = {0};
    while (!feof(f)) {
        char buffer[256];
        char* end = fgets(buffer, sizeof(buffer), f);
        if (!end) {
            fprintf(stderr, "Could not read line\n");
            return;
        }
        if (end[strlen(end) - 1] != '\n') {
            fprintf(stderr, "Line too long\n");
            return;
        }
        int a = atoi(buffer);
        int b = target - a;
        if (0 <= b && b < target && seen[b]) {
            printf("%d\n", a * b);
            break;
        }
        seen[a] = 1;
    }

    fclose(f);
}

void puzzle2(void) {
    FILE* f = fopen("input", "r");
    if (!f) {
        fprintf(stderr, "Could not open file\n");
        return;
    }

    const int target = 2020;
    char seen[2020] = {0};
    while (!feof(f)) {
        char buffer[256];
        char* end = fgets(buffer, sizeof(buffer), f);
        if (!end) {
            fprintf(stderr, "Could not read line\n");
            return;
        }
        if (end[strlen(end) - 1] != '\n') {
            fprintf(stderr, "Line too long\n");
            return;
        }
        int a = atoi(buffer);
        for (int b = 0; b < target; b++) {
            int c = target - a - b;
            if (seen[b] && 0 <= c && c < target && seen[c]) {
                printf("%d\n", a * b * c);
                return;
            }
        }
        seen[a] = 1;
    }

    fclose(f);
}

int main(void) {
    puzzle1();
    puzzle2();
}
