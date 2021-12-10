from collections import defaultdict
from typing import DefaultDict, Optional, Set, Tuple

GuardsSleeps = DefaultDict[int, Set[Tuple[int, int]]]


def total_sleep(sleeps: Set[Tuple[int, int]]) -> float:
    return sum(
        stop - start
        for start, stop in sleeps
    )


def sleepiest_minute(sleeps: Set[Tuple[int, int]]) -> int:
    slepts: DefaultDict[int, int] = defaultdict(int)
    for start, stop in sleeps:
        for minute in range(start, stop):
            slepts[minute] += 1
    return max(slepts, key=slepts.__getitem__)


def read_guards_sleeps(filename: str) -> GuardsSleeps:
    with open(filename) as f:
        lines = sorted(f)
    sleeping_since: Optional[int] = None
    guard_id: Optional[int] = None
    guards_sleeps: GuardsSleeps = defaultdict(set)
    for line in lines:
        # line = '[1518-11-01 00:00] Guard #10 begins shift\n'
        t, event = line.strip()[1:].split('] ', 1)
        minute = int(t[-2:])
        if event == 'falls asleep':
            sleeping_since = minute
        elif event == 'wakes up':
            assert sleeping_since is not None
            assert guard_id is not None
            guards_sleeps[guard_id].add((sleeping_since, minute))
            sleeping_since = None
        elif event.endswith(' begins shift'):
            guard_id = int(event[len('Guard #'):-len(' begins shift')])
            sleeping_since = None
        else:
            assert False
    return guards_sleeps


def strategy1(guards_sleeps: GuardsSleeps) -> int:
    sleepiest_guard = max(guards_sleeps, key=lambda guard_id: total_sleep(guards_sleeps[guard_id]))
    return sleepiest_guard * sleepiest_minute(guards_sleeps[sleepiest_guard])


def strategy2(guards_sleeps: GuardsSleeps) -> int:
    slept_minutes: DefaultDict[Tuple[int, int], int] = defaultdict(int)
    for guard_id, sleeps in guards_sleeps.items():
        for start, stop in sleeps:
            for minute in range(start, stop):
                slept_minutes[guard_id, minute] += 1
    sleepiest_minute, sleepiest_guard = max(slept_minutes, key=slept_minutes.__getitem__)
    return sleepiest_minute * sleepiest_guard


example = read_guards_sleeps('example')
input = read_guards_sleeps('input')

assert strategy1(example) == 240
assert strategy1(input) == 101194

assert strategy2(example) == 4455
assert strategy2(input) == 102095
