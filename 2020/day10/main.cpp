#include <algorithm>
#include <cassert>
#include <iostream>
#include <iterator>
#include <fstream>
#include <numeric>
#include <vector>

using std::adjacent_difference;
using std::cout;
using std::endl;
using std::istream_iterator;
using std::ifstream;
using std::sort;
using std::vector;

int main(void) {
    ifstream f("input");
    assert(f.is_open());
    vector<unsigned long> values(istream_iterator<unsigned long>(f), istream_iterator<unsigned long>{});

    // puzzle 1
    sort(values.begin(), values.end());
    values.insert(values.begin(), 0);
    unsigned long max = *(values.end() - 1) + 3;
    values.push_back(max);
    vector<unsigned long> diffs(values.size());
    adjacent_difference(values.begin(), values.end(), diffs.begin());
    size_t ones = count(diffs.begin(), diffs.end(), 1);
    size_t threes = count(diffs.begin(), diffs.end(), 3);
    cout << ones * threes << endl;

    /// puzzle 2
    vector<unsigned long> S(max + 1);
    S[0] = 1;
    for (const auto& value: values) {
        for (unsigned long diff = 1; diff < 4; diff++) {
            if (diff <= value) {
                S[value] += S[value - diff];
            }
        }
    }
    cout << *(S.end() - 1) << endl;
}
