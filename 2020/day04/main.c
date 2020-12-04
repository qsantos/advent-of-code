#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* readfile(const char* filename) {
    FILE* f = fopen(filename, "r");
    assert(f);

    fseek(f, 0, SEEK_END);
    long pos = ftell(f);
    fseek(f, 0, SEEK_SET);
    assert(pos > 0);
    size_t filesize = (size_t) pos;

    char* ret = malloc(filesize + 1);
    assert(ret);
    assert(fread(ret, filesize, 1, f) == 1);
    ret[filesize] = '\0';

    fclose(f);

    return ret;
}

int count_valid_passwords(int(*passport_validator)(char*)) {
    char* data = readfile("input");

    int n_valid_passports = 0;
    char* current_password = data;
    while (current_password) {
        char* end = strstr(current_password, "\n\n");
        if (end != NULL) {
            *end = '\0';
        }
        n_valid_passports += passport_validator(current_password);
        current_password = end ? end + 2 : NULL;
    }
    return n_valid_passports;
}

int valid_passport1(char* passport) {
    int found = 0;
    char* current_field = passport;
    while (*current_field) {
        size_t field_length = strcspn(current_field, " \n");
        if (field_length == 0) {
            break;
        }
        current_field[field_length] = '\0';
        char* sep = strchr(current_field, ':');
        assert(sep);
        *sep = '\0';

        const char* name = current_field;
        if (strcmp(name, "byr") == 0) { found |= 1 << 0; }
        if (strcmp(name, "iyr") == 0) { found |= 1 << 1; }
        if (strcmp(name, "eyr") == 0) { found |= 1 << 2; }
        if (strcmp(name, "hgt") == 0) { found |= 1 << 3; }
        if (strcmp(name, "hcl") == 0) { found |= 1 << 4; }
        if (strcmp(name, "ecl") == 0) { found |= 1 << 5; }
        if (strcmp(name, "pid") == 0) { found |= 1 << 6; }

        current_field += field_length + 1;
    }
    return found == 0x7f;
}

void puzzle1(void) {
    printf("%d\n", count_valid_passwords(valid_passport1));
}

int valid_passport2(char* passport) {
    int found = 0;
    char* current_field = passport;
    while (*current_field) {
        size_t field_length = strcspn(current_field, " \n");
        if (field_length == 0) {
            break;
        }
        current_field[field_length] = '\0';
        char* sep = strchr(current_field, ':');
        assert(sep);
        *sep = '\0';

        const char* name = current_field;
        const char* value = sep + 1;
        if (strcmp(name, "byr") == 0) {
            found |= 1 << 0;
            int byr = atoi(value);
            if (!(1920 <= byr && byr <= 2002)) {
                return 0;
            }
        }
        if (strcmp(name, "iyr") == 0) {
            found |= 1 << 1;
            int iyr = atoi(value);
            if (!(2010 <= iyr && iyr <= 2020)) {
                return 0;
            }
        }
        if (strcmp(name, "eyr") == 0) {
            found |= 1 << 2;
            int eyr = atoi(value);
            if (!(2020 <= eyr && eyr <= 2030)) {
                return 0;
            }
        }
        if (strcmp(name, "hgt") == 0) {
            found |= 1 << 3;
            size_t n = strlen(value);
            if (n < 2) { return 0; }
            if (strcmp(value + n - 2, "cm") == 0) {
                int l = atoi(value);
                if (!(150 <= l && l <= 193)) {
                    return 0;
                }
            } else if (strcmp(value + n - 2, "in") == 0) {
                int l = atoi(value);
                if (!(59 <= l && l <= 76)) {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        if (strcmp(name, "hcl") == 0) {
            found |= 1 << 4;
            if (strlen(value) != 7 || value[0] != '#' || strspn(value + 1, "0123456789abcdef") != 6) {
                return 0;
            }
        }
        if (strcmp(name, "ecl") == 0) {
            found |= 1 << 5;
            if (strcmp(value, "amb") != 0
                && strcmp(value, "blu") != 0
                && strcmp(value, "brn") != 0
                && strcmp(value, "gry") != 0
                && strcmp(value, "grn") != 0
                && strcmp(value, "hzl") != 0
                && strcmp(value, "oth") != 0
            ) {
                return 0;
            }
        }
        if (strcmp(name, "pid") == 0) {
            found |= 1 << 6;
            if (strlen(value) != 9 || strspn(value, "0123456789") != 9) {
                return 0;
            }
        }

        current_field += field_length + 1;
    }
    return found == 0x7f;
}

void puzzle2(void) {
    printf("%d\n", count_valid_passwords(valid_passport2));
}

int main(void) {
    puzzle1();
    puzzle2();
}
