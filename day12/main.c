#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define N_MOONS 4
#define N_AXES 3

struct MoonAxis {
    int pos;
    int vel;
};

struct Cycle {
    uint64_t lambda;
    uint64_t mu;
};

typedef struct MoonAxis MoonsAxis[N_MOONS];
typedef MoonsAxis Moons[N_AXES];

Moons example1 = {
    {{-1, 0}, {  2, 0}, { 4, 0}, { 3, 0}},
    {{ 0, 0}, {-10, 0}, {-8, 0}, { 5, 0}},
    {{ 2, 0}, { -7, 0}, { 8, 0}, {-1, 0}},
};

Moons example2 = {
    {{ -8, 0}, { 5, 0}, { 2, 0}, { 9, 0}},
    {{-10, 0}, { 5, 0}, {-7, 0}, {-8, 0}},
    {{  0, 0}, {10, 0}, { 3, 0}, {-3, 0}},
};

void moons_axis_copy(MoonsAxis dst, MoonsAxis src) {
    memcpy(dst, src, sizeof(MoonsAxis));
}

void moons_copy(Moons dst, Moons src) {
    memcpy(dst, src, sizeof(Moons));
}

int state_eq(MoonsAxis a, MoonsAxis b) {
    for (int i = 0; i < N_MOONS; i += 1) {
        if (a[i].pos != b[i].pos) {
            return 0;
        }
        if (a[i].vel != b[i].vel) {
            return 0;
        }
    }
    return 1;
}

void update_velocities(MoonsAxis moons_axis) {
    for (int i = 0; i < N_MOONS; i += 1) {
        for (int j = 0; j < i; j += 1) {
            if (moons_axis[i].pos < moons_axis[j].pos) {
                moons_axis[i].vel += 1;
            } else if (moons_axis[i].pos > moons_axis[j].pos) {
                moons_axis[i].vel -= 1;
            }

            if (moons_axis[j].pos < moons_axis[i].pos) {
                moons_axis[j].vel += 1;
            } else if (moons_axis[j].pos > moons_axis[i].pos) {
                moons_axis[j].vel -= 1;
            }
        }
    }
}

void update_positions(MoonsAxis moons_axis) {
    for (int i = 0; i < N_MOONS; i += 1) {
        moons_axis[i].pos += moons_axis[i].vel;
    }
}

void do_step(MoonsAxis moons_axis) {
    update_velocities(moons_axis);
    update_positions(moons_axis);
}

int potential_energy(Moons moons, int i) {
    int r = 0;
    for (int axis = 0; axis < N_AXES; axis += 1) {
        r += abs(moons[axis][i].pos);
    }
    return r;
}

int kinetic_energy(Moons moons, int i) {
    int r = 0;
    for (int axis = 0; axis < N_AXES; axis += 1) {
        r += abs(moons[axis][i].vel);
    }
    return r;
}

int total_energy(Moons moons, int i) {
    return potential_energy(moons, i) * kinetic_energy(moons, i);
}

int system_energy(Moons moons) {
    int r = 0;
    for (int i = 0; i < N_MOONS; i += 1) {
        r += total_energy(moons, i);
    }
    return r;
}

int simulate(Moons x0, int steps) {
    Moons moons;
    moons_copy(moons, x0);
    for (int axis = 0; axis < N_AXES; axis += 1) {
        for (int step = 0; step < steps; step += 1) {
            do_step(moons[axis]);
        }
    }
    return system_energy(moons);
}

struct Cycle floyd(MoonsAxis x0) {
    /* tortoise = f(x0) */
    MoonsAxis tortoise;
    moons_axis_copy(tortoise, x0);
    do_step(tortoise);

    /* hare = f(f(x0)) */
    MoonsAxis hare;
    moons_axis_copy(hare, x0);
    do_step(hare);
    do_step(hare);

    while (!state_eq(tortoise, hare)) {
        do_step(tortoise);
        do_step(hare);
        do_step(hare);
    }

