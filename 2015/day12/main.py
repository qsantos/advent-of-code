import json
from typing import List, Dict, Union

with open('input') as f:
    data = json.load(f)


def sum_numbers(thing: Union[List, Dict, int], *, ignore: str = '') -> int:
    if isinstance(thing, int):
        return thing
    elif isinstance(thing, str):
        return 0
    elif isinstance(thing, list):
        return sum(sum_numbers(subthing, ignore=ignore) for subthing in thing)
    elif isinstance(thing, dict):
        if ignore in thing.values():
            return 0
        else:
            return sum(sum_numbers(subthing, ignore=ignore) for subthing in thing.values())
    else:
        assert False, type(thing)


assert sum_numbers(json.loads('[1,2,3]')) == 6
assert sum_numbers(json.loads('{"a":2,"b":4}')) == 6
assert sum_numbers(json.loads('[[[3]]]')) == 3
assert sum_numbers(json.loads('{"a":{"b":4},"c":-1}')) == 3
assert sum_numbers(json.loads('{"a":[-1,1]}')) == 0
assert sum_numbers(json.loads('[-1,{"a":1}]')) == 0
assert sum_numbers(json.loads('[]')) == 0
assert sum_numbers(json.loads('{}')) == 0

assert sum_numbers(data) == 191164

assert sum_numbers(json.loads('[1,2,3]'), ignore='red') == 6
assert sum_numbers(json.loads('[1,{"c":"red","b":2},3]'), ignore='red') == 4
assert sum_numbers(json.loads('{"d":"red","e":[1,2,3,4],"f":5}'), ignore='red') == 0
assert sum_numbers(json.loads('[1,"red",5]'), ignore='red') == 6

assert sum_numbers(data, ignore='red') == 87842
