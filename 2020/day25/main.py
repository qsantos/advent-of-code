p = 20201227


def find_secret(pk1: int, pk2: int) -> int:
    sk = 0
    v = 1
    while v != pk1 and v != pk2:
        v = (v * 7) % p
        sk += 1
    if v == pk1:
        return pow(pk2, sk, p)
    else:
        return pow(pk1, sk, p)


assert find_secret(5764801, 17807724) == 14897079  # example
assert find_secret(1327981, 2822615) == 10187657  # input
