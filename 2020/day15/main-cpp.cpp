#include <cstdint>
#include <iostream>
#include <unordered_map>
#include <vector>

uint64_t nth_number(std::vector<uint64_t> numbers, uint64_t last_turn) {
    uint64_t last = numbers.back();
    numbers.pop_back();
    uint64_t turn = numbers.size();
    std::unordered_map<uint64_t, uint64_t> last_seen;
    for (size_t i = 0; i < numbers.size(); i++) {
        last_seen[numbers[i]] = i;
    }
    while (turn < last_turn - 1) {
        auto occurrence = last_seen.find(last);
        uint64_t number = occurrence != last_seen.end() ? turn - occurrence->second : 0;
        last_seen[last] = turn;
        last = number;
        turn++;
    }
    return last;
}

int main(void) {
    std::vector<uint64_t> numbers = {2, 0, 1, 9, 5, 19};
    std::cout << nth_number(numbers, 2020) << std::endl;
    std::cout << nth_number(numbers, 30000000) << std::endl;
}
