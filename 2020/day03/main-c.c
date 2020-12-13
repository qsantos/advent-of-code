#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define GRID(x, y) (grid[(y) * (width+1) + (x)])

char* read_grid(size_t* width, size_t* height) {
    FILE* f = fopen("input", "r");
    assert(f);
    assert(fseek(f, 0, SEEK_END) == 0);
    long pos = ftell(f);
    assert(pos >= 0);
    assert(fseek(f, 0, SEEK_SET) == 0);

    size_t filesize = (size_t) pos;
    char* buffer = malloc(filesize + 1);
    assert(buffer);
    assert(fread(buffer, filesize, 1, f) == 1);
    fclose(f);

    // strip
    while (buffer[filesize - 1] == '\n') {
        filesize--;
    }
    buffer[filesize] = '\0';

    // find dimensions
    *width = (size_t) (strchr(buffer, '\n') - buffer);
    *height = (filesize + 1) / (*width + 1);
    assert((*width + 1) * *height == filesize + 1);

    return buffer;
}

size_t count_slope(char* grid, size_t width, size_t height, size_t dx, size_t dy) {
    size_t x = 0, y = 0, c = 0;
    while (y < height) {
        c += GRID(x, y) == '#';
        x = (x + dx) % width;
        y += dy;
    }
    return c;
}

void puzzle1(char* grid, size_t width, size_t height) {
    printf("%zu\n", count_slope(grid, width, height, 3, 1));
}

void puzzle2(char* grid, size_t width, size_t height) {
    size_t r = 1;
    r *= count_slope(grid, width, height, 1, 1);
    r *= count_slope(grid, width, height, 3, 1);
    r *= count_slope(grid, width, height, 5, 1);
    r *= count_slope(grid, width, height, 7, 1);
    r *= count_slope(grid, width, height, 1, 2);
    printf("%zu\n", r);
}

int main(void) {
    size_t width, height;
    char* grid = read_grid(&width, &height);

    puzzle1(grid, width, height);
    puzzle2(grid, width, height);

    free(grid);
}
