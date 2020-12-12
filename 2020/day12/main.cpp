#include <cassert>
#include <fstream>
#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::ifstream;
using std::string;
using std::vector;

void puzzle1(vector<string> instructions) {
    int de = 1, dn = 0;
    int e = 0, n = 0;
    for (const auto& instruction: instructions) {
        char code = instruction[0];
        int value = stoi(instruction.substr(1));
        switch (code) {
        case 'E': e += value; break;
        case 'W': e -= value; break;
        case 'N': n += value; break;
        case 'S': n -= value; break;
        case 'R':
            value = 360 - value;
        case 'L':
            assert(value % 90 == 0);
            value %= 360;
            while (value) {
                int tmp = de;
                de = -dn;
                dn = tmp;
                value -= 90;
            }
            break;
        case 'F':
            n += dn * value;
            e += de * value;
            break;
        default:
            assert(0);
        }
    }
    cout << abs(e) + abs(n) << endl;
}

void puzzle2(vector<string> instructions) {
    int se = 0, sn = 0;
    int we = 10, wn = 1;
    for (const auto& instruction: instructions) {
        char code = instruction[0];
        int value = stoi(instruction.substr(1));
        switch (code) {
        case 'E': we += value; break;
        case 'W': we -= value; break;
        case 'N': wn += value; break;
        case 'S': wn -= value; break;
        case 'L':
        case 'R':
            if (code == 'R') {
                value = 360 - value;
            }
            assert(value % 90 == 0);
            value %= 360;
            while (value) {
                int tmp = we;
                we = -wn;
                wn = tmp;
                value -= 90;
            }
            break;
        case 'F':
            sn += wn * value;
            se += we * value;
            break;
        default:
            assert(0);
        }
    }
    cout << abs(se) + abs(sn) << endl;
}

int main(void) {
    ifstream f("input");
    assert(f.is_open());
    vector<string> instructions;
    string line;
    while (getline(f, line)) {
        instructions.push_back(line);
    }

    puzzle1(instructions);
    puzzle2(instructions);
}
