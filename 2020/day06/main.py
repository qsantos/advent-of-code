def main() -> None:
    with open('input') as f:
        data = f.read()

    # puzzle 1
    tot = 0
    for group in data.split('\n\n'):
        tot += len(set.union(*(set(person) for person in group.strip().split('\n'))))
    print(tot)

    # puzzle 2
    tot = 0
    for group in data.split('\n\n'):
        tot += len(set.intersection(*(set(person) for person in group.strip().split('\n'))))
    print(tot)


if __name__ == '__main__':
    main()