    uint64_t mu = 0;
    moons_axis_copy(tortoise, x0);
    while (!state_eq(tortoise, hare)) {
        do_step(tortoise);
        do_step(hare);
        mu += 1;
    }

    uint64_t lambda = 1;
    moons_axis_copy(hare, tortoise);
    do_step(hare);
    while (!state_eq(tortoise, hare)) {
        do_step(hare);
        lambda += 1;
    }

    return (struct Cycle){lambda, mu};
}

struct Cycle brent(MoonsAxis x0) {
    uint64_t power = 1;
    uint64_t lambda = 1;

    /* tortoise = x0 */
    MoonsAxis tortoise;
    moons_axis_copy(tortoise, x0);

    /* hare = f(x0) */
    MoonsAxis hare;
    moons_axis_copy(hare, x0);
    do_step(hare);

    while (!state_eq(tortoise, hare)) {
        if (power == lambda) {
            moons_axis_copy(tortoise, hare);
            power *= 2;
            lambda = 0;
        }
        do_step(hare);
        lambda += 1;
    }

    /* tortoise = hare = x0 */
    moons_axis_copy(tortoise, x0);
    moons_axis_copy(hare, x0);

    for (uint64_t i = 0; i < lambda; i += 1) {
        do_step(hare);
    }

    uint64_t mu = 0;
    while (!state_eq(tortoise, hare)) {
        do_step(tortoise);
        do_step(hare);
        mu += 1;
    }

    return (struct Cycle){lambda, mu};
}

uint64_t gcd(uint64_t a, uint64_t b) {
    if (b == 0) {
        return a;
    }
    if (a < b) {
        return gcd(b, a);
    }
    return gcd(b, a % b);
}

uint64_t lcm(uint64_t a, uint64_t b) {
    return a / gcd(a, b) * b;
}

struct Cycle global_cycle(Moons moons, struct Cycle (*method)(MoonsAxis)) {
    struct Cycle cycles[N_AXES];
    for (int axis = 0; axis < N_AXES; axis += 1) {
        cycles[axis] = method(moons[axis]);
    }

    uint64_t global_lambda = cycles[0].lambda;
    for (uint64_t axis = 1; axis < N_AXES; axis += 1) {
        global_lambda = lcm(global_lambda, cycles[axis].lambda);
    }

    uint64_t global_mu = 0;  /* TODO */

    return (struct Cycle){global_lambda, global_mu};
}

uint64_t first_repetition(Moons moons, struct Cycle (*method)(MoonsAxis)) {
    struct Cycle cycle = global_cycle(moons, method);
    return cycle.lambda + cycle.mu;
}

void read_moons(Moons moons, const char* filename) {
    FILE* f = fopen(filename, "r");
    assert(f != NULL);

    char line[2048];
    for (int i = 0; i < N_MOONS; i += 1) {
        fgets(line, sizeof(line), f);
        int x, y, z;
        sscanf(line, "<x=%d, y=%d, z=%d>\n", &x, &y, &z);
        moons[0][i].pos = x;
        moons[1][i].pos = y;
        moons[2][i].pos = z;
        moons[0][i].vel = 0;
        moons[1][i].vel = 0;
        moons[2][i].vel = 0;
    }

    fclose(f);
}

int main(void) {
    assert(gcd(1, 1) == 1);
    assert(gcd(6, 10) == 2);
    assert(gcd(10, 6) == 2);
    assert(gcd(128, 128) == 128);

    Moons moons;
    read_moons(moons, "input");

    // puzzle 1
    assert(simulate(example1, 10) == 179);
    assert(simulate(example2, 100) == 1940);
    assert(simulate(moons, 1000) == 7202);

    // puzzle 2
    assert(first_repetition(example1, floyd) == 2772);
    assert(first_repetition(example2, floyd) == 4686774924);
    assert(first_repetition(moons, floyd) == 537881600740876);
}
