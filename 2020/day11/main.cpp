#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <vector>

using std::count;
using std::cout;
using std::endl;
using std::ifstream;
using std::string;
using std::vector;

using Grid = vector<vector<char>>;

int count(const Grid& grid) {
    int ret = 0;
    for (const auto& r: grid) {
        ret += (int) count(r.begin(), r.end(), '#');
    }
    return ret;
}

Grid next_adjacent(const Grid& grid) {
    int rows = (int) grid.size();
    int cols = (int) grid[0].size();
    Grid ret;
    for (int row = 0; row < rows; row++) {
        vector<char> r;
        for (int col = 0; col < cols; col++) {
            if (grid[row][col] == '.') {
                r.push_back('.');
                continue;
            }
            int count = 0;
            for (int nr = row - 1; nr <= row + 1; nr++) {
                for (int nc = col - 1; nc <= col + 1; nc++) {
                    if ((nr != row || nc != col) && (0 <= nr && nr < rows && 0 <= nc && nc < cols)) {
                        count += grid[nr][nc] == '#';
                    }
                }
            }
            if (count == 0) {
                r.push_back('#');
            } else if (count >= 4) {
                r.push_back('L');
            } else {
                r.push_back(grid[row][col]);
            }
        }
        ret.push_back(r);
    }
    return ret;
}

Grid next_line_of_sight(const Grid& grid) {
    int rows = (int) grid.size();
    int cols = (int) grid[0].size();
    Grid ret;
    for (int row = 0; row < rows; row++) {
        vector<char> r;
        for (int col = 0; col < cols; col++) {
            if (grid[row][col] == '.') {
                r.push_back('.');
                continue;
            }
            int count = 0;
            for (int dr = -1; dr <= +1; dr++) {
                for (int dc = -1; dc <= +1; dc++) {
                    if (dr == 0 && dc == 0) {
                        continue;
                    }
                    int i = 1;
                    while (true) {
                        int nr = row + dr * i;
                        int nc = col + dc * i;
                        if (!(0 <= nr && nr < rows && 0 <= nc && nc < cols)) {
                            break;
                        }
                        char c = grid[nr][nc];
                        if (c == '#') {
                            count++;
                            break;
                        } else if (c == 'L') {
                            break;
                        }
                        i++;
                    }
                }
            }
            if (count == 0) {
                r.push_back('#');
            } else if (count >= 5) {
                r.push_back('L');
            } else {
                r.push_back(grid[row][col]);
            }
        }
        ret.push_back(r);
    }
    return ret;
}

int puzzle1(Grid grid) {
    while (true) {
        Grid new_grid = next_adjacent(grid);
        if (new_grid == grid) {
            return count(grid);
        }
        grid = new_grid;
    }
}

int puzzle2(Grid grid) {
    while (true) {
        Grid new_grid = next_line_of_sight(grid);
        if (new_grid == grid) {
            return count(grid);
        }
        grid = new_grid;
    }
}

int main(void) {
    ifstream f("input");
    assert(f.is_open());
    Grid grid;
    string line;
    while (getline(f, line)) {
        grid.emplace_back(line.begin(), line.end());
    }

    cout << puzzle1(grid) << endl;
    cout << puzzle2(grid) << endl;
}
