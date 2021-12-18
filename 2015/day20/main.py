def presents_of_house1(n: int) -> int:
    elves = 0
    d = 1
    while d * d < n:
        if n % d == 0:
            elves += d + n // d
        d += 1
    if d * d == n:
        elves += d
    return elves * 10


def presents_of_house2(n: int) -> int:
    elves = 0
    d = 1
    while d * d < n:
        if n % d == 0:
            if n // d <= 50:
                elves += d
            if d <= 50:
                elves += n // d
        d += 1
    if d * d == n:
        if d <= 50:
            elves += d
    return elves * 11


input = 33100000

assert presents_of_house1(1) == 10
assert presents_of_house1(2) == 30
assert presents_of_house1(3) == 40
assert presents_of_house1(4) == 70
assert presents_of_house1(5) == 60
assert presents_of_house1(6) == 120
assert presents_of_house1(7) == 80
assert presents_of_house1(8) == 150
assert presents_of_house1(9) == 130

n = 10
while presents_of_house1(n) < input:
    n += 1
assert n == 776160

n = 10
while presents_of_house2(n) < input:
    n += 1
assert n == 786240
