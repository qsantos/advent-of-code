def main() -> None:
    with open('input') as f:
        seat_ids = {
            int(line.translate(str.maketrans('FBLR', '0101')), 2)
            for line in f
        }
    # puzzle 1
    print(max(seat_ids))
    # puzzle 2
    seat_id, = set(range(min(seat_ids) + 1, max(seat_ids) + 1)) - seat_ids
    print(seat_id)


if __name__ == '__main__':
    main()
