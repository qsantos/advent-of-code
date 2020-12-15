#include <bitset>
#include <fstream>
#include <iostream>

int main(void) {
    std::ifstream f("input");
    if (!f.is_open()) {
        std::cerr << "Could not open file" << std::endl;
    }
    std::string line, mask;
    while (getline(f, line)) {
        if (line.substr(0, 7) == "mask = ") {
            // mask = %b
            mask = line.substr(7);
        } else {
            // mem[%d] = %d
        }
    }
}
