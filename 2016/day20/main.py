from typing import List, Tuple

debug = False

Interval = Tuple[int, int]
IntervalSet = List[Interval]


def read_intervals(filename: str) -> List[Interval]:
    intervals = []
    with open(filename) as f:
        for line in f:
            a, b = line.strip().split('-')
            intervals.append((int(a), int(b)))
    return intervals


def lowest_unblocked_ip(intervals: List[Interval]) -> int:
    cur = 0
    while True:
        for low, high in intervals:
            if low <= cur <= high:
                cur = high + 1
                break
        else:
            return cur
    assert False


def merge_interval_in_set(interval_set: IntervalSet, new_interval: Interval) -> None:
    if debug:
        print(f'Merging {new_interval} into {interval_set}')
    na, nb = new_interval

    i = 0
    while i < len(interval_set):
        a, b = interval_set[i]
        if na <= b:
            break
        i += 1
    else:
        if debug:
            print('Strictly greater: appending')
        interval_set.append((na, nb))
        return

    a, b = interval_set[i]
    if debug:
        print(f'The upper bound of {a}-{b} is greater than the lower bound of the new interval')

    if nb < a:
        if debug:
            print(f'The new interval is before {a}-{b}')
        interval_set.insert(i, (na, nb))
        return
    elif nb < b:
        if debug:
            print(f'The new interval might extend {a}-{b} to the left')
        interval_set[i] = (min(a, na), b)
        return
    elif na < a:
        if debug:
            print(f'The new interval includes {a}-{b}')
    else:
        if debug:
            print(f'The new interval extends {a}-{b} to the right')
        na = a

    j = i + 1
    while j < len(interval_set):
        a, b = interval_set[j]
        if nb <= b:
            break
        j += 1
    else:
        if debug:
            print('The new interval includes everything else')
        interval_set[i:] = [(na, nb)]
        return

    a, b = interval_set[j]
    if debug:
        print(f'The upper bound of {a}-{b} is greater than that of the new interval')

    if nb < a:
        if debug:
            print(f'The upper bound of the new interval is not in {a}-{b}, keep both')
        # the last interval is disjoint from the new interval
        pass
    else:
        # merge the last interval with the new interval
        if debug:
            print(f'The upper bound of the new interval is in {a}-{b}, merge the two')
        nb = b
        j += 1

    if debug:
        print(f'Replacing intervals {i} to {j} (excluded)')
    interval_set[i:j] = [(na, nb)]


def count_unblocked_ips(intervals: List[Interval]) -> int:
    interval_set: IntervalSet = []
    for interval in intervals:
        merge_interval_in_set(interval_set, interval)
    blocked_ips = 0
    for a, b in interval_set:
        blocked_ips += b - a + 1
    return 2**32 - blocked_ips


def main() -> None:
    example = read_intervals('example')
    input = read_intervals('input')

    assert lowest_unblocked_ip(example) == 3
    assert lowest_unblocked_ip(input) == 4793564

    interval_set: IntervalSet = []
    merge_interval_in_set(interval_set, (125, 175))
    assert interval_set == [(125, 175)]
    merge_interval_in_set(interval_set, (100, 200))
    assert interval_set == [(100, 200)]
    merge_interval_in_set(interval_set, (150, 250))
    assert interval_set == [(100, 250)]
    merge_interval_in_set(interval_set, (300, 500))
    assert interval_set == [(100, 250), (300, 500)]
    merge_interval_in_set(interval_set, (255, 255))
    assert interval_set == [(100, 250), (255, 255), (300, 500)]
    merge_interval_in_set(interval_set, (260, 260))
    assert interval_set == [(100, 250), (255, 255), (260, 260), (300, 500)]
    merge_interval_in_set(interval_set, (290, 290))
    assert interval_set == [(100, 250), (255, 255), (260, 260), (290, 290), (300, 500)]
    merge_interval_in_set(interval_set, (250, 350))
    assert interval_set == [(100, 500)]

    print(count_unblocked_ips(input))


if __name__ == '__main__':
    main()
