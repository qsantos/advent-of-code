def main() -> None:
    with open('input') as f:
        values = [int(line) for line in f]

    # puzzle 1
    values.sort()
    values.insert(0, 0)
    values.append(values[-1] + 3)
    diffs = [values[i + 1] - values[i] for i in range(len(values) - 1)]
    print(diffs.count(1) * diffs.count(3))

    # puzzle 2
    S = [0] * (values[-1] + 1)
    S[0] = 1
    for value in values:
        for diff in [1, 2, 3]:
            if diff <= value:
                S[value] += S[value - diff]
    print(S[-1])


if __name__ == '__main__':
    main()
