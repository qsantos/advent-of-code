def floor(parentheses: str) -> int:
    return parentheses.count('(') - parentheses.count(')')


def first_basement(parentheses: str) -> int:
    floor = 0
    for i, parenthesis in enumerate(parentheses):
        if parenthesis == '(':
            floor += 1
        else:
            floor -= 1
        if floor < 0:
            return i + 1
    assert False


def main() -> None:
    with open('input') as f:
        input = f.read().strip()

    assert floor('(())') == 0
    assert floor('()()') == 0
    assert floor('(((') == 3
    assert floor('(()(()(') == 3
    assert floor('))(((((') == 3
    assert floor('())') == -1
    assert floor('))(') == -1
    assert floor(')))') == -3
    assert floor(')())())') == -3
    assert floor(input) == 280

    assert first_basement(')') == 1
    assert first_basement('()())') == 5
    assert first_basement(input) == 1797


if __name__ == '__main__':
    main()
