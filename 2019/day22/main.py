from typing import List, Tuple

Orders = List[str]
Linear = Tuple[int, int]


def euclidean(a: int, b: int) -> Tuple[int, int, int]:
    if a == 0:
        return (b, 0, 1)
    else:
        g, y, x = euclidean(b % a, a)
        return (g, x - (b // a) * y, y)


def modinv(a: int, m: int) -> int:
    g, x, y = euclidean(a, m)
    if g != 1:
        raise Exception('modular inverse does not exist')
    else:
        return x % m


def read_orders(filename: str) -> Orders:
    with open(filename) as f:
        return f.read().strip().split('\n')


def forward_shuffle(orders: Orders, n_cards: int, target: int) -> int:
    for order in orders:
        if order == 'deal into new stack':
            target = (- target - 1) % n_cards
        elif order.startswith('cut '):
            N = int(order[4:])
            target = (target - N) % n_cards
        elif order.startswith('deal with increment '):
            N = int(order[20:])
            target = (target * N) % n_cards
        else:
            assert False, order
    return target


def backward_shuffle(orders: Orders, n_cards: int, target: int) -> int:
    for order in reversed(orders):
        if order == 'deal into new stack':
            target = (- target - 1) % n_cards
        elif order.startswith('cut '):
            N = int(order[4:])
            target = (target + N) % n_cards
        elif order.startswith('deal with increment '):
            N = int(order[20:])
            target = (target * modinv(N, n_cards)) % n_cards
        else:
            assert False, order
    return target


def full_shuffle(orders: Orders, n_cards: int) -> List[int]:
    return [
        backward_shuffle(orders, n_cards, i)
        for i in range(n_cards)
    ]


def forward_shuffle_polynomial(orders: Orders, n_cards: int) -> Linear:
    a, b = 0, 1
    for order in orders:
        if order == 'deal into new stack':
            a = - a - 1
            b = - b
        elif order.startswith('cut '):
            N = int(order[4:])
            a -= N
        elif order.startswith('deal with increment '):
            N = int(order[20:])
            a = (a * N) % n_cards
            b = (b * N) % n_cards
        else:
            assert False, order
    return a % n_cards, b % n_cards


def backward_shuffle_polynomial(orders: Orders, n_cards: int) -> Linear:
    a, b = 0, 1
    for order in reversed(orders):
        if order == 'deal into new stack':
            a = - a - 1
            b = - b
        elif order.startswith('cut '):
            N = int(order[4:])
            a += N
        elif order.startswith('deal with increment '):
            N = int(order[20:])
            invN = modinv(N, n_cards)
            a = (a * invN) % n_cards
            b = (b * invN) % n_cards
        else:
            assert False, order
    return a % n_cards, b % n_cards


def linear_eval(p: Linear, v: int, m: int) -> int:
    a, b = p
    return (a + b * v) % m


def linear_compose(a: Linear, b: Linear, m: int) -> Linear:
    return linear_eval(a, b[0], m), (a[1] * b[1] % m)


def linear_powmod(p: Linear, n: int, m: int) -> Linear:
    ret = 0, 1
    cur = p
    while n:
        if n % 2:
            ret = linear_compose(cur, ret, m)
        cur = linear_compose(cur, cur, m)
        n //= 2
    return ret


def check_forward_backward(orders: Orders, n_cards: int) -> None:
    fp = forward_shuffle_polynomial(orders, n_cards)
    bp = backward_shuffle_polynomial(orders, n_cards)

    for i in range(n_cards):
        pos = forward_shuffle(orders, n_cards, i)
        assert backward_shuffle(orders, n_cards, pos) == i
        assert linear_eval(fp, i, n_cards) == pos
        assert linear_eval(bp, pos, n_cards) == i


def check_linear_powmod(orders: Orders, n_cards: int) -> None:
    f = forward_shuffle_polynomial(orders, n_cards)
    for i in range(n_cards):
        target = i
        for iteration in range(1, 42):
            target = forward_shuffle(orders, n_cards, target)
            g = linear_powmod(f, iteration, n_cards)
            assert linear_eval(g, i, n_cards) == target, (i, iteration)


def check_example(filename: str, shuffled: List[int]) -> None:
    orders = read_orders(filename)
    assert full_shuffle(orders, 10) == shuffled
    check_forward_backward(orders, 10)
    check_linear_powmod(orders, 10)


def main() -> None:
    check_example('example1', [0, 3, 6, 9, 2, 5, 8, 1, 4, 7])
    check_example('example2', [3, 0, 7, 4, 1, 8, 5, 2, 9, 6])
    check_example('example3', [6, 3, 0, 7, 4, 1, 8, 5, 2, 9])
    check_example('example4', [9, 2, 5, 8, 1, 4, 7, 0, 3, 6])

    challenge = read_orders('input')

    # puzzle 1
    assert forward_shuffle(challenge, 10007, 2019) == 3749

    # puzzle 2
    n_cards = 119315717514047
    bp = backward_shuffle_polynomial(challenge, n_cards)
    power = linear_powmod(bp, 101741582076661, n_cards)
    assert linear_eval(power, 2020, n_cards) == 77225522112241


if __name__ == '__main__':
    main()
