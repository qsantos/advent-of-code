#include <iostream>
#include <fstream>
#include <unordered_set>

using std::cerr;
using std::cout;
using std::endl;
using std::ifstream;
using std::string;
using std::unordered_set;

void puzzle1(void) {
    ifstream f("input");
    if (!f.is_open()) {
        cerr << "Failed to open file" << endl;
        return;
    }

    unordered_set<int> seen;
    const int target = 2020;
    while (true) {
        int a;
        f >> a;
        int b = target - a;
        if (seen.find(b) != seen.end()) {
            cout << a * b << endl;
            return;
        }
        seen.insert(a);
    }
}

void puzzle2(void) {
    ifstream f("input");
    if (!f.is_open()) {
        cerr << "Failed to open file" << endl;
        return;
    }

    unordered_set<int> seen;
    const int target = 2020;
    while (true) {
        int a;
        f >> a;
        for (auto& b : seen) {
            int c = target - a - b;
            if (seen.find(c) != seen.end()) {
                cout << a * b * c << endl;
                return;
            }
        }
        seen.insert(a);
    }
}

int main(void) {
    puzzle1();
    puzzle2();
}
