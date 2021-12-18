from typing import List


def read_packages(filename: str) -> List[int]:
    with open(filename) as f:
        return [
            int(line.strip())
            for line in f
        ]


def can_balance(packages: List[int], n_groups: int) -> bool:
    if n_groups == 1:
        return True

    def aux(i: int, weight: int) -> bool:
        if i == len(packages):
            return weight == target_weight and can_balance(list(available), n_groups - 1)
        elif weight > target_weight:
            return False
        a = aux(i + 1, weight)
        if a:
            return True
        package = packages[i]
        available.remove(package)
        b = aux(i + 1, weight + package)
        available.add(package)
        return b
    available = set(packages)
    target_weight, r = divmod(sum(packages), n_groups)
    assert r == 0
    return aux(0, 0)


def balance_packages(packages: List[int], n_groups: int) -> int:
    def aux(rem: int, i: int, weight: int, quantum: int) -> int:
        if rem == 0:
            if weight == target_weight and can_balance(list(available), n_groups - 1):
                return quantum
            else:
                return -1
        elif i == len(packages):
            return -1
        elif weight > target_weight:
            return -1
        q1 = aux(rem, i + 1, weight, quantum)
        package = packages[i]
        available.remove(package)
        q2 = aux(rem - 1, i + 1, weight + package, quantum * package)
        available.add(package)
        if q1 < 0:
            return q2
        elif q2 < 0:
            return q1
        else:
            return min(q1, q2)
    available = set(packages)
    assert len(available) == len(packages)
    target_weight, r = divmod(sum(packages), n_groups)
    assert r == 0
    for first_group_size in range(1, len(packages)):
        r = aux(first_group_size, 0, 0, 1)
        if r >= 0:
            return r
    return -1


example = read_packages('example')
input = read_packages('input')

assert balance_packages(example, 3) == 99
assert balance_packages(input, 3) == 10723906903
assert balance_packages(example, 4) == 44
assert balance_packages(input, 4) == 74850409
