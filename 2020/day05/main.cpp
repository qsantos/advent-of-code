#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_set>

using std::cout;
using std::endl;
using std::ifstream;
using std::max_element;
using std::string;
using std::unordered_set;

int main(void) {
    ifstream f("input");
    string line;
    unordered_set<int> seat_ids;
    while (getline(f, line)) {
        replace(line.begin(), line.end(), 'F', '0');
        replace(line.begin(), line.end(), 'B', '1');
        replace(line.begin(), line.end(), 'L', '0');
        replace(line.begin(), line.end(), 'R', '1');
        seat_ids.insert(stoi(line, NULL, 2));
    }

    // puzzle 1
    cout << *max_element(seat_ids.begin(), seat_ids.end()) << endl;

    // puzzle 2
    for (int seat_id = 0; seat_id < 1<<10; seat_id++) {
        if (seat_ids.find(seat_id) != seat_ids.end()) {
            continue;
        }
        if (seat_ids.find(seat_id - 1) == seat_ids.end()) {
            continue;
        }
        if (seat_ids.find(seat_id + 1) == seat_ids.end()) {
            continue;
        }
        cout << seat_id << endl;
    }
}
