#include <assert.h>
#include <stdio.h>
#include <string.h>

void puzzle1(void) {
    FILE* f = fopen("input", "r");
    assert(f);
    size_t n_valid_passwords = 0;
    while (!feof(f)) {
        size_t lowest, highest;
        char letter;
        char password[256];
        fscanf(f, "%zu-%zu %c: %s\n", &lowest, &highest, &letter, password);
        size_t count = 0;
        for (char* p = password; *p; p++) {
            count += *p == letter;
        }
        n_valid_passwords += lowest <= count && count <= highest;
    }
    fclose(f);
    printf("%zu\n", n_valid_passwords);
}

void puzzle2(void) {
    FILE* f = fopen("input", "r");
    assert(f);
    size_t n_valid_passwords = 0;
    while (!feof(f)) {
        size_t first, second;
        char letter;
        char password[256];
        fscanf(f, "%zu-%zu %c: %s\n", &first, &second, &letter, password);
        int first_ok = first <= strlen(password) && password[first - 1] == letter;
        int second_ok = second <= strlen(password) && password[second - 1] == letter;
        n_valid_passwords += first_ok != second_ok;
    }
    fclose(f);
    printf("%zu\n", n_valid_passwords);
}

int main(void) {
    puzzle1();
    puzzle2();
}
