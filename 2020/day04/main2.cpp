#include <cassert>
#include <cstdio>
#include <fstream>
#include <map>
#include <regex>
#include <sstream>
#include <vector>

using std::ifstream;
using std::invalid_argument;
using std::map;
using std::regex;
using std::regex_match;
using std::smatch;
using std::stringstream;
using std::string;
using std::vector;

typedef map<string, string> Passport;

vector<string>split(string s, string delimiter) {
    vector<string> ret;
    size_t last = 0;
    size_t next = 0;
    size_t offset = delimiter.length();
    while ((next = s.find(delimiter, last)) != string::npos) {
        ret.push_back(s.substr(last, next - last));
        last = next + offset;
    }
    ret.push_back(s.substr(last));
    return ret;
}

vector<string>split_of(string s, string delimiter) {
    vector<string> ret;
    size_t last = 0;
    size_t next = 0;
    while ((next = s.find_first_of(delimiter, last)) != string::npos) {
        ret.push_back(s.substr(last, next - last));
        last = next + 1;
    }
    ret.push_back(s.substr(last));
    return ret;
}

Passport parse_passport(string passport) {
    Passport ret;
    for (auto& field: split_of(passport, " \n")) {
        size_t pos = field.find(":");
        if (pos == string::npos) {
            continue;
        }
        ret[field.substr(0, pos)] = field.substr(pos + 1);
    }
    return ret;
}

vector<Passport> parse_passports(void) {
    ifstream f("input");
    assert(f.is_open());
    stringstream ss;
    ss << f.rdbuf();
    string data = ss.str();

    vector<Passport> ret;
    for (auto& part: split(data, "\n\n")) {
        ret.push_back(parse_passport(part));
    }
    return ret;
}

void puzzle1(vector<Passport> passports) {
    int n_valid_passports = 0;
    for (auto& passport: passports) {
        int ok = 1;
        for (auto& key: {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}) {
            if (passport.find(string(key)) == passport.end()) {
                ok = 0;
                break;
            }
        }
        n_valid_passports += ok;
    }
    printf("%d\n", n_valid_passports);
}

void puzzle2(vector<Passport> passports) {
    regex re_hgt("([0-9]+)(cm|in)");
    regex re_hcl("#[0-9a-f]{6}");
    regex re_ecl("amb|blu|brn|gry|grn|hzl|oth");
    regex re_pid("[0-9]{9}");
    smatch m;
    int n_valid_passports = 0;
    for (auto& passport: passports) {
        int ok = 1;
        try {
            ok &= 1920 <= stoi(passport["byr"]) && stoi(passport["byr"]) <= 2002;
            ok &= 2010 <= stoi(passport["iyr"]) && stoi(passport["iyr"]) <= 2020;
            ok &= 2020 <= stoi(passport["eyr"]) && stoi(passport["eyr"]) <= 2030;
            ok &= regex_match(passport["hgt"], m, re_hgt);
            if (ok) {
                int l = stoi(m[1]);
                if (m[2] == "cm") { ok &= 150 <= l && l <= 193; }
                else              { ok &= 59 <= l && l <= 76; }
            }
            ok &= regex_match(passport["hcl"], m, re_hcl);
            ok &= regex_match(passport["ecl"], m, re_ecl);
            ok &= regex_match(passport["pid"], m, re_pid);
        } catch (invalid_argument&) {
            ok = 0;
        }
        n_valid_passports += ok;
    }
    printf("%d\n", n_valid_passports);
}

int main(void) {
    auto passports = parse_passports();
    puzzle1(passports);
    puzzle2(passports);
}
