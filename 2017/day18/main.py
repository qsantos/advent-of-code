from collections import deque
from typing import Deque, Dict, List, Tuple


Instruction = Tuple[str, str, str]


def read_instructions(filename: str) -> List[Instruction]:
    instructions = []
    with open(filename) as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) == 2:
                opcode, a = parts
                b = ''
            else:
                opcode, a, b = parts
            instructions.append((opcode, a, b))
    return instructions


def recover_frequency(instructions: List[Instruction]) -> int:
    mem: Dict[str, int] = {}
    last_played_frequency = 0
    ip = 0
    while True:
        opcode, a, b = instructions[ip]
        try:
            vb = int(b)
        except ValueError:
            vb = mem.get(b, 0)
        if opcode == 'snd':
            last_played_frequency = mem.get(a, 0)
        elif opcode == 'set':
            mem[a] = vb
        elif opcode == 'add':
            mem[a] = mem.get(a, 0) + vb
        elif opcode == 'mul':
            mem[a] = mem.get(a, 0) * vb
        elif opcode == 'mod':
            mem[a] = mem.get(a, 0) % vb
        elif opcode == 'rcv':
            if mem.get(a, 0) != 0:
                return last_played_frequency
        elif opcode == 'jgz':
            if mem.get(a, 0) > 0:
                ip += vb - 1
        else:
            assert False, opcode
        ip += 1


def run_instruction(
    instructions: List[Instruction],
    mem: Dict[str, int],
    ip: int,
    snd: Deque[int],
    rcv: Deque[int],
) -> Tuple[int, bool, bool]:
    opcode, a, b = instructions[ip]
    waiting = False
    sent_something = False
    try:
        va = int(a)
    except ValueError:
        va = mem.get(a, 0)
    try:
        vb = int(b)
    except ValueError:
        vb = mem.get(b, 0)
    if opcode == 'snd':
        snd.append(va)
        sent_something = True
    elif opcode == 'set':
        mem[a] = vb
    elif opcode == 'add':
        mem[a] = mem.get(a, 0) + vb
    elif opcode == 'mul':
        mem[a] = mem.get(a, 0) * vb
    elif opcode == 'mod':
        mem[a] = mem.get(a, 0) % vb
    elif opcode == 'rcv':
        if rcv:
            mem[a] = rcv.popleft()
        else:
            waiting = True
            ip -= 1
    elif opcode == 'jgz':
        if va > 0:
            ip += vb - 1
    else:
        assert False, opcode
    ip += 1
    return ip, waiting, sent_something


def times_1_sent(instructions: List[Instruction]) -> int:
    mem0 = {'p': 0}
    mem1 = {'p': 1}
    queue0: Deque[int] = deque()
    queue1: Deque[int] = deque()
    ip0 = 0
    ip1 = 0
    count = 0
    while True:
        ip0, wait0, sent_something0 = run_instruction(instructions, mem0, ip0, queue0, queue1)
        ip1, wait1, sent_something1 = run_instruction(instructions, mem1, ip1, queue1, queue0)
        if sent_something1:
            count += 1
        if wait0 and wait1:
            break
    return count


def main() -> None:
    example1 = read_instructions('example1')
    example2 = read_instructions('example2')
    input = read_instructions('input')

    assert recover_frequency(example1) == 4
    assert recover_frequency(input) == 3188

    assert times_1_sent(example2) == 3
    assert times_1_sent(input) == 7112


if __name__ == '__main__':
    main()
