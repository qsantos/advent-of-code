#include <cassert>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <set>

using std::cout;
using std::endl;
using std::ifstream;
using std::map;
using std::regex;
using std::regex_match;
using std::regex_search;
using std::smatch;
using std::set;
using std::string;

int main(void) {
    // parse graph
    map<string, set<string>> parents;
    map<string, map<string, int>> children;
    ifstream f("input");
    assert(f.is_open());
    regex re_line(R"((.*?) bags contain (.*)\.)");
    regex re_subbags(R"(([0-9]+) (.*?) bags?)");
    string line;
    while (getline(f, line)) {
        smatch m_line;
        assert(regex_match(line, m_line, re_line));
        string bag = m_line[1];
        smatch m_subbag;
        string subbags = m_line[2];
        while (regex_search(subbags, m_subbag, re_subbags)) {
            int count = stoi(m_subbag[1]);
            string child = m_subbag[2];
            parents[child].insert(bag);
            children[bag][child] = count;
            subbags = m_subbag.suffix();
        }
    }

    // puzzle 1
    set<string> ascendants = {"shiny gold"};
    while (true) {
        set<string> new_ascendants = ascendants;
        for (const auto& bag: ascendants) {
            for (const auto& parent: parents[bag]) {
                new_ascendants.insert(parent);
            }
        }
        if (new_ascendants == ascendants) {
            break;
        }
        ascendants = new_ascendants;
    }
    cout << ascendants.size() - 1 << endl;

    // puzzle 2
    int total = 0;
    map<string, int> level;
    level["shiny gold"] = 1;
    while (!level.empty()) {
        map<string, int> sublevel;
        for (const auto& [bag, count]: level) {
            for (const auto& [child, mult]: children[bag]) {
                sublevel[child] += count * mult;
                total += count * mult;
            }
        }
        level = sublevel;
    }
    cout << total << endl;
}
