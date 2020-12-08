#include <cassert>
#include <fstream>
#include <iostream>
#include <tuple>
#include <unordered_set>
#include <vector>

using std::cout;
using std::endl;
using std::ifstream;
using std::tuple;
using std::string;
using std::unordered_set;
using std::vector;

enum class Op {
    NOP = 0,
    ACC = 1,
    JMP = 2,
};

using Program = vector<tuple<Op, int>>;

Program read_program(void) {
    ifstream f("input");
    assert(f.is_open());
    Program program;
    string line;
    while (getline(f, line)) {
        string op_str = line.substr(0, 3);
        int arg = stoi(line.substr(4));
        Op op;
        if (op_str == "acc") {
            op = Op::ACC;
        } else if (op_str == "jmp") {
            op = Op::JMP;
        } else if (op_str == "nop") {
            op = Op::NOP;
        } else {
            assert(0);
        }
        program.push_back({op, arg});
    }
    return program;
}

tuple<bool, int> run(const Program& program) {
    size_t ip = 0;
    int acc = 0;
    unordered_set<size_t> seen;
    while (ip < program.size()) {
        if (seen.find(ip) != seen.end()) {
            return {false, acc};
        }
        seen.insert(ip);
        const auto& [instr, arg] = program[ip];
        switch (instr) {
        case Op::ACC:
            acc += arg;
            ip += 1;
            break;
        case Op::JMP:
            ip += arg;
            break;
        case Op::NOP:
            ip += 1;
            break;
        default:
            assert(0);
        }
    }
    return {true, acc};
}

int main(void) {
    auto program = read_program();

    // puzzle 1
    {
        auto [ok, ret] = run(program);
        assert(!ok);
        cout << ret << endl;
    }

    // puzzle 2
    for (size_t ip = 0; ip < program.size(); ip++) {
        const auto [instr, arg] = program[ip];
        if (instr == Op::ACC) {
            continue;
        } else if (instr == Op::JMP) {
            program[ip] = {Op::NOP, arg};
        } else if (instr == Op::NOP) {
            program[ip] = {Op::JMP, arg};
        } else {
            assert(0);
        }
        const auto& [ok, ret] = run(program);
        if (ok) {
            cout << ret << endl;
            break;
        }
        program[ip] = {instr, arg};
    }
}
