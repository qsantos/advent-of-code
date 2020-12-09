#include <algorithm>
#include <cassert>
#include <deque>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <vector>

using std::cout;
using std::deque;
using std::endl;
using std::ifstream;
using std::istream_iterator;
using std::max_element;
using std::min_element;
using std::set;
using std::string;
using std::vector;

static const size_t preamble_size = 25;

bool is_sum_of_two(set<unsigned long> values, unsigned long target) {
    for (const auto& value: values) {
        if (values.find(target - value) != values.end()) {
            return true;
        }
    }
    return false;
}

unsigned long find_invalid(vector<unsigned long> values) {
    deque<unsigned long> preamble(values.begin(), values.begin() + preamble_size);
    for (size_t i = preamble_size; i < values.size(); i++) {
        unsigned long value = values[i];
        if (!is_sum_of_two(set<unsigned long>(preamble.begin(), preamble.end()), value)) {
            return value;
        }
        preamble.pop_front();
        preamble.push_back(value);
    }
    assert(0);
}


vector<unsigned long> find_span(vector<unsigned long> values, unsigned long target) {
    size_t n = values.size();
    for (size_t i = 0; i < n; i++) {
        size_t sum = 0;
        for (size_t j = i + 1; j < n; j++) {
            sum += values[j];
            if (sum == target) {
                return vector<unsigned long>(values.begin() + i, values.begin() + j - 1);
            }
        }
    }
    assert(0);
}

int main(void) {
    ifstream f("input");
    assert(f.is_open());
    vector<unsigned long> values(istream_iterator<unsigned long>(f), istream_iterator<unsigned long>{});

    // puzzle 1
    unsigned long invalid = find_invalid(values);
    cout << invalid << endl;

    // puzzle 2
    auto span = find_span(values, invalid);
    auto min = *min_element(span.begin(), span.end());
    auto max = *max_element(span.begin(), span.end());
    cout << min + max << endl;
}
