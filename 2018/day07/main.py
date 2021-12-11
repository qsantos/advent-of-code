from copy import deepcopy
from heapq import heapify, heappop, heappush
from string import ascii_uppercase
from typing import Dict, List, Set, Tuple

Dependencies = Dict[str, Set[str]]


def read_dependencies(filename: str) -> Dependencies:
    ret: Dependencies = {}
    with open(filename) as f:
        for line in f:
            # line = 'Step A must be finished before step B can begin.\n'
            _, a, _, _, _, _, _, b, _, _ = line.split()
            if b in ret:
                ret[b].add(a)
            else:
                ret[b] = {a}
    return ret


def topological_sort(dependencies: Dependencies) -> str:
    dependencies = deepcopy(dependencies)
    dangling_tasks = list({
        task
        for deps in dependencies.values()
        for task in deps
        if task not in dependencies
    })
    ret = []
    heapify(dangling_tasks)
    while dangling_tasks:
        task = heappop(dangling_tasks)
        ret.append(task)
        for other, deps in dependencies.items():
            if task in deps:
                deps.remove(task)
                if not deps:
                    heappush(dangling_tasks, other)
    return ''.join(ret)


def task_duration(base_time: int, task: str) -> int:
    return base_time + ascii_uppercase.index(task) + 1


def total_time(dependencies: Dependencies, base_time: int, n_workers: int) -> int:
    dependencies = deepcopy(dependencies)
    dangling_tasks = list({
        task
        for deps in dependencies.values()
        for task in deps
        if task not in dependencies
    })
    in_process_tasks: List[Tuple[int, str, int]] = []
    t = 0
    available_workers = set(range(n_workers))
    heapify(dangling_tasks)
    while dangling_tasks or in_process_tasks:
        if available_workers and dangling_tasks:
            task = heappop(dangling_tasks)
            worker = min(available_workers)
            work_end = t + task_duration(base_time, task)
            available_workers.remove(worker)
            heappush(in_process_tasks, (work_end, task, worker))
        else:
            assert in_process_tasks
            t, task, worker = heappop(in_process_tasks)
            available_workers.add(worker)
            for other, deps in dependencies.items():
                if task in deps:
                    deps.remove(task)
                    if not deps:
                        heappush(dangling_tasks, other)
    return t


example = read_dependencies('example')
input = read_dependencies('input')

assert topological_sort(example) == 'CABDFE'
assert topological_sort(input) == 'ABGKCMVWYDEHFOPQUILSTNZRJX'

assert total_time(example, 0, 2) == 15
assert total_time(input, 60, 5) == 898
