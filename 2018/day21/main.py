from typing import Iterator, List, Tuple

Instruction = Tuple[str, int, int, int]
Program = Tuple[int, List[Instruction]]


def read_program(filename: str) -> Program:
    with open(filename) as f:
        line = next(f)
        assert line.startswith('#ip ')
        ip_register = int(line.strip()[len('#ip '):])
        instructions = []
        for line in f:
            opcode, a, b, c = line.strip().split()
            instructions.append((opcode, int(a), int(b), int(c)))
    return ip_register, instructions


def run_opcode_by_name(mem: List[int], name: str, a: int, b: int, c: int) -> None:
    if name == 'addr':
        mem[c] = mem[a] + mem[b]
    elif name == 'addi':
        mem[c] = mem[a] + b
    elif name == 'mulr':
        mem[c] = mem[a] * mem[b]
    elif name == 'muli':
        mem[c] = mem[a] * b
    elif name == 'banr':
        mem[c] = mem[a] & mem[b]
    elif name == 'bani':
        mem[c] = mem[a] & b
    elif name == 'borr':
        mem[c] = mem[a] | mem[b]
    elif name == 'bori':
        mem[c] = mem[a] | b
    elif name == 'setr':
        mem[c] = mem[a]
    elif name == 'seti':
        mem[c] = a
    elif name == 'gtir':
        mem[c] = 1 if a > mem[b] else 0
    elif name == 'gtri':
        mem[c] = 1 if mem[a] > b else 0
    elif name == 'gtrr':
        mem[c] = 1 if mem[a] > mem[b] else 0
    elif name == 'eqir':
        mem[c] = 1 if a == mem[b] else 0
    elif name == 'eqri':
        mem[c] = 1 if mem[a] == b else 0
    elif name == 'eqrr':
        mem[c] = 1 if mem[a] == mem[b] else 0
    else:
        assert False, name


def iter_keys(program: Program) -> Iterator[int]:
    r4 = 0
    while True:
        r3 = r4 | 65536
        r4 = 10552971
        while True:
            r4 += r3 & 255
            r4 &= 16777215
            r4 *= 65899
            r4 &= 16777215
            if r3 < 256:
                break
            r3 //= 256
        yield r4


def main() -> None:
    input = read_program('input')

    it = iter_keys(input)
    key = next(it)
    assert key == 103548

    prev = key
    seen = {key}
    for key in it:
        if key in seen:
            print(prev)
            break
        seen.add(key)
        prev = key


if __name__ == '__main__':
    main()
