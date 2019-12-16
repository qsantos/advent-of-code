#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Signal {
    size_t n;
    int v[];
};

struct Signal* signal_new(size_t n) {
    struct Signal* ret = malloc(sizeof(struct Signal) + n * sizeof(int));
    assert(ret != NULL);
    ret->n = n;
    return ret;
}

void signal_copy(struct Signal* dst, const struct Signal* src) {
    assert(dst->n == src->n);
    for (size_t i = 0; i < src->n; i += 1) {
        dst->v[i] = src->v[i];
    }
}

struct Signal* signal_parse(const char* s, size_t c) {
    size_t n = strlen(s);
    struct Signal* ret = signal_new(n * c);
    assert(ret != NULL);
    size_t k = 0;
    for (size_t i = 0; i < c; i += 1) {
        for (size_t j = 0; j < n; j += 1) {
            ret->v[k] = s[j] - '0';
            k += 1;
        }
    }
    return ret;
}

size_t min(size_t a, size_t b) {
    return a < b ? a : b;
}

void signal_step(struct Signal* dst, const struct Signal* src, int* sums) {
    assert(dst->n == src->n);
    size_t n = src->n;

    sums[0] = 0;
    for (size_t i = 0; i < n; i += 1) {
        sums[i + 1] = sums[i] + src->v[i];
    }

    for (size_t i = 0; i < n / 3; i += 1) {
        int digit = 0;

        size_t j = 0;
        while (1) {
            // i + 1 - 1 zeroes
            j += i;

            // i + 1 ones
            if (j >= n) {
                break;
            }
            digit -= sums[j];
            j = min(n, j + i + 1);
            digit += sums[j];

            // i + 1 zeroes
            j += i + 1;

            // i + 1 minus ones
            if (j >= n) {
                break;
            }
            digit += sums[j];
            j = min(n, j + i + 1);
            digit -= sums[j];

            // 1 zero
            j += 1;
        }

        dst->v[i] = abs(digit) % 10;
    }
    for (size_t i = n / 3; i < n / 2; i += 1) {
        dst->v[i] = abs(sums[2 * i + 1] - sums[i]) % 10;
    }
    for (size_t i = n / 2; i < n; i += 1) {
        dst->v[i] = abs(sums[n] - sums[i]) % 10;
    }
}

void signal_steps(struct Signal* dst, const struct Signal* src, size_t steps) {
    assert(dst->n == src->n);

    if (steps == 0) {
        signal_copy(dst, src);
        return;
    }

    int* sums = malloc(sizeof(int) * (src->n + 1));
    assert(sums != NULL);

    signal_step(dst, src, sums);
    for (size_t step = 1; step < steps; step += 1) {
        signal_step(dst, dst, sums);
    }

    free(sums);
}

int signal_read_int(struct Signal* signal, int offset, int digits) {
    assert(digits <= (int) signal->n);
    assert(offset <= (int) signal->n - digits);
    int ret = 0;
    for (int digit = 0; digit < digits; digit += 1) {
        ret *= 10;
        ret += signal->v[offset + digit];
    }
    return ret;
}

int signal_extract_message(const char* s) {
    struct Signal* signal = signal_parse(s, 10000);
    int offset = signal_read_int(signal, 0, 7);

    signal_steps(signal, signal, 100);
    int ret = signal_read_int(signal, offset, 8);

    free(signal);
    return ret;
}

void signal_test_steps(const char* start, size_t steps, int ref) {
    struct Signal* signal = signal_parse(start, 1);
    signal_steps(signal, signal, steps);

    int res = signal_read_int(signal, 0, 8);
    if (res != ref) {
        printf("%s + %zu = %i != %i\n", start, steps, res, ref);
    }

    free(signal);
}

void signal_test_read_int(void) {
    struct Signal* signal = signal_parse("98765432109876543210", 1);
    assert(signal_read_int(signal, 7, 8) == 21098765);
    free(signal);
}

static const char* input = (
    "5978199846243867500618549676248592543697050347275117445908032699"
    "4618036736403094024111488348676644802419244196591075975610084280"
    "3080594156950599183689118908528517600320005432057240917646333907"
    "6521256130708233828786671548954506956633030387391434374519829739"
    "1838950197434577938472242535458546669655890258618400619467693925"
    "1856018804535819474757415367869569202866812719370423942720344101"
    "6108036504444068283024877454701822334755130859069898921988043039"
    "4446893636437913072055636558787182933357009123440661477321673973"
    "8778759740286546886393135023823658542453116411987625204780100159"
    "6878920227074688039926825117649059942746938538436467515346144800"
    "7234636949"
);

int main(void) {
    // puzzle 1
    signal_test_steps("12345678", 4, 1029498);
    signal_test_steps("80871224585914546619083218645595", 100, 24176176);
    signal_test_steps("19617804207202209144916044189917", 100, 73745418);
    signal_test_steps("69317163492948606335995924319873", 100, 52432133);
    signal_test_steps(input, 100, 23135243);

    // puzzle 2
    signal_test_read_int();
    assert(signal_extract_message("03036732577212944063491565474664") == 84462026);
    assert(signal_extract_message("02935109699940807407585447034323") == 78725270);
    assert(signal_extract_message("03081770884921959731165446850517") == 53553731);
    assert(signal_extract_message(input) == 21130597);
}
